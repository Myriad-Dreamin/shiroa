import { execFileSync } from 'child_process';
import fs from 'fs';

const args = process.argv.slice(2);
let outputJson = false;
let targetArg;

for (const arg of args) {
    if (arg === '--json') {
        outputJson = true;
        continue;
    }

    if (!targetArg) {
        targetArg = arg;
        continue;
    }

    usage(`Unexpected argument: ${arg}`);
}

if (!targetArg) {
    usage('Missing target version');
}

const targetVersion = normalizeVersion(targetArg);
const targetTag = `v${targetVersion}`;
const releaseVersion = stripReleaseCandidateSuffix(targetVersion);
const targetReleaseType = targetVersion === releaseVersion ? 'stable' : 'release-candidate';
const expectedBranch = `bump-version-${targetVersion}`;
const currentBranch = readCurrentBranch();
const currentDate = new Date().toISOString().slice(0, 10);
const changelogPath = changelogPathFor(releaseVersion);
const previousStableTag = findPreviousStableTag(releaseVersion);

const workspaceVersion = readWorkspaceVersion(readText('Cargo.toml'));
const frontendVersion = readPackageJsonVersion('frontend/package.json');
const cliVersion = readCliVersion('cli/src/args.rs');
const lockVersions = readCargoLockPackageVersions(['shiroa', 'shiroa-build']);
const typstPackageVersions = readTypstPackageVersions([
    'packages/shiroa/typst.toml',
    'themes/mdbook/typst.toml',
    'themes/starlight/typst.toml',
]);

const versionFiles = [
    versionFile('Cargo.toml', workspaceVersion, targetVersion, 'workspace.package.version'),
    versionFile('frontend/package.json', frontendVersion, targetVersion, 'frontend package version'),
    versionFile('cli/src/args.rs', cliVersion, targetVersion, 'clap version attribute'),
    ...Object.entries(lockVersions).map(([name, version]) =>
        versionFile('Cargo.lock', version, targetVersion, `Cargo.lock package ${name}`),
    ),
];

const changelog = inspectChangelog(changelogPath, releaseVersion, previousStableTag);
const draftRelease = inspectDraftReleaseScript();
const cargoDependencyCheck = inspectCargoDependencyPins();

const blockers = [
    ...versionFiles
        .filter((item) => item.current !== item.target)
        .map((item) => `${item.label} in ${item.path} is ${item.current}, expected ${item.target}`),
    ...(currentBranch === expectedBranch
        ? []
        : [`current branch is ${currentBranch}, expected ${expectedBranch}`]),
    ...changelog.blockers,
    ...draftRelease.blockers,
    ...cargoDependencyCheck.blockers,
];

const result = {
    targetVersion,
    targetTag,
    targetReleaseType,
    releaseVersion,
    releaseTag: `v${releaseVersion}`,
    expectedBranch,
    currentBranch,
    currentDate,
    previousStableTag,
    versionFiles,
    typstPackageVersions,
    changelog,
    draftRelease,
    cargoDependencyCheck,
    commands: buildCommands(targetVersion, targetTag, expectedBranch, changelogPath),
    readiness: {
        ready: blockers.length === 0,
        blockers,
    },
};

if (outputJson) {
    process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
} else {
    printHuman(result);
}

function usage(message) {
    if (message) {
        console.error(message);
    }

    console.error('Usage: node scripts/release-preflight.mjs <target-version> [--json]');
    process.exit(1);
}

function normalizeVersion(version) {
    const normalized = version.startsWith('v') ? version.slice(1) : version;
    if (!/^\d+\.\d+\.\d+(-rc[1-9]\d*)?$/.test(normalized)) {
        usage(`Invalid target version: ${version}`);
    }

    return normalized;
}

function stripReleaseCandidateSuffix(version) {
    return version.replace(/-rc[1-9]\d*$/, '');
}

function changelogPathFor(version) {
    const [major, minor] = version.split('.');
    return `CHANGELOG/CHANGELOG-${major}.${minor}.md`;
}

function readText(filePath) {
    return fs.readFileSync(filePath, 'utf8');
}

function readCurrentBranch() {
    try {
        return execFileSync('git', ['rev-parse', '--abbrev-ref', 'HEAD'], {
            encoding: 'utf8',
            stdio: ['ignore', 'pipe', 'pipe'],
        }).trim();
    } catch {
        return '(unknown)';
    }
}

function readWorkspaceVersion(cargoToml) {
    const sectionMatch = cargoToml.match(/^\[workspace\.package\]\n([\s\S]*?)(?=^\[[^\]]+\]|\Z)/m);
    if (!sectionMatch) {
        throw new Error('Missing [workspace.package] in Cargo.toml');
    }

    const versionMatch = sectionMatch[1].match(/^version\s*=\s*"([^"]+)"/m);
    if (!versionMatch) {
        throw new Error('Missing workspace.package.version in Cargo.toml');
    }

    return versionMatch[1];
}

function readPackageJsonVersion(filePath) {
    return JSON.parse(readText(filePath)).version;
}

function readCliVersion(filePath) {
    const match = readText(filePath).match(/#\[clap\(name = "shiroa", version = "([^"]+)"\)\]/);
    if (!match) {
        throw new Error(`Missing clap version attribute in ${filePath}`);
    }

    return match[1];
}

function readCargoLockPackageVersions(packageNames) {
    const lock = readText('Cargo.lock');
    const versions = {};

    for (const packageName of packageNames) {
        const regex = new RegExp(
            `\\[\\[package\\]\\]\\nname = "${escapeRegExp(packageName)}"\\nversion = "([^"]+)"`,
        );
        const match = lock.match(regex);
        if (!match) {
            throw new Error(`Missing ${packageName} package in Cargo.lock`);
        }
        versions[packageName] = match[1];
    }

    return versions;
}

function readTypstPackageVersions(paths) {
    return paths.map((filePath) => {
        const match = readText(filePath).match(/^version\s*=\s*"([^"]+)"/m);
        return {
            path: filePath,
            version: match?.[1] ?? null,
        };
    });
}

function versionFile(path, current, target, label) {
    return {
        path,
        label,
        current,
        target,
        ready: current === target,
    };
}

function inspectChangelog(filePath, releaseVersion, previousTag) {
    const blockers = [];
    const expectedHeading = `## v${releaseVersion} - [`;
    const expectedCompare = previousTag
        ? `https://github.com/Myriad-Dreamin/shiroa/compare/${previousTag}...v${releaseVersion}`
        : null;

    if (!fs.existsSync(filePath)) {
        blockers.push(`${filePath} is missing`);
    }

    const readme = readText('CHANGELOG/README.md');
    const readmeLink = `[CHANGELOG-${releaseVersion.split('.').slice(0, 2).join('.')}.md](./${filePath.slice('CHANGELOG/'.length)})`;
    if (!readme.includes(readmeLink)) {
        blockers.push(`CHANGELOG/README.md is missing ${readmeLink}`);
    }

    if (fs.existsSync(filePath)) {
        const text = readText(filePath);
        if (!text.includes(expectedHeading)) {
            blockers.push(`${filePath} is missing ${expectedHeading}...]`);
        }
        if (expectedCompare && !text.includes(expectedCompare)) {
            blockers.push(`${filePath} is missing full changelog link ${expectedCompare}`);
        }
    }

    return {
        path: filePath,
        releaseVersion,
        expectedHeading,
        previousTag,
        expectedCompare,
        ready: blockers.length === 0,
        blockers,
    };
}

function inspectDraftReleaseScript() {
    const text = readText('scripts/draft-release.mjs');
    const blockers = [];

    if (text.includes("const changelogPath = './CHANGELOG/CHANGELOG-0.3.md';")) {
        blockers.push('scripts/draft-release.mjs still hard-codes CHANGELOG-0.3.md');
    }

    return {
        path: 'scripts/draft-release.mjs',
        dynamicChangelogPath: !blockers.length,
        blockers,
    };
}

function inspectCargoDependencyPins() {
    const files = ['Cargo.toml', 'cli/Cargo.toml', 'tools/build-from-source/Cargo.toml'];
    const blockers = [];

    for (const filePath of files) {
        const lines = readText(filePath).split('\n');
        lines.forEach((line, index) => {
            if (/\bbranch\s*=/.test(line) && !line.trimStart().startsWith('#')) {
                blockers.push(`${filePath}:${index + 1} uses a branch dependency`);
            }
        });
    }

    return {
        ready: blockers.length === 0,
        blockers,
    };
}

function findPreviousStableTag(releaseVersion) {
    let tags = [];
    try {
        tags = execFileSync('git', ['tag', '--list', 'v*', '--sort=-version:refname'], {
            encoding: 'utf8',
            stdio: ['ignore', 'pipe', 'pipe'],
        })
            .split('\n')
            .map((line) => line.trim())
            .filter((tag) => /^v\d+\.\d+\.\d+$/.test(tag));
    } catch {
        return null;
    }

    return tags.find((tag) => compareVersions(tag.slice(1), releaseVersion) < 0) ?? null;
}

function compareVersions(left, right) {
    const leftParts = left.split('.').map(Number);
    const rightParts = right.split('.').map(Number);
    const length = Math.max(leftParts.length, rightParts.length);

    for (let index = 0; index < length; index += 1) {
        const leftValue = leftParts[index] ?? 0;
        const rightValue = rightParts[index] ?? 0;
        if (leftValue !== rightValue) {
            return leftValue - rightValue;
        }
    }

    return 0;
}

function buildCommands(version, tag, branch, changelogPath) {
    return {
        inspect: `node scripts/release-preflight.mjs ${tag} --json`,
        review: `git diff -- Cargo.toml frontend/package.json cli/src/args.rs Cargo.lock scripts/draft-release.mjs scripts/release-preflight.mjs CHANGELOG/README.md ${changelogPath}`,
        validate: [
            'cargo fmt --check --all',
            'cargo check --workspace',
            `node scripts/release-preflight.mjs ${tag}`,
        ],
        commit: [
            `git add Cargo.toml frontend/package.json cli/src/args.rs Cargo.lock scripts/draft-release.mjs scripts/release-preflight.mjs CHANGELOG/README.md ${changelogPath}`,
            `git commit -m 'build: bump version to ${version}'`,
        ],
        external: [
            `git push -u origin ${branch}`,
            `gh pr create --repo Myriad-Dreamin/shiroa --title "build: bump version to ${version}" --body "+tag ${tag}"`,
            `git tag ${tag}`,
            `git push origin ${tag}`,
        ],
    };
}

function printHuman(data) {
    console.log(`Release preflight for ${data.targetTag} (${data.targetReleaseType})`);
    console.log(`Branch: ${data.currentBranch} (expected ${data.expectedBranch})`);
    console.log(`Changelog: ${data.changelog.path}`);
    console.log(`Previous stable tag: ${data.previousStableTag ?? '(none found)'}`);
    console.log('');

    console.log('Version files:');
    for (const item of data.versionFiles) {
        const marker = item.ready ? 'OK' : 'MISMATCH';
        console.log(`- [${marker}] ${item.label}: ${item.current} -> ${item.target} (${item.path})`);
    }

    console.log('');
    console.log('Typst package versions (not bumped for release candidates):');
    for (const item of data.typstPackageVersions) {
        console.log(`- ${item.path}: ${item.version ?? '(unknown)'}`);
    }

    console.log('');
    if (data.readiness.ready) {
        console.log('Ready: yes');
        console.log('External actions require approval:');
        for (const command of data.commands.external) {
            console.log(`- ${command}`);
        }
    } else {
        console.log('Ready: no');
        for (const blocker of data.readiness.blockers) {
            console.log(`- ${blocker}`);
        }
    }
}

function escapeRegExp(value) {
    return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

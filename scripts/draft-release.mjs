import { exec } from 'child_process';
import fs from 'fs';

const versionToUpload = process.argv[2];

const changelogPath = './CHANGELOG/CHANGELOG-0.2.md';

const DIST_CMD = "dist";
// const DIST_CMD = "cargo run --manifest-path ../cargo-dist/cargo-dist/Cargo.toml --bin dist --";

const run = (command) => {
    return new Promise((resolve, reject) => {
        exec(command, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            }
            resolve(stdout);
        });
    });
};

const collapsed = (content, summary) => {
    return `<details>

<summary><strong>${summary}</strong></summary>

${content}

</details>`;
}

const main = async () => {
    if (!versionToUpload) {
        console.error("Please provide the version to upload");
        process.exit(1);
    }

    // read version from packages.json
    const packageJson = JSON.parse(
        fs.readFileSync('./frontend/package.json', 'utf8')
    );
    if (packageJson.version !== versionToUpload) {
        console.error(`Version in Cargo.toml (${packageJson.version}) is different from the version to upload (${versionToUpload})`);
        process.exit(1);
    }

    // run dist host command
    // remove target/distrib/dist-manifest.json which causes stateful announce...
    if (fs.existsSync('target/distrib/dist-manifest.json')) {
        fs.unlinkSync('target/distrib/dist-manifest.json');
    }
    
    await run(DIST_CMD + ' generate');

    const distManifest = await run(DIST_CMD + ' host --steps=upload --steps=release --output-format=json');
    const distData = JSON.parse(distManifest);
    const binInstallText = distData.announcement_github_body;
    // write to file
    fs.writeFileSync('target/announcement-dist.md', binInstallText);

    const changelogPlainRaw = await run(`parse-changelog ${changelogPath}`);
    // **Full Changelog**: 
    // Patch the full changelog link
    const fullChangelogLine = /\*\*Full Changelog\*\*: https:\/\/github.com\/Myriad-Dreamin\/shiroa\/compare\/v(\d+\.\d+\.\d+)...v(\d+\.\d+\.\d+)/;
    let anyMatched = false;
    const changelogPlain = changelogPlainRaw.replace(fullChangelogLine, (_match, p1, p2) => {
        anyMatched = true;
        if (!versionToUpload.startsWith(p2)) {
            console.error(`Failed to patch the full changelog link, expected version to upload to start with ${p2}, but got ${versionToUpload}`);
            process.exit(1);
        }

        return `\*\*Full Changelog\*\*: https://github.com/Myriad-Dreamin/shiroa/compare/v${p1}...v${versionToUpload}`;
    });
    if (!anyMatched) {
        console.error("Failed to patch the full changelog link");
        process.exit(1);
    }

    fs.writeFileSync('target/announcement-changelog.md', changelogPlain);

    // concat and generate final announcement
    const binInstallSection = collapsed(binInstallText, `Download Binary`);
    const announcement = [changelogPlain, binInstallSection].join('\n\n');
    fs.writeFileSync('target/announcement.gen.md', announcement);

    console.log("Please check the generated announcement in target/announcement.gen.md");
};

main();

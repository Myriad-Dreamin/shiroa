#import "/github-pages/docs/book.typ": book-page, media

#show: book-page.with(title: "Rendering Tests")

#lorem(10)

= Hello

World.

= Condition in Heading $a N > 0$

This is an $"inline equation" lim_(x arrow 0) sin(x) / x$

This is a block equation:

$ integral.cont f(x) dif x $

Condition ($a N > 0$).
- Condition in list $a N > 0$.

  $ #([$integral.cont f(x) dif x$] * 10) $

#lorem(10)

== Condition in Heading $a N > 0$

#lorem(10)

=== Condition in Heading $a N > 0$

#lorem(10)

==== Condition in Heading $a N > 0$

#lorem(10)

= Consecutive Headings

== Consecutive Headings 2

=== Consecutive Headings x

=== Consecutive Headings x

== Consecutive Headings 3

== Consecutive Headings 4

= Non-Consecutive Headings

#lorem(10)

== Non-Consecutive Headings 2

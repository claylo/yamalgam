# YAML Ain't Markup Language (YAML™) Version 1.1

## Final Draft -- 2005-01-18

### Oren Ben-Kiki

`<[oren@ben-kiki.org](mailto:oren@ben-kiki.org)>`

### Clark Evans

`<[cce@clarkevans.com](mailto:cce@clarkevans.com)>`

### Ingy döt Net

`<[ingy@ingy.net](mailto:ingy@ingy.net)>`

**Status of this Document**

This specification is a draft reflecting consensus reached by members of
 the [yaml-core mailing list](http://lists.sourceforge.net/lists/listinfo/yaml-core). Any questions regarding this draft should be
 raised on this list. We expect all further changes to be strictly
 limited to wording corrections and fixing production bugs.

We wish to thank implementers, who have tirelessly tracked earlier
 versions of this specification, as well as our fabulous user community
 whose feedback has both validated and clarified our direction.

**Abstract**

YAML™ (rhymes with "camel") is a
 human-friendly, cross language, Unicode based data serialization
 language designed around the common native data structures of agile
 programming languages. It is broadly useful for programming needs
 ranging from configuration files to Internet messaging to object
 persistence to data auditing. Together with the [Unicode standard for characters](http://www.unicode.org/),
 this specification provides all the information necessary to understand
 YAML Version 1.1 and to create programs that process YAML information.

---

**Table of Contents**

[1. Introduction](#id838426)
[1.1. Goals](#id838638)
[1.2. Prior Art](#id838686)
[1.3. Relation to XML](#id856927)
[1.4. Terminology](#id856957)

[2. Preview](#id857168)
[2.1. Collections](#id857181)
[2.2. Structures](#id857577)
[2.3. Scalars](#id858081)
[2.4. Tags](#id858600)
[2.5. Full Length Example](#id859040)

[3. Processing YAML Information](#id859109)
[3.1. Processes](#id859458)
[3.1.1. Represent](#id859497)
[3.1.2. Serialize](#id859873)
[3.1.3. Present](#id860109)
[3.1.4. Parse](#id860341)
[3.1.5. Compose](#id860452)
[3.1.6. Construct](#id860557)

[3.2. Information Models](#id860735)
[3.2.1. Representation Graph](#id861060)
[3.2.1.1. Nodes](#id861435)
[3.2.1.2. Tags](#id861700)
[3.2.1.3. Nodes Comparison](#id862121)

[3.2.2. Serialization Tree](#id862929)
[3.2.2.1. Keys Order](#id863110)
[3.2.2.2. Anchors and Aliases](#id863390)

[3.2.3. Presentation Stream](#id863676)
[3.2.3.1. Node Styles](#id863975)
[3.2.3.2. Scalar Formats](#id864510)
[3.2.3.3. Comments](#id864687)
[3.2.3.4. Directives](#id864824)

[3.3. Loading Failure Points](#id864977)
[3.3.1. Well-Formed and Identified](#id865423)
[3.3.2. Resolved](#id865585)
[3.3.3. Recognized and Valid](#id866900)
[3.3.4. Available](#id867229)

[4. Productions Conventions](#id867381)
[4.1. Production Prefixes](#id867562)
[4.2. Production Parameters](#id867808)

[5. Characters](#id868518)
[5.1. Character Set](#id868524)
[5.2. Character Encoding](#id868742)
[5.3. Indicator Characters](#id868988)
[5.4. Line Break Characters](#id871136)
[5.5. Miscellaneous Characters](#id871856)
[5.6. Escape Sequences](#id872840)

[6. Syntax Primitives](#id891745)
[6.1. Indentation Spaces](#id891751)
[6.2. Comments](#id892353)
[6.3. Separation Spaces](#id893014)
[6.4. Ignored Line Prefix](#id893482)
[6.5. Line Folding](#id894136)

[7. YAML Character Stream](#id895107)
[7.1. Directives](#id895217)
[7.1.1. "**`YAML`**" Directive](#id895631)
[7.1.2. "**`TAG`**" Directive](#id896044)
[7.1.2.1. Tag Prefixes](#id896411)
[7.1.2.2. Tag Handles](#id896876)

[7.2. Document Boundary Markers](#id897596)
[7.3. Documents](#id898031)
[7.4. Complete Stream](#id898785)

[8. Nodes](#id899622)
[8.1. Node Anchors](#id899912)
[8.2. Node Tags](#id900262)
[8.3. Node Content](#id901659)
[8.4. Alias Nodes](#id902561)
[8.5. Complete Nodes](#id902919)
[8.5.1. Flow Nodes](#id902924)
[8.5.2. Block Nodes](#id903421)

[9. Scalar Styles](#id903915)
[9.1. Flow Scalar Styles](#id904158)
[9.1.1. Double Quoted](#id904245)
[9.1.2. Single Quoted](#id905860)
[9.1.3. Plain](#id907281)

[9.2. Block Scalar Header](#id926597)
[9.2.1. Block Style Indicator](#id926836)
[9.2.2. Block Indentation Indicator](#id927035)
[9.2.3. Block Chomping Indicator](#id927557)

[9.3. Block Scalar Styles](#id928806)
[9.3.1. Literal](#id928909)
[9.3.2. Folded](#id929764)

[10. Collection Styles](#id930798)
[10.1. Sequence Styles](#id931088)
[10.1.1. Flow Sequences](#id931285)
[10.1.2. Block Sequences](#id931893)

[10.2. Mapping Styles](#id932806)
[10.2.1. Flow Mappings](#id933010)
[10.2.2. Block Mappings](#id934537)

[Index](#id935693)

## Chapter 1. Introduction

"YAML Ain't Markup Language" (abbreviated YAML) is a data
 serialization language designed to be human-friendly and work well with
 modern programming languages for common everyday tasks. This
 specification is both an introduction to the YAML language and the
 concepts supporting it; it is also a complete reference of the
 information needed to develop [applications](#application/) for processing YAML.

Open, interoperable and readily understandable tools have advanced
 computing immensely. YAML was designed from the start to be useful and
 friendly to people working with data. It uses Unicode [printable](<#printable character/>) characters, some of
 which provide structural information and the rest containing the data
 itself. YAML achieves a unique cleanness by minimizing the amount of
 structural characters and allowing the data to show itself in a natural
 and meaningful way. For example, [indentation](<#indentation space/>) may be used for structure, colons separate
 "[mapping key:](<#key/information model>) [value](<#value/information model>)" pairs, and dashes are used to create
 "bullet" lists.

There are myriad flavors of data structures, but they can all be
 adequately [represented](#represent/) with three
 basic primitives: [mappings](<#mapping/information model>) (hashes/dictionaries), [sequences](<#sequence/information model>) (arrays/lists) and [scalars](<#scalar/information model>) (strings/numbers). YAML leverages these
 primitives and adds a simple typing system and [aliasing](<#alias/information model>) mechanism to form a
 complete language for serializing any data structure. While most
 programming languages can use YAML for data serialization, YAML excels in
 working with those languages that are fundamentally built around the
 three basic primitives. These include the new wave of agile languages
 such as Perl, Python, PHP, Ruby, and Javascript.

There are hundreds of different languages for programming, but only a
 handful of languages for storing and transferring data. Even though its
 potential is virtually boundless, YAML was specifically created to work
 well for common use cases such as: configuration files, log files,
 interprocess messaging, cross-language data sharing, object persistence,
 and debugging of complex data structures. When data is easy to view and
 understand, programming becomes a simpler task.

## 1.1. Goals

The design goals for YAML are:

1. YAML is easily readable by humans.
2. YAML matches the native data structures of agile languages.
3. YAML data is portable between programming languages.
4. YAML has a consistent model to support generic tools.
5. YAML supports one-pass processing.
6. YAML is expressive and extensible.
7. YAML is easy to implement and use.

## 1.2. Prior Art

YAML's initial direction was set by the data serialization and
 markup language discussions among [SML-DEV members](http://www.docuverse.com/smldev/). Later
 on, it directly incorporated experience from Brian Ingerson's Perl
 module [Data::Denter](http://search.cpan.org/doc/INGY/Data-Denter-0.13/Denter.pod). Since then, YAML has matured through ideas and
 support from its user community.

YAML integrates and builds upon concepts described by [C](http://cm.bell-labs.com/cm/cs/cbook/index.html), [Java](http://java.sun.com/), [Perl](http://www.perl.org/), [Python](http://www.python.org/), [Ruby](http://www.ruby-lang.org/), [RFC0822](http://www.ietf.org/rfc/rfc0822.txt) (MAIL), [RFC1866](http://www.ics.uci.edu/pub/ietf/html/rfc1866.txt) (HTML), [RFC2045](http://www.ietf.org/rfc/rfc2045.txt) (MIME), [RFC2396](http://www.ietf.org/rfc/rfc2396.txt) (URI), [XML](http://www.w3.org/TR/REC-xml.html), [SAX](http://www.saxproject.org/) and [SOAP](http://www.w3.org/TR/SOAP).

The syntax of YAML was motivated by Internet Mail (RFC0822) and remains
 partially compatible with that standard. Further, borrowing from MIME
 (RFC2045), YAML's top-level production is a [stream](<#stream/information model>) of
 independent [documents](<#document/information model>); ideal for message-based distributed
 processing systems.

YAML's [indentation](<#indentation space/>)-based scoping is similar to Python's
 (without the ambiguities caused by [tabs](#tab/)). [Indented blocks](<#block style/information model>) facilitate easy
 inspection of the data's structure. YAML's [literal style](<#literal style/information model>) leverages this by enabling formatted text to be cleanly
 mixed within an [indented](<#indentation space/>) structure without troublesome [escaping](<#escaping in double-quoted style/>). YAML also allows the use of
 traditional [indicator](#indicator/)-based
 scoping similar to Perl's. Such [flow content](<#flow style/information model>) can be freely
 nested inside [indented blocks](<#block style/information model>).

YAML's [double-quoted style](<#double-quoted style/information model>) uses familiar C-style [escape sequences](<#escaping in double-quoted style/>).
 This enables ASCII encoding of non-[printable](<#printable character/>) or 8-bit (ISO 8859-1) characters such as ["**`\x3B`**"](#ns-esc-8-bit). Non-[printable](<#printable character/>) 16-bit Unicode and
 32-bit (ISO/IEC 10646) characters are supported with [escape sequences](<#escaping in double-quoted style/>) such as ["**`\u003B`**"](#ns-esc-16-bit) and ["**`\U0000003B`**"](#ns-esc-32-bit).

Motivated by HTML's end-of-line normalization, YAML's [line folding](<#line folding/>) employs an intuitive
 method of handling [line breaks](<#line break character/>). A single [line break](<#line break character/>) is [folded](<#line folding/>) into a single space, while [empty lines](<#empty line/>) are interpreted as [line break](<#line break character/>) characters. This technique allows for
 paragraphs to be word-wrapped without affecting the [canonical form](<#canonical form/>) of the [content](<#content/information model>).

YAML's core type system is based on the requirements of agile
 languages such as Perl, Python, and Ruby. YAML directly supports both [collection](<#collection/information model>) ([mapping](<#mapping/information model>), [sequence](<#sequence/information model>))
 and [scalar content](<#scalar/information model>). Support for common types enables programmers to use
 their language's native data structures for YAML manipulation,
 instead of requiring a special document object model (DOM).

Like XML's SOAP, YAML supports [serializing](#serialize/) native graph data structures
 through an [aliasing](<#alias/information model>) mechanism. Also like SOAP, YAML provides for [application](#application/)-defined [types](<#tag/information model>). This
 allows YAML to [represent](#represent/) rich
 data structures required for modern distributed computing. YAML
 provides globally unique [type names](<#global tag/>) using a namespace mechanism inspired by Java's
 DNS-based package naming convention and XML's URI-based namespaces.

YAML was designed to support incremental interfaces that include both
 input ("**`getNextEvent()`**") and output
 "**`sendNextEvent()`**") one-pass interfaces. Together, these
 enable YAML to support the processing of large [documents](<#document/information model>) (e.g. transaction logs) or continuous [streams](<#stream/information model>) (e.g. feeds from a
 production machine).

## 1.3. Relation to XML

Newcomers to YAML often search for its correlation to the eXtensible
 Markup Language (XML). Although the two languages may actually compete
 in several application domains, there is no direct correlation between
 them.

YAML is primarily a data serialization language. XML was designed to be
 backwards compatible with the Standard Generalized Markup Language
 (SGML) and thus had many design constraints placed on it that YAML does
 not share. Inheriting SGML's legacy, XML is designed to support
 structured documentation, where YAML is more closely targeted at data
 structures and messaging. Where XML is a pioneer in many domains, YAML
 is the result of lessons learned from XML and other technologies.

It should be mentioned that there are ongoing efforts to define
 standard XML/YAML mappings. This generally requires that a subset of
 each language be used. For more information on using both XML and YAML,
 please visit [https://yaml.org/xml/index.html](/xml/index.html).

## 1.4. Terminology

This specification uses key words based on [RFC2119](http://www.ietf.org/rfc/rfc2119.txt) to indicate
 requirement level. In particular, the following words are used to
 describe the actions of a YAML [processor](#processor/):

May
The word *may*, or the adjective *optional*, mean that
 conforming YAML [processors](#processor/) are permitted, but *need not* behave as described.

Should
The word *should*, or the
 adjective *recommended*,
 mean that there could be reasons for a YAML [processor](#processor/) to deviate from the
 behavior described, but that such deviation could hurt
 interoperability and should therefore be advertised with
 appropriate notice.

Must
The word *must*, or the term *required* or *shall*, mean that the behavior described
 is an absolute requirement of the specification.

## Chapter 2. Preview

This section provides a quick glimpse into the expressive power of YAML.
 It is not expected that the first-time reader grok all of the examples.
 Rather, these selections are used as motivation for the remainder of the
 specification.

## 2.1. Collections

YAML's [block collections](<#block collection style/information model>) use [indentation](<#indentation space/>) for scope
 and begin each entry on its own line. [Block sequences](<#block sequence style/information model>) indicate each entry with a dash and space ( ["**`-`**"](<#- block sequence entry/>)). [Mappings](<#mapping/information model>) use a colon and
 space (["**`: `**"](<#: mapping value/>)) to mark each [mapping key](<#key/information model>): [value](<#value/information model>) pair.

- **Example 2.1.
 Sequence of Scalars
(ball players)** ```
- Mark McGwire
- Sammy Sosa
- Ken Griffey

``` **Example 2.2.
 Mapping Scalars to Scalars
(player statistics)** ```
hr: 65 # Home runs
avg: 0.278 # Batting average
rbi: 147 # Runs Batted In

```
- **Example 2.3.
 Mapping Scalars to Sequences
(ball clubs in each league)** ```
american:
 - Boston Red Sox
 - Detroit Tigers
 - New York Yankees
national:
 - New York Mets
 - Chicago Cubs
 - Atlanta Braves

``` **Example 2.4.
 Sequence of Mappings
(players' statistics)** ```
-
 name: Mark McGwire
 hr: 65
 avg: 0.278
-
 name: Sammy Sosa
 hr: 63
 avg: 0.288

```
YAML also has [flow styles](<#flow style/information model>), using explicit [indicators](#indicator/) rather than [indentation](<#indentation space/>) to denote scope.
 The [flow sequence](<#flow sequence style/information model>) is written as a comma separated list
 within square brackets. In a similar manner, the [flow mapping](<#flow mapping style/information model>) uses curly braces.

| **Example 2.5. Sequence of Sequences**  ``` - [name        , hr, avg  ]   - [Mark McGwire, 65, 0.278] - [Sammy Sosa  , 63, 0.288]    ``` | **Example 2.6. Mapping of Mappings**  ``` Mark McGwire: {hr: 65, avg: 0.278}   Sammy Sosa: {     hr: 63,     avg: 0.288   }  ``` |
| --- | --- |

## 2.2. Structures

YAML uses three dashes (["**`---`**"](<#document boundary marker/>)) to separate [documents](<#document/information model>) within a [stream](<#stream/information model>). Three dots ( ["**`...`**"](<#document boundary marker/>)) indicate the end of
 a document without starting a new one, for use in communication
 channels. [Comment](<#comment/information model>) lines begin with the Octothorpe (also called
 "hash", "sharp", or "number
 sign" - ["**`#`**"](<## comment/>)).

| **Example 2.7.   Two Documents in a Stream     (each with a leading comment)**   ``` # Ranking of 1998 home runs   --- - Mark McGwire - Sammy Sosa - Ken Griffey  # Team ranking --- - Chicago Cubs - St Louis Cardinals  ``` | **Example 2.8.   Play by Play Feed     from a Game**   ``` ---   time: 20:03:20 player: Sammy Sosa action: strike (miss) ... --- time: 20:03:47 player: Sammy Sosa action: grand slam ...  ``` |
| --- | --- |


Repeated [nodes](<#node/information model>) are first [identified](#identified/) by an [anchor](<#anchor/information model>) (marked with the ampersand - ["**`&`**"](<#& anchor/>)), and are then [aliased](<#alias/information model>) (referenced with an asterisk - ["**`*`**"](<#* alias/>)) thereafter.

| **Example 2.9.   Single Document with     Two Comments**   ``` ---   hr: # 1998 hr ranking   - Mark McGwire   - Sammy Sosa rbi:   # 1998 rbi ranking   - Sammy Sosa   - Ken Griffey  ``` | **Example 2.10.   Node for "`Sammy Sosa`"     appears twice in this document**   ``` ---   hr:   - Mark McGwire   # Following node labeled SS   - &SS Sammy Sosa rbi:   - *SS # Subsequent occurrence   - Ken Griffey  ``` |
| --- | --- |


A question mark and space [("**`? `**"](<#? mapping key/>)) indicate a complex [mapping key](<#key/information model>).
 Within a [block collection](<#block collection style/information model>), [key:](<#key/information model>) [value](<#value/information model>) pairs can start
 immediately following the dash, colon, or question mark.

| **Example 2.11. Mapping between Sequences**  ``` ? - Detroit Tigers     - Chicago cubs :   - 2001-07-23  ? [ New York Yankees,     Atlanta Braves ] : [ 2001-07-02, 2001-08-12,     2001-08-14 ]  ``` | **Example 2.12. In-Line Nested Mapping**  ``` ---   # products purchased - item    : Super Hoop   quantity: 1 - item    : Basketball   quantity: 4 - item    : Big Shoes   quantity: 1   ``` |
| --- | --- |

## 2.3. Scalars

[Scalar content](<#scalar/information model>) can be written in [block](<#block style/information model>) form, using a [literal style](<#literal style/information model>) (["**`|`**"](<#| literal style/>)) where all [line breaks](<#line break character/>) are significant.
 Alternatively, they can be written with the [folded style](<#folded style/information model>) [("**`>`**"](<#> folded style/>)) where each [line break](<#line break character/>) is [folded](<#line folding/>) to a space unless it ends an [empty](<#empty line/>) or a ["more indented" line](<#more indented line/>).

- **Example 2.13.
 In literals,
newlines are preserved** ```
# ASCII Art
--- |
 \//||\/||
 // || ||__

``` **Example 2.14.
 In the plain scalar,
newlines become spaces** ```
---
 Mark McGwire's
 year was crippled
 by a knee injury.

```
- **Example 2.15.
 Folded newlines are preserved
for "more indented" and blank lines** ```
>
 Sammy Sosa completed another
 fine season with great stats.

 63 Home Runs
 0.288 Batting Average

 What a year!

``` **Example 2.16.
 Indentation determines scope**
```
name: Mark McGwire
accomplishment: >
 Mark set a major league
 home run record in 1998.
stats: |
 65 Home Runs
 0.278 Batting Average


```
YAML's [flow scalars](<#flow scalar style/information model>) include the [plain style](<#plain style/information model>) (most
 examples thus far) and [quoted styles](<#quoted style/information model>). The [double-quoted style](<#double-quoted style/information model>) provides [escape sequences](<#escaping in double-quoted style/>).
 The [single-quoted style](<#single-quoted style/information model>) is useful when [escaping](<#escaping in double-quoted style/>) is not
 needed. All [flow scalars](<#flow scalar style/information model>) can span
 multiple lines; [line breaks](<#line break character/>) are always [folded](<#line folding/>).

| **Example 2.17. Quoted Scalars**  ``` unicode: "Sosa did fine.\u263A"   control: "\b1998\t1999\t2000\n" hexesc:  "\x13\x10 is \r\n"  single: '"Howdy!" he cried.' quoted: ' # not a ''comment''.' tie-fighter: '|\-*-/|'  ``` | **Example 2.18. Multi-line Flow Scalars**  ``` plain:     This unquoted scalar   spans many lines.  quoted: "So does this   quoted scalar.\n"   ``` |
| --- | --- |

## 2.4. Tags

In YAML, [untagged nodes](<#non-specific tag/>) are given an type depending on the [application](#application/). The examples in this
 specification generally use the ["**`seq`**"](/type/seq.html), ["**`map`**"](/type/map.html) and ["**`str`**"](/type/str.html) types from the [YAML tag repository](/type/index.html). A few examples also use the ["**`int`**"](/type/int.html) and ["**`float`**"](/type/float.html) types. The repository includes additional types such as ["**`null`**"](/type/null.html), ["**`bool`**"](/type/bool.html), ["**`set`**"](/type/set.html) and
 others.

- **Example 2.19. Integers**```
canonical: 12345
decimal: +12,345
sexagesimal: 3:25:45
octal: 014
hexadecimal: 0xC


``` **Example 2.20. Floating Point**```
canonical: 1.23015e+3
exponential: 12.3015e+02
sexagesimal: 20:30.15
fixed: 1,230.15
negative infinity: -.inf
not a number: .NaN

```
- **Example 2.21. Miscellaneous**```
null: ~
true: y
false: n
string: '12345'

``` **Example 2.22. Timestamps**```
canonical: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14

```
Explicit typing is denoted with a [tag](<#tag/information model>) using the exclamation
 point (["**`!`**"](<#! tag indicator/>)) symbol. [Global tags](<#global tag/>) are URIs and may be
 specified in a [shorthand](<#tag shorthand/>) form using a [handle](<#tag handle/>). [Application](#application/)-specific [local tags](<#local tag/>) may also be used.

| **Example 2.23. Various Explicit Tags**  ``` ---   not-date: !!str 2002-04-28  picture: !!binary |  R0lGODlhDAAMAIQAAP//9/X  17unp5WZmZgAAAOfn515eXv  Pz7Y6OjuDg4J+fn5OTk6enp  56enmleECcgggoBADs=  application specific tag: !something |  The semantics of the tag  above may be different for  different documents.   ``` | **Example 2.24. Global Tags**  ``` %TAG ! tag:clarkevans.com,2002:   --- !shape   # Use the ! handle for presenting   # tag:clarkevans.com,2002:circle - !circle   center: &ORIGIN {x: 73, y: 129}   radius: 7 - !line   start: *ORIGIN   finish: { x: 89, y: 102 } - !label   start: *ORIGIN   color: 0xFFEEBB   text: Pretty vector drawing.  ``` |
| --- | --- |

| **Example 2.25. Unordered Sets**  ``` # sets are represented as a   # mapping where each key is # associated with the empty string --- !!set ? Mark McGwire ? Sammy Sosa ? Ken Griff  ``` | **Example 2.26. Ordered Mappings**  ``` # ordered maps are represented as   # a sequence of mappings, with # each mapping having one key --- !!omap - Mark McGwire: 65 - Sammy Sosa: 63 - Ken Griffy: 58  ``` |
| --- | --- |

## 2.5. Full Length Example

Below are two full-length examples of YAML. On the left is a sample
 invoice; on the right is a sample log file.

| **Example 2.27. Invoice**  ``` --- !<tag:clarkevans.com,2002:invoice>   invoice: 34843 date   : 2001-01-23 bill-to: &id001     given  : Chris     family : Dumars     address:         lines: |             458 Walkman Dr.             Suite #292         city    : Royal Oak         state   : MI         postal  : 48046 ship-to: *id001 product:     - sku         : BL394D       quantity    : 4       description : Basketball       price       : 450.00     - sku         : BL4438H       quantity    : 1       description : Super Hoop       price       : 2392.00 tax  : 251.42 total: 4443.52 comments:     Late afternoon is best.     Backup contact is Nancy     Billsmer @ 338-4338.  ``` | **Example 2.28. Log File**  ``` ---   Time: 2001-11-23 15:01:42 -5 User: ed Warning:   This is an error message   for the log file --- Time: 2001-11-23 15:02:31 -5 User: ed Warning:   A slightly different error   message. --- Date: 2001-11-23 15:03:17 -5 User: ed Fatal:   Unknown variable "bar" Stack:   - file: TopClass.py     line: 23     code: |       x = MoreObject("345\n")   - file: MoreClass.py     line: 58     code: |-       foo = bar     ``` |
| --- | --- |

## Chapter 3. Processing YAML Information

YAML is both a text format and a method for [presenting](#present/) any data structure in this format.
 Therefore, this specification defines two concepts: a class of data
 objects called YAML [representations](#representation/), and a syntax for [presenting](#present/) YAML [representations](#representation/) as a series of
 characters, called a YAML [stream](<#stream/information model>). A YAML *processor* is a tool for converting
 information between these complementary views. It is assumed that a YAML
 processor does its work on behalf of another module, called an *application*. This chapter describes the
 information structures a YAML processor must provide to or obtain from
 the application.

YAML information is used in two ways: for machine processing, and for
 human consumption. The challenge of reconciling these two perspectives is
 best done in three distinct translation stages: [representation](#representation/), [serialization](#serialization/), and [presentation](#presentation/). [Representation](#representation/) addresses how YAML
 views native data structures to achieve portability between programming
 environments. [Serialization](#serialization/) concerns itself with turning a YAML [representation](#representation/) into a serial form,
 that is, a form with sequential access constraints. [Presentation](#presentation/) deals with the formatting
 of a YAML [serialization](#serialization/) as a
 series of characters in a human-friendly manner.

**Figure 3.1. Processing Overview**

![Processing Overview](overview2.png)

A YAML processor need not expose the [serialization](#serialization/) or [representation](#representation/) stages. It may
 translate directly between native data structures and a character [stream](<#stream/information model>) (*dump* and *load* in the diagram above). However, such a
 direct translation should take place so that the native data structures
 are [constructed](#construct/) only from
 information available in the [representation](#representation/).

## 3.1. Processes

This section details the processes shown in the diagram above. Note a
 YAML [processor](#processor/) need not provide
 all these processes. For example, a YAML library may provide only YAML
 input ability, for loading configuration files, or only output ability,
 for sending data to other [applications](#application/).

### 3.1.1. Represent

YAML *represents* any native
 data structure using three [node kinds](#kind/): [sequence](<#sequence/information model>) - an ordered series of entries; [mapping](<#mapping/information model>) -
 an unordered association of [unique](#equality/) [keys](<#key/information model>) to [values](<#value/information model>); and [scalar](<#scalar/information model>) - any datum with opaque structure [presentable](#present/) as a series of Unicode
 characters. Combined, these primitives generate directed graph
 structures. These primitives were chosen because they are both
 powerful and familiar: the [sequence](<#sequence/information model>) corresponds to a
 Perl array and a Python list, the [mapping](<#mapping/information model>) corresponds to a Perl
 hash table and a Python dictionary. The [scalar](<#scalar/information model>) represents strings,
 integers, dates, and other atomic data types.

Each YAML [node](<#node/information model>) requires, in addition to its [kind](#kind/) and [content](<#content/information model>), a [tag](<#tag/information model>) specifying
 its data type. Type specifiers are either [global](<#global tag/>) URIs, or are [local](<#local tag/>) in scope to a single [application](#application/). For example, an integer
 is represented in YAML with a [scalar](<#scalar/information model>) plus the [global tag](<#global tag/>) "**`tag:yaml.org,2002:int`**". Similarly, an invoice object,
 particular to a given organization, could be represented as a [mapping](<#mapping/information model>) together with the [local tag](<#local tag/>) "**`!invoice`**". This simple model
 can represent any data structure independent of programming language.

### 3.1.2. Serialize

For sequential access mediums, such as an event callback API, a YAML [representation](#representation/) must be *serialized* to an ordered tree.
 Since in a YAML [representation](#representation/), [mapping keys](<#key/information model>) are unordered and [nodes](<#node/information model>) may be referenced more than once (have more
 than one incoming "arrow"), the serialization process is
 required to impose an [ordering](<#key order/>) on the [mapping keys](<#key/information model>) and to replace the second and subsequent references to
 a given [node](<#node/information model>) with place holders called [aliases](<#alias/information model>). YAML
 does not specify how these *serialization details* are chosen. It is up to the
 YAML [processor](#processor/) to come up with
 human-friendly [key order](<#key order/>) and [anchor](<#anchor/information model>) names, possibly with the help of the [application](#application/). The result of this
 process, a YAML [serialization tree](#serialization/), can then be traversed to produce a series of event
 calls for one-pass processing of YAML data.

### 3.1.3. Present

The final output process is *presenting* the YAML [serializations](#serialization/) as a character [stream](<#stream/information model>) in a human-friendly manner. To maximize human
 readability, YAML offers a rich set of stylistic options which go
 far beyond the minimal functional needs of simple data storage.
 Therefore the YAML [processor](#processor/) is required to introduce various *presentation details* when creating the [stream](<#stream/information model>), such
 as the choice of [node styles](#style/), how
 to [format content](#format/), the amount of [indentation](<#indentation space/>), which [tag handles](<#tag handle/>) to use, the [node tags](<#tag/information model>) to leave [unspecified](<#non-specific tag/>), the set of [directives](<#directive/information model>) to provide and
 possibly even what [comments](<#comment/information model>) to add. While some of this can be done with
 the help of the [application](#application/), in general this process
 should be guided by the preferences of the user.

### 3.1.4. Parse

*Parsing* is the inverse process of [presentation](#present/), it takes a [stream](<#stream/information model>) of characters and produces a series of
 events. Parsing discards all the [details](<#presentation detail/>) introduced in the [presentation](#present/) process, reporting only the [serialization](#serialization/) events.
 Parsing can fail due to [ill-formed](<#ill-formed stream/>) input.

### 3.1.5. Compose

*Composing* takes a series of [serialization](#serialization/) events and
 produces a [representation graph](#representation/). Composing discards all the [serialization details](<#serialization detail/>) introduced in the [serialization](#serialize/) process, producing only
 the [representation graph](#representation/).
 Composing can fail due to any of several reasons, detailed [below](<#load failure point/>).

### 3.1.6. Construct

The final input process is *constructing* native data structures
 from the YAML [representation](#representation/). Construction must
 be based only on the information available in the [representation](#representation/), and not on
 additional [serialization](#serialization/) or [presentation details](<#presentation detail/>) such as [comments](<#comment/information model>), [directives](<#directive/information model>), [mapping key order](<#key order/>), [node styles](#style/), [content format](#format/), [indentation](<#indentation space/>) levels etc.
 Construction can fail due to the [unavailability](<#unavailable tag/>) of the required native data types.

## 3.2. Information Models

This section specifies the formal details of the results of the above
 processes. To maximize data portability between programming languages
 and implementations, users of YAML should be mindful of the distinction
 between [serialization](#serialization/) or [presentation](#presentation/) properties and
 those which are part of the YAML [representation](#representation/). Thus, while imposing
 a [order](<#key order/>) on [mapping keys](<#key/information model>) is
 necessary for flattening YAML [representations](#representation/) to a sequential
 access medium, this [serialization detail](<#serialization detail/>) must not be used to convey [application](#application/) level information.
 In a similar manner, while [indentation](<#indentation space/>) technique and a choice of a [node style](#style/) are needed for the human
 readability, these [presentation details](<#presentation detail/>) are neither part of the YAML [serialization](#serialization/) nor the YAML [representation](#representation/). By carefully
 separating properties needed for [serialization](#serialization/) and [presentation](#presentation/), YAML [representations](#representation/) of [application](#application/) information will be
 consistent and portable between various programming environments.

The following diagram summarizes the three information models. Full
 arrows denote composition, hollow arrows denote inheritance,
 "**`1`**" and "**`*`**" denote "one" and
 "many" relationships. A single "**`+`**" denotes [serialization](#serialization/) details, a
 double "**`++`**" denotes [presentation](#presentation/) details.

**Figure 3.2. Information Models**

![Information Models](model2.png)

### 3.2.1. Representation Graph

YAML's *representation* of native data is a rooted, connected, directed graph of [tagged](<#tag/information model>) [nodes](<#node/information model>). By
 "directed graph" we mean a set of [nodes](<#node/information model>) and
 directed edges ("arrows"), where each edge connects one [node](<#node/information model>) to another (see [a formal definition](http://www.nist.gov/dads/HTML/directedGraph.html)). All the [nodes](<#node/information model>) must be reachable from
 the *root node* via such edges.
 Note that the YAML graph may include cycles, and a [node](<#node/information model>) may have
 more than one incoming edge.

[Nodes](<#node/information model>) that are defined in terms of other [nodes](<#node/information model>) are [collections](<#collection/information model>) and [nodes](<#node/information model>) that are independent of
 any other [nodes](<#node/information model>) are [scalars](<#scalar/information model>). YAML supports two [kinds](#kind/) of [collection nodes](<#collection/information model>), [sequences](<#sequence/information model>) and [mappings](<#mapping/information model>). [Mapping nodes](<#mapping/information model>) are somewhat tricky because their [keys](<#key/information model>) are
 unordered and must be [unique](#equality/).

**Figure 3.3. Representation Model**

![Representation Model](represent2.png)

#### 3.2.1.1. Nodes

YAML *nodes* have *content* of one of three *kinds*: scalar, sequence, or
 mapping. In addition, each node has a [tag](<#tag/information model>) which serves to
 restrict the set of possible values which the node's content can
 have.

Scalar
The content of a *scalar* node is an
 opaque datum that can be [presented](#present/) as a series of zero or
 more Unicode characters.

Sequence
The content of a *sequence* node is an
 ordered series of zero or more nodes. In particular, a sequence
 may contain the same node more than once or it could even
 contain itself (directly or indirectly).

Mapping
The content of a *mapping* node is an
 unordered set of *key:* *value* node pairs, with
 the restriction that each of the keys is [unique](#equality/). YAML places no further
 restrictions on the nodes. In particular, keys may be arbitrary
 nodes, the same node may be used as the value of several
 key: value pairs, and a mapping could even contain itself
 as a key or a value (directly or indirectly).

When appropriate, it is convenient to consider sequences and
 mappings together, as *collections*. In this view,
 sequences are treated as mappings with integer keys starting at
 zero. Having a unified collections view for sequences and mappings
 is helpful both for creating practical YAML tools and APIs and for
 theoretical analysis.

#### 3.2.1.2. Tags

YAML [represents](#represent/) type
 information of native data structures with a simple identifier,
 called a *tag*. *Global
 tags* are [URIs](http://www.ietf.org/rfc/rfc2396.txt) and hence
 globally unique across all [applications](#application/). The
 "**`tag`**": [URI scheme](http://www.taguri.org) ([mirror](/spec/taguri.txt)) is
 recommended for all global YAML tags. In contrast, *local tags* are specific to a single [application](#application/). Local tags
 start with *"**`!`**"*, are not URIs and are not
 expected to be globally unique. YAML provides a ["**`TAG`**" directive](<#TAG directive/>) to
 make tag notation less verbose; it also offers easy migration from
 local to global tags. To ensure this, local tags are restricted to
 the URI character set and use URI character [escaping](<#escaping in URI/>).

YAML does not mandate any special relationship between different
 tags that begin with the same substring. Tags ending with URI
 fragments (containing ["**`#`**"](<## comment/>)) are no exception; tags that
 share the same base URI but differ in their fragment part are
 considered to be different, independent tags. By convention,
 fragments are used to identify different "variants" of
 a tag, while "**`/`**" is used to define nested tag
 "namespace" hierarchies. However, this is merely a
 convention, and each tag may employ its own rules. For example,
 Perl tags may use "**`::`**" to express namespace
 hierarchies, Java tags may use "**`.`**", etc.

YAML tags are used to associate meta information with each [node](<#node/information model>). In
 particular, each tag must specify the expected [node kind](#kind/) ([scalar](<#scalar/information model>), [sequence](<#sequence/information model>), or [mapping](<#mapping/information model>)). [Scalar](<#scalar/information model>) tags must also provide mechanism for converting [formatted content](#format/) to a [canonical form](<#canonical form/>) for supporting [equality](#equality/) testing.
 Furthermore, a tag may provide additional information such as the
 set of allowed [content values](<#content/information model>) for validation, a mechanism for [tag resolution](<#tag resolution/>), or any
 other data that is applicable to all of the tag's [nodes](<#node/information model>).

#### 3.2.1.3. Nodes Comparison

Since YAML [mappings](<#mapping/information model>) require [key](<#key/information model>) uniqueness, [representations](#representation/) must include a
 mechanism for testing the equality of [nodes](<#node/information model>). This is non-trivial
 since YAML allows various ways to [format](#format/) a given [scalar content](<#scalar/information model>). For
 example, the integer eleven can be written as "**`013`**"
 (octal) or "**`0xB`**" (hexadecimal). If both forms are
 used as [keys](<#key/information model>) in the same [mapping](<#mapping/information model>), only a YAML [processor](#processor/) which recognizes
 integer [formats](#format/) would correctly
 flag the duplicate [key](<#key/information model>) as an error.

Canonical Form
YAML supports the need for [scalar](<#scalar/information model>) equality by
 requiring that every [scalar](<#scalar/information model>)[tag](<#tag/information model>) must
 specify a mechanism to producing the *canonical form* of any [formatted content](#format/). This
 form is a Unicode character string which [presents](#present/) the [content](<#content/information model>) and can be used for equality testing.
 While this requirement is stronger than a well defined equality
 operator, it has other uses, such as the production of digital
 signatures.

Equality
Two [nodes](<#node/information model>) must have the same [tag](<#tag/information model>) and [content](<#content/information model>) to be *equal*. Since each [tag](<#tag/information model>) applies to exactly one [kind](#kind/),
 this implies that the two [nodes](<#node/information model>) must have the
 same [kind](#kind/) to be equal. Two [scalars](<#scalar/information model>) are equal only when their [tags](<#tag/information model>) and
 canonical forms are equal character-by-character. Equality of [collections](<#collection/information model>) is defined recursively. Two [sequences](<#sequence/information model>) are equal only when they have the
 same [tag](<#tag/information model>) and length, and each [node](<#node/information model>) in
 one [sequence](<#sequence/information model>) is equal to the corresponding [node](<#node/information model>) in the other [sequence](<#sequence/information model>). Two [mappings](<#mapping/information model>) are equal only when they have the
 same [tag](<#tag/information model>) and an equal set of [keys](<#key/information model>),
 and each [key](<#key/information model>) in this set is associated with equal [values](<#value/information model>) in both [mappings](<#mapping/information model>).

Identity
Two [nodes](<#node/information model>) are *identical* only when they [represent](#represent/) the same native data
 structure. Typically, this corresponds to a single memory
 address. Identity should not be confused with equality; two
 equal [nodes](<#node/information model>) need not have the same identity. A YAML [processor](#processor/) may treat
 equal [scalars](<#scalar/information model>) as if they were identical. In
 contrast, the separate identity of two distinct but equal [collections](<#collection/information model>) must be preserved.

### 3.2.2. Serialization Tree

To express a YAML [representation](#representation/) using a serial API,
 it necessary to impose an [order](<#key order/>) on [mapping keys](<#key/information model>) and employ [alias nodes](<#alias/information model>) to indicate a subsequent occurrence of a previously
 encountered [node](<#node/information model>). The result of this process is a *serialization tree*, where each [node](<#node/information model>) has an ordered set of children. This tree can be traversed for a
 serial event-based API. [Construction](#construct/) of native structures from
 the serial interface should not use [key order](<#key order/>) or [anchors](<#anchor/information model>) for the preservation of important data.

**Figure 3.4. Serialization Model**

![Serialization Model](serialize2.png)

#### 3.2.2.1. Keys Order

In the [representation](#representation/) model, [mapping keys](<#key/information model>) do not have an order. To [serialize](#serialize/) a [mapping](<#mapping/information model>),
 it is necessary to impose an *ordering* on its [keys](<#key/information model>). This order is a [serialization detail](<#serialization detail/>) and should not be used when [composing](#compose/) the [representation graph](#representation/) (and hence
 for the preservation of important data). In every case where [node](<#node/information model>) order is significant, a [sequence](<#sequence/information model>) must be used. For example, an ordered [mapping](<#mapping/information model>) can be [represented](#represent/) as a [sequence](<#sequence/information model>) of [mappings](<#mapping/information model>), where each [mapping](<#mapping/information model>) is a single [key:](<#key/information model>) [value](<#value/information model>) pair. YAML provides
 convenient compact notation for this case.

#### 3.2.2.2. Anchors and Aliases

In the [representation graph](#representation/), a [node](<#node/information model>) may appear in more than one [collection](<#collection/information model>). When [serializing](#serialize/) such data, the first
 occurrence of the [node](<#node/information model>) is *identified* by an *anchor* and
 each subsequent occurrence is [serialized](#serialize/) as an *alias node* which refers back to this anchor. Otherwise, anchor names are a [serialization detail](<#serialization detail/>) and are discarded once [composing](#compose/) is completed. When [composing](#compose/) a [representation graph](#representation/) from [serialized](#serialize/) events, an alias
 node refers to the most recent [node](<#node/information model>) in the [serialization](#serialization/) having the
 specified anchor. Therefore, anchors need not be unique within a [serialization](#serialization/). In
 addition, an anchor need not have an alias node referring to it. It
 is therefore possible to provide an anchor for all [nodes](<#node/information model>) in [serialization](#serialization/).

### 3.2.3. Presentation Stream

A YAML *presentation* is a *stream* of Unicode characters making use of of [styles](#style/), [formats](#format/), [comments](<#comment/information model>) [directives](<#directive/information model>) and other [presentation details](<#presentation detail/>) to [present](#present/) a YAML [serialization](#serialization/) in a human readable
 way. Although a YAML [processor](#processor/) may provide these [details](<#presentation detail/>) when [parsing](#parse/), they should not be reflected in
 the resulting [serialization](#serialization/). YAML allows several [serializations](#serialization/) to be
 contained in the same YAML character stream as a series of *documents* separated by [document boundary markers](<#document boundary marker/>). Documents appearing in the same stream
 are independent; that is, a [node](<#node/information model>) must not appear in more
 than one [serialization tree](#serialization/) or [representation graph](#representation/).

**Figure 3.5. Presentation Model**

![Presentation Model](present2.png)

#### 3.2.3.1. Node Styles

Each [node](<#node/information model>) is presented in some *style*, depending on its [kind](#kind/). The node style is a [presentation detail](<#presentation detail/>) and is
 not reflected in the [serialization tree](#serialization/) or [representation graph](#representation/). There are two groups of styles, *block* and *flow*. Block styles use [indentation](<#indentation space/>) to denote nesting
 and scope within the [document](<#document/information model>). In contrast, flow
 styles rely on explicit [indicators](#indicator/) to denote nesting and
 scope.

YAML provides a rich set of [scalar styles](<#scalar/information model>). *Block
 scalar styles* include the *literal style* and
 the *folded style*; *flow scalar styles* include
 the *plain style* and two *quoted styles*, the *single-quoted style* and the *double-quoted style*. These styles offer a range of
 trade-offs between expressive power and readability.

Normally, the [content](<#content/information model>) of *block collections* begins on the next line. In most cases, YAML also allows block
 collections to start *in-line* for more compact
 notation when nesting *block sequences* and *block mappings* inside each other. When nesting *flow collections*, a *flow mapping* with a *single key: value pair* may be specified
 directly inside a *flow sequence*, allowing for
 a compact "ordered mapping" notation.

**Figure 3.6. Kind/Style Combinations**

![Kind/Style Combinations](styles2.png)

#### 3.2.3.2. Scalar Formats

YAML allows [scalar content](<#scalar/information model>) to be [presented](#present/) in several *formats*. For example, the boolean
 "**`true`**" might also be written as
 "**`yes`**". [Tags](<#tag/information model>) must specify a mechanism for converting any
 formatted [scalar content](<#scalar/information model>) to a [canonical form](<#canonical form/>) for use in [equality](#equality/) testing. Like [node style](#style/), the format is a [presentation detail](<#presentation detail/>) and is
 not reflected in the [serialization tree](#serialization/) and [representation graph](#representation/).

#### 3.2.3.3. Comments

*Comments* are a [presentation detail](<#presentation detail/>) and must not have any effect
 on the [serialization tree](#serialization/) or [representation graph](#representation/). In particular, comments are not associated with a
 particular [node](<#node/information model>). The usual purpose of a comment is to
 communicate between the human maintainers of a file. A typical
 example is comments in a configuration file. Comments may not
 appear inside [scalars](<#scalar/information model>), but may be interleaved with such [scalars](<#scalar/information model>) inside [collections](<#collection/information model>).

#### 3.2.3.4. Directives

Each [document](<#document/information model>) may be associated with a set of *directives*. A directive has a name and an optional
 sequence of parameters. Directives are instructions to the YAML [processor](#processor/), and like all
 other [presentation details](<#presentation detail/>) are not reflected in the YAML [serialization tree](#serialization/) or [representation graph](#representation/). This
 version of YAML defines a two directives, ["**`YAML`**"](<#YAML directive/>) and ["**`TAG`**"](<#TAG directive/>). All other
 directives are [reserved](<#reserved directive/>) for future versions of YAML.

## 3.3. Loading Failure Points

The process of [loading](#load/) native data
 structures from a YAML [stream](<#stream/information model>) has several potential *failure points*. The character [stream](<#stream/information model>) may be [ill-formed](<#ill-formed stream/>), [aliases](<#alias/information model>) may be [unidentified](<#unidentified alias/>), [unspecified tags](<#non-specific tag/>) may be [unresolvable](<#unresolved tag/>), [tags](<#tag/information model>) may be [unrecognized](<#unrecognized tag/>), the [content](<#content/information model>) may
 be [invalid](<#invalid content/>), and a native
 type may be [unavailable](<#unavailable tag/>).
 Each of these failures results with an incomplete loading.

A *partial
 representation* need not [resolve](<#tag resolution/>) the [tag](<#tag/information model>) of each [node](<#node/information model>), and the [canonical form](<#canonical form/>) of [scalar content](<#scalar/information model>) need not be available. This weaker representation is useful for cases
 of incomplete knowledge of the types used in the [document](<#document/information model>). In
 contrast, a *complete
 representation* specifies the [tag](<#tag/information model>) of each [node](<#node/information model>), and
 provides the [canonical form](<#canonical form/>) of [scalar content](<#scalar/information model>), allowing for [equality](#equality/) testing. A complete
 representation is required in order to [construct](#construct/) native data structures.

**Figure 3.7. Loading Failure Points**

![Loading Failure Points](validity2.png)

### 3.3.1. Well-Formed and Identified

A *well-formed* character [stream](<#stream/information model>) must match the productions specified in the
 next chapter. Successful loading also requires that each [alias](<#alias/information model>) shall
 refer to a previous [node](<#node/information model>) [identified](#identified/) by the [anchor](<#anchor/information model>). A
 YAML [processor](#processor/) should reject *ill-formed streams* and *unidentified aliases*.
 A YAML [processor](#processor/) may recover
 from syntax errors, possibly by ignoring certain parts of the input,
 but it must provide a mechanism for reporting such errors.

### 3.3.2. Resolved

It is not required that all the [tags](<#tag/information model>) of the [complete representation](<#complete representation/>) be explicitly specified in the character [stream](<#stream/information model>). During [parsing](#parse/), [nodes](<#node/information model>) that omit the [tag](<#tag/information model>) are given a *non-specific tag*: *"**`?`**"* for [plain scalars](<#plain style/information model>) and *"**`!`**"* for all other [nodes](<#node/information model>). These
 non-specific tags must be *resolved* to a *specific tag* (either a [local tag](<#local tag/>) or a [global tag](<#global tag/>)) for a [complete representation](<#complete representation/>) to be [composed](#compose/).

Resolving the [tag](<#tag/information model>) of a [node](<#node/information model>) must only depend on
 the following three parameters: the non-specific tag of the [node](<#node/information model>), the path
 leading from the [root node](<#root node/>) to
 the [node](<#node/information model>), and the [content](<#content/information model>) (and hence the [kind](#kind/)) of the [node](<#node/information model>). In
 particular, resolution must not consider [presentation details](<#presentation detail/>) such as [comments](<#comment/information model>), [indentation](<#indentation space/>) and [node style](#style/). Also, resolution must not consider the [content](<#content/information model>) of
 any other [node](<#node/information model>), except for the [content](<#content/information model>) of the [key nodes](<#key/information model>) directly along the path leading from the [root node](<#root node/>) to the resolved [node](<#node/information model>). In particular,
 resolution must not consider the [content](<#content/information model>) of a sibling [node](<#node/information model>) in a [collection](<#collection/information model>) or the [content](<#content/information model>) of the [value node](<#value/information model>) associated with a resolved [key node](<#key/information model>).

Tag resolution is specific to the [application](#application/), hence a YAML [processor](#processor/) should provide a mechanism
 allowing the [application](#application/) to
 specify the tag resolution rules. It is recommended that [nodes](<#node/information model>) having
 the "**`!`**" non-specific tag should be resolved as
 "**`tag:yaml.org,2002:seq`**",
 "**`tag:yaml.org,2002:map`**" or
 "**`tag:yaml.org,2002:str`**" depending on the [node's kind](<#node/information model>). This convention allows the author of a YAML character [stream](<#stream/information model>) to exert some measure of control over the tag
 resolution process. By explicitly specifying a [plain scalar](<#plain style/information model>) has the "**`!`**" non-specific tag, the [node](<#node/information model>) is resolved as a string, as if it was [quoted](<#quoted style/information model>) or written in a [block style](<#block style/information model>). Note, however, that each [application](#application/) may override this
 behavior. For example, an [application](#application/) may automatically detect
 the type of programming language used in source code [presented](#present/) as a non-[plain](<#plain style/information model>) [scalar](<#scalar/information model>) and resolve it accordingly.

When a [node](<#node/information model>) has more than one occurrence (using an [anchor](<#anchor/information model>) and [alias nodes](<#alias/information model>)), tag resolution must depend only on the path to the
 first occurrence of the [node](<#node/information model>). Typically, the path leading to a [node](<#node/information model>) is
 sufficient to determine its specific tag. In cases where the path
 does not imply a single specific tag, the resolution also needs to
 consider the [node content](<#content/information model>) to select amongst the set of possible [tags](<#tag/information model>).
 Thus, [plain scalars](<#plain style/information model>) may be matched against a set of
 regular expressions to provide automatic resolution of integers,
 floats, timestamps, and similar types. Similarly, the [content](<#content/information model>) of [mapping nodes](<#mapping/information model>) may be matched against sets of expected [keys](<#key/information model>) to
 automatically resolve points, complex numbers, and similar types.

The combined effect of these rules is to ensure that tag resolution
 can be performed as soon as a [node](<#node/information model>) is first encountered in
 the [stream](<#stream/information model>), typically before its [content](<#content/information model>) is [parsed](#parse/). Also, tag resolution only
 requires referring to a relatively small number of previously parsed [nodes](<#node/information model>). Thus, tag resolution in one-pass [processors](#processor/) is both possible and
 practical.

If a [document](<#document/information model>) contains *unresolved tags*, the YAML [processor](#processor/) is unable to [compose](#compose/) a [complete representation](<#complete representation/>) graph. In such a
 case, the YAML [processor](#processor/) may [compose](#compose/) an [partial representation](<#partial representation/>),
 based on each [node's kind](#kind/) and
 allowing for non-specific tags.

### 3.3.3. Recognized and Valid

To be *valid*, a [node](<#node/information model>) must have
 a [tag](<#tag/information model>) which is *recognized* by
 the YAML [processor](#processor/) and its [content](<#content/information model>) must satisfy the constraints imposed by this [tag](<#tag/information model>).
 If a [document](<#document/information model>) contains a [scalar node](<#scalar/information model>) with an *unrecognized tag* or *invalid content*, only a [partial representation](<#partial representation/>) may
 be [composed](#compose/). In contrast, a YAML [processor](#processor/) can always [compose](#compose/) a [complete representation](<#complete representation/>) for an unrecognized
 or an invalid [collection](<#collection/information model>), since [collection](<#collection/information model>) [equality](#equality/) does not depend upon knowledge
 of the [collection's](<#collection/information model>) data type. However, such a [complete representation](<#complete representation/>) can not be used to [construct](#construct/) a
 native data structure.

### 3.3.4. Available

In a given processing environment, there need not be an *available* native type corresponding
 to a given [tag](<#tag/information model>). If a [node's tag](<#tag/information model>) is *unavailable*, a YAML [processor](#processor/) will not be able to [construct](#construct/) a native data structure for
 it. In this case, a [complete representation](<#complete representation/>) may still be [composed](#compose/), and an [application](#application/) may wish to use this [representation](#representation/) directly.

## Chapter 4. Productions Conventions

The following chapters describe the syntax of YAML character [streams](#stream/syntax) in detail using a
 series of BNF productions. In most cases, productions are introduced in a
 "bottom-up" order; basic productions are specified before
 the more complex productions using them. Examples accompanying the
 productions display sample YAML text side-by-side with equivalent YAML
 text using only [flow collections](<#flow collection style/syntax>) and [double-quoted scalars](<#double-quoted style/syntax>). For improved readability, the equivalent YAML text
 uses the "**`!!seq`**", "**`!!map`**", and
 "**`!!str`**" [shorthands](<#tag shorthand/>) instead of the [verbatim](<#verbatim tag/>) "**`!<tag:yaml.org,2002:seq>`**",
 "**`!<tag:yaml.org,2002:map>`**" and
 "**`!<tag:yaml.org,2002:str>`**" forms. These types are
 used to [resolve](<#tag resolution/>) all [untagged nodes](<#non-specific tag/>), except for a few
 examples that use the "**`!!int`**" and "**`!!float`**"
 types.

## 4.1. Production Prefixes

To make the syntax easier to follow, production names use
 Hungarian-style notation. Each production is given one of the following
 prefix based on the type of characters it matches.

**`e-`**
A production matching no characters.

**`c-`**
A production matching one or more characters starting and ending
 with a special (non-space) character.

**`b-`**
A production matching a single [line break](<#line break character/>).

**`nb-`**
A production matching one or more characters starting and ending
 with a non-[break](<#line break character/>) character.

**`s-`**
A production matching one or more characters starting and ending
 with a space character.

**`ns-`**
A production matching one or more characters starting and ending
 with a non-space character.

`X`**`-`**`Y`**`-`**
A production matching a sequence of one or more characters,
 starting with an `X`**`-`** character and ending with a `Y`**`-`** character.

**`l-`**
A production matching one or more lines (shorthand for **`s-b-`**).

`X`**`+`**, `X`**`-`**`Y`**`+`**
A production as above, with the additional property that the [indentation](<#indentation space/>) level
 used is greater than the specified `n` parameter.

## 4.2. Production Parameters

As YAML's syntax is designed for maximal readability, it makes heavy
 use of the context that each syntactical entity appears in. For
 notational compactness, this is expressed using parameterized BNF
 productions. The set of parameters and the range of allowed values
 depend on the specific production. The full list of possible parameters
 and their values is:

Indentation: `n` or `m`
Since the character [stream](#stream/syntax) depends upon [indentation](<#indentation space/>) level to
 delineate blocks, many productions are parameterized by it. In some
 cases, the notations "**`production(<n)`**",
 "**`production(≤n)`**" and
 "**`production(>n)`**" are used; these are shorthands
 for "**`production(m)`**" for some specific `m` where 0 ≤ `m` < `n`,
 0 ≤ `m` ≤ `n` and `m` > `n`,
 respectively.

Context: `c`
YAML supports two groups of *contexts*, distinguishing between [block styles](<#block style/syntax>) and [flow styles](<#flow style/syntax>). In the [block styles](<#block style/syntax>), [indentation](<#indentation space/>) is used
 to delineate structure. Due to the fact that the ["**`-`**"](<#- block sequence entry/>) character denoting a [block sequence](<#block sequence style/syntax>) entry is perceived as
 an [indentation](<#indentation space/>) character, some productions distinguish between the [block-in](<#block-in context/>) context (inside a [block sequence](<#block sequence style/syntax>)) and the [block-out](<#block-out context/>) context (outside one). In the [flow styles](<#flow style/syntax>),
 explicit [indicators](#indicator/) are used
 to delineate structure. As [plain scalars](<#plain style/syntax>) have no such [indicators](#indicator/), they are the most context
 sensitive, distinguishing between being nested inside a [flow collection](<#flow collection style/syntax>) ([flow-in](<#flow-in context/>) context) or being outside one ([flow-out](<#flow-out context/>) context). YAML also
 provides a terse and intuitive syntax for [simple keys](<#simple key/>). [Plain scalars](<#plain style/syntax>) in this ([flow-key](<#flow-key context/>)) context are the most
 restricted, for readability and implementation reasons.

(Scalar) Style: `s`
[Scalar content](#scalar/syntax) may be [presented](#present/) in one of five [styles](#scalar/syntax): the [plain](<#plain style/syntax>), [double-quoted](<#double-quoted style/syntax>) and [single-quoted](<#single-quoted style/syntax>)[flow styles](<#flow style/syntax>), and the [literal](<#literal style/syntax>) and [folded](<#folded style/syntax>)[block styles](<#block style/syntax>).

(Block) Chomping: `t`
Block scalars offer three possible mechanisms for [chomping](#chomping/) any trailing [line breaks](<#line break character/>): [strip](<#strip chomping/>), [clip](<#clip chomping/>) and [keep](<#keep chomping/>).

## Chapter 5. Characters

## 5.1. Character Set

YAML [streams](#stream/syntax) use the *printable* subset of the Unicode character set. On input, a YAML [processor](#processor/) must accept all printable
 ASCII characters, the space, [tab](#tab/), [line break](<#line break character/>), and all
 Unicode characters beyond #x9F. On output, a YAML [processor](#processor/) must only produce these
 acceptable characters, and should also [escape](<#escaping in double-quoted style/>) all non-printable Unicode
 characters. The allowed character range explicitly excludes the
 surrogate block **`#xD800-#xDFFF`**, DEL **`#x7F`**, the C0 control block **`#x0-#x1F`** (except for **`#x9`**, **`#xA`**, and **`#xD`**), the C1 control block **`#x80-#x9F`**, **`#xFFFE`**, and **`#xFFFF`**. Any such characters must be [presented](#present/) using [escape](<#escaping in double-quoted style/>) sequences.

- | [1] | c-printable | `::=` | #x9 \| #xA \| #xD \| [#x20-#x7E] /* 8 bit */ \| #x85 \| [#xA0-#xD7FF] \| [#xE000-#xFFFD] /* 16 bit */ \| [#x10000-#x10FFFF] /* 32 bit */ | |
| --- | --- | --- | --- | --- |

## 5.2. Character Encoding

All characters mentioned in this specification are Unicode code
 points. Each such code point is written as one or more octets
 depending on the *character
 encoding* used. Note that in UTF-16, characters above **`#xFFFF`** are written as four octets, using a
 surrogate pair. A YAML [processor](#processor/) must support the UTF-16 and
 UTF-8 character encodings. If a character [stream](#stream/syntax) does not begin with a *byte order mark* (**`#FEFF`**), the character encoding shall be
 UTF-8. Otherwise it shall be either UTF-8, UTF-16 LE, or UTF-16 BE as
 indicated by the byte order mark. On output, it is recommended that a
 byte order mark should only be emitted for UTF-16 character
 encodings. Note that the UTF-32 encoding is explicitly not supported.
 For more information about the byte order mark and the Unicode
 character encoding schemes see the Unicode [FAQ](http://www.unicode.org/unicode/faq/utf_bom.html).

- | [2] | c-byte-order-mark | `::=` | #xFEFF | |
| --- | --- | --- | --- | --- |


In the examples, byte order mark characters are displayed as
 "**`⇔`**".

**Example 5.1. Byte Order Mark**

| ``` ⇔# Comment only.    ```  ``` Legend:   [c-byte-order-mark](#c-byte-order-mark)  ``` | ``` # This stream contains no   # documents, only comments.  ``` |
| --- | --- |

**Example 5.2. Invalid Byte Order Mark**

| ``` # Invalid use of BOM   ⇔# inside a # document.  ``` | ``` ERROR:    A BOM must not appear  inside a document.  ``` |
| --- | --- |

## 5.3. Indicator Characters

*Indicators* are characters that
 have special semantics used to describe the structure and [content](#content/syntax) of a YAML [document](#document/syntax).

- A ["**`-`**"](<#- block sequence entry/>) (**`#2D`**,
 hyphen) denotes a [block sequence](<#block sequence style/syntax>) entry.

- | [3] | c-sequence-entry | `::=` | "-" | |
| --- | --- | --- | --- | --- |


- A ["**`?`**"](<#? mapping key/>) (**`#3F`**, question mark) denotes a [mapping key](#key/syntax).

- | [4] | c-mapping-key | `::=` | "?" | |
| --- | --- | --- | --- | --- |


- A ["**`:`**"](<#: mapping value/>) (**`#3A`**, colon) denotes a [mapping value](#value/syntax).

- | [5] | c-mapping-value | `::=` | ":" | |
| --- | --- | --- | --- | --- |


**Example 5.3. Block Structure Indicators**

- ```
sequence:
- one
- two
mapping:
 ? sky
 : blue
 ? sea : green

```
```
Legend:
 [c-sequence-entry](#c-sequence-entry)
 [c-mapping-key](#c-mapping-key)
 [c-mapping-value](#c-mapping-value)

``` ```
%YAML 1.1
---
!!map {
 ? !!str "sequence"
 : !!seq [
 !!str "one", !!str "two"
 ],
 ? !!str "mapping"
 : !!map {
 ? !!str "sky" : !!str "blue",
 ? !!str "sea" : !!str "green",
 }
}

```

- A ["**`,`**"](<#, end flow entry/>) (**`#2C`**, comma) ends a [flow collection](<#flow collection style/syntax>) entry.

- | [6] | c-collect-entry | `::=` | "," | |
| --- | --- | --- | --- | --- |


- A ["**`[`**"](<#[ start flow sequence/>) (**`#5B`**,
 left bracket) starts a [flow sequence](<#flow sequence style/syntax>).

- | [7] | c-sequence-start | `::=` | "[" | |
| --- | --- | --- | --- | --- |


- A ["**`\]`**"](<#] end flow sequence/>) (**`#5D`**,
 right bracket) ends a [flow sequence](<#flow sequence style/syntax>).

- | [8] | c-sequence-end | `::=` | "]" | |
| --- | --- | --- | --- | --- |


- A ["**`{`**"](<#{ start flow mapping/>) (**`#7B`**,
 left brace) starts a [flow mapping](<#flow mapping style/syntax>).

- | [9] | c-mapping-start | `::=` | "{" | |
| --- | --- | --- | --- | --- |


- A ["**`}`**"](<#} end flow mapping/>) (**`#7D`**,
 right brace) ends a [flow mapping](<#flow mapping style/syntax>).

- | [10] | c-mapping-end | `::=` | "}" | |
| --- | --- | --- | --- | --- |


**Example 5.4. Flow Collection Indicators**

- ```
sequence: [ one, two, ]
mapping: { sky: blue, sea: green }

```
```
Legend:
 [c-sequence-start](#c-sequence-start) [c-sequence-end](#c-sequence-end)
 [c-mapping-start](#c-mapping-start) [c-mapping-end](#c-mapping-end)
 [c-collect-entry](#c-collect-entry)

``` ```
%YAML 1.1
---
!!map {
 ? !!str "sequence"
 : !!seq [
 !!str "one", !!str "two"
 ],
 ? !!str "mapping"
 : !!map {
 ? !!str "sky" : !!str "blue",
 ? !!str "sea" : !!str "green",
 }
}

```

- An ["**`#`**"](<## comment/>) (**`#23`**, octothorpe, hash, sharp, number sign)
 denotes a [comment](#comment/syntax).

- | [11] | c-comment | `::=` | "#" | |
| --- | --- | --- | --- | --- |


**Example 5.5. Comment Indicator**

| ``` # Comment only.    ```  ``` Legend:   [c-comment](#c-comment)  ``` | ``` # This stream contains no   # documents, only comments.  ``` |
| --- | --- |

- An ["**`&`**"](<#& anchor/>) (**`#26`**, ampersand) denotes a [node's anchor property](#anchor/syntax).

- | [12] | c-anchor | `::=` | "&" | |
| --- | --- | --- | --- | --- |


- An ["**`*`**"](<#* alias/>) (**`#2A`**, asterisk) denotes an [alias node](#alias/syntax).

- | [13] | c-alias | `::=` | "*" | |
| --- | --- | --- | --- | --- |


- An ["**`!`**"](<#! tag indicator/>) (**`#21`**, exclamation) denotes a [node's tag](#tag/syntax).

- | [14] | c-tag | `::=` | "!" | |
| --- | --- | --- | --- | --- |


**Example 5.6. Node Property Indicators**

- ```

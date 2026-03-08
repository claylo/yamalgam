# YAML Ain't Markup Language (YAML™) Version 1.2

## Revision 1.2.0 (2009-07-21)

### Oren Ben-Kiki

`<[oren@ben-kiki.org](mailto:oren@ben-kiki.org)>`

### Clark Evans

`<[cce@clarkevans.com](mailto:cce@clarkevans.com)>`

### Ingy döt Net

`<[ingy@ingy.net](mailto:ingy@ingy.net)>`

**Abstract**

YAML™ (rhymes with "camel") is a
 human-friendly, cross language, Unicode based data serialization
 language designed around the common native data types of agile
 programming languages. It is broadly useful for programming needs
 ranging from configuration files to Internet messaging to object
 persistence to data auditing. Together with the [Unicode standard for characters](http://www.unicode.org/),
 this specification provides all the information necessary to understand
 YAML Version 1.2 and to create programs that process YAML information.

## Chapter 1. Introduction

"YAML Ain't Markup Language" (abbreviated YAML) is a data
 serialization language designed to be human-friendly and work well with
 modern programming languages for common everyday tasks. This
 specification is both an introduction to the YAML language and the
 concepts supporting it, and also a complete specification of the
 information needed to develop [applications](#application//) for processing YAML.

Open, interoperable and readily understandable tools have advanced
 computing immensely. YAML was designed from the start to be useful and
 friendly to people working with data. It uses Unicode [printable](<#printable character//>) characters, [some](#indicator//) of which provide structural
 information and the rest containing the data itself. YAML achieves a
 unique cleanness by minimizing the amount of structural characters and
 allowing the data to show itself in a natural and meaningful way. For
 example, [indentation](#space/indentation/) may be used for structure, [colons](<#: mapping value//>) separate [key: value pairs](<#key: value pair//>), and [dashes](<#- block sequence entry//>) are used to create
 "bullet" [lists](#sequence//).

There are myriad flavors of [data structures](<#native data structure//>), but they can all be adequately [represented](#represent//) with three basic primitives: [mappings](#mapping//) (hashes/dictionaries), [sequences](#sequence//) (arrays/lists) and [scalars](#scalar//) (strings/numbers). YAML
 leverages these primitives, and adds a simple typing system and [aliasing](#alias//) mechanism to form a complete language
 for [serializing](#serialize//) any [native data structure](<#native data structure//>). While
 most programming languages can use YAML for data serialization, YAML
 excels in working with those languages that are fundamentally built
 around the three basic primitives. These include the new wave of agile
 languages such as Perl, Python, PHP, Ruby, and Javascript.

There are hundreds of different languages for programming, but only a
 handful of languages for storing and transferring data. Even though its
 potential is virtually boundless, YAML was specifically created to work
 well for common use cases such as: configuration files, log files,
 interprocess messaging, cross-language data sharing, object persistence,
 and debugging of complex data structures. When data is easy to view and
 understand, programming becomes a simpler task.

## 1.1. Goals

The design goals for YAML are, in decreasing priority:

1. YAML is easily readable by humans.
2. YAML matches the [native data structures](<#native data structure//>) of agile languages.
3. YAML data is portable between programming languages.
4. YAML has a consistent model to support generic tools.
5. YAML supports one-pass processing.
6. YAML is expressive and extensible.
7. YAML is easy to implement and use.

## 1.2. Prior Art

YAML's initial direction was set by the data serialization and
 markup language discussions among [SML-DEV members](http://www.docuverse.com/smldev/). Later
 on, it directly incorporated experience from Ingy döt Net's
 Perl module [Data::Denter](http://search.cpan.org/dist/Data-Denter/). Since then, YAML has matured through ideas and
 support from its user community.

YAML integrates and builds upon concepts described by [C](http://cm.bell-labs.com/cm/cs/cbook/index.html), [Java](http://java.sun.com/), [Perl](http://www.perl.org/), [Python](http://www.python.org/), [Ruby](http://www.ruby-lang.org/), [RFC0822](http://www.ietf.org/rfc/rfc0822.txt) (MAIL), [RFC1866](http://www.ics.uci.edu/pub/ietf/html/rfc1866.txt) (HTML), [RFC2045](http://www.ietf.org/rfc/rfc2045.txt) (MIME), [RFC2396](http://www.ietf.org/rfc/rfc2396.txt) (URI), [XML](http://www.w3.org/TR/REC-xml.html), [SAX](http://www.saxproject.org/), [SOAP](http://www.w3.org/TR/SOAP), and [JSON](http://www.json.org/).

The syntax of YAML was motivated by Internet Mail (RFC0822) and remains
 partially compatible with that standard. Further, borrowing from MIME
 (RFC2045), YAML's top-level production is a [stream](#stream//) of independent [documents](#document//), ideal for message-based
 distributed processing systems.

YAML's [indentation](#space/indentation/)-based scoping is similar
 to Python's (without the ambiguities caused by [tabs](#tab//)). [Indented blocks](#style/block/) facilitate easy inspection
 of the data's structure. YAML's [literal style](#style/block/literal) leverages
 this by enabling formatted text to be cleanly mixed within an [indented](#space/indentation/) structure
 without troublesome [escaping](<#escaping/in double-quoted scalars/>). YAML also allows the use of
 traditional [indicator](#indicator//)-based
 scoping similar to JSON's and Perl's. Such [flow content](#style/flow/) can be freely
 nested inside [indented blocks](#style/block/).

YAML's [double-quoted style](#style/flow/double-quoted) uses familiar
 C-style [escape sequences](<#escaping/in double-quoted scalars/>). This enables ASCII encoding of
 non-[printable](<#printable character//>) or 8-bit
 (ISO 8859-1) characters such as ["**`\x3B`**"](#ns-esc-8-bit). Non-[printable](<#printable character//>) 16-bit Unicode and
 32-bit (ISO/IEC 10646) characters are supported with [escape sequences](<#escaping/in double-quoted scalars/>) such as ["**`\u003B`**"](#ns-esc-16-bit) and ["**`\U0000003B`**"](#ns-esc-32-bit).

Motivated by HTML's end-of-line normalization, YAML's [line folding](<#line folding//>) employs an intuitive
 method of handling [line breaks](<#line break//>).
 A single [line break](<#line break//>) is [folded](<#line folding//>) into a single [space](#space//), while [empty lines](<#empty line//>) are interpreted as [line break](<#line break//>) characters. This technique allows for
 paragraphs to be word-wrapped without affecting the [canonical form](<#scalar/canonical form/>) of
 the [scalar content](#scalar//).

YAML's core type system is based on the requirements of agile
 languages such as Perl, Python, and Ruby. YAML directly supports both [collections](#collection//) ([mappings](#mapping//), [sequences](#sequence//)) and [scalars](#scalar//). Support for these common types
 enables programmers to use their language's [native data structures](<#native data structure//>) for YAML manipulation,
 instead of requiring a special document object model (DOM).

Like XML's SOAP, YAML supports [serializing](#serialize//) a graph of [native data structures](<#native data structure//>) through an [aliasing](#alias//) mechanism. Also
 like SOAP, YAML provides for [application](#application//)-defined [types](#tag//). This allows YAML to [represent](#represent//) rich data structures required
 for modern distributed computing. YAML provides globally unique [type names](#tag/global/) using a
 namespace mechanism inspired by Java's DNS-based package naming
 convention and XML's URI-based namespaces. In addition, YAML allows
 for private [types](#tag/local/) specific to a single [application](#application//).

YAML was designed to support incremental interfaces that include both
 input ("**`getNextEvent()`**") and output
 ("**`sendNextEvent()`**") one-pass interfaces. Together, these
 enable YAML to support the processing of large [documents](#document//) (e.g. transaction logs) or
 continuous [streams](#stream//) (e.g. feeds from
 a production machine).

## 1.3. Relation to JSON

Both JSON and YAML aim to be human readable data interchange formats.
 However, JSON and YAML have different priorities. JSON's foremost
 design goal is simplicity and universality. Thus, JSON is trivial to
 generate and parse, at the cost of reduced human readability. It also
 uses a lowest common denominator information model, ensuring any JSON
 data can be easily processed by every modern programming environment.

In contrast, YAML's foremost design goals are human readability and
 support for [serializing](#serialize//) arbitrary [native data structures](<#native data structure//>). Thus, YAML allows for extremely readable files,
 but is more complex to generate and parse. In addition, YAML ventures
 beyond the lowest common denominator data types, requiring more complex
 processing when crossing between different programming environments.

YAML can therefore be viewed as a natural superset of JSON, offering
 improved human readability and a more complete information model. This
 is also the case in practice; every JSON file is also a valid YAML
 file. This makes it easy to migrate from JSON to YAML if/when the
 additional features are required.

It may be useful to define a intermediate format between YAML and JSON.
 Such a format would be trivial to parse (but not very human readable),
 like JSON. At the same time, it would allow for [serializing](#serialize//) arbitrary [native data structures](<#native data structure//>), like
 YAML. Such a format might also serve as YAML's "canonical format".
 Defining such a "YSON" format (YSON is a Serialized Object
 Notation) can be done either by enhancing the JSON specification or by
 restricting the YAML specification. Such a definition is beyond the
 scope of this specification.

## 1.4. Relation to XML

Newcomers to YAML often search for its correlation to the eXtensible
 Markup Language (XML). Although the two languages may actually compete
 in several application domains, there is no direct correlation between
 them.

YAML is primarily a data serialization language. XML was designed to be
 backwards compatible with the Standard Generalized Markup Language
 (SGML), which was designed to support structured documentation. XML
 therefore had many design constraints placed on it that YAML does not
 share. XML is a pioneer in many domains, YAML is the result of lessons
 learned from XML and other technologies.

It should be mentioned that there are ongoing efforts to define
 standard XML/YAML mappings. This generally requires that a subset of
 each language be used. For more information on using both XML and YAML,
 please visit [http://yaml.org/xml](http://yaml.org/xml).

## 1.5. Terminology

This specification uses key words based on [RFC2119](http://www.ietf.org/rfc/rfc2119.txt) to indicate
 requirement level. In particular, the following words are used to
 describe the actions of a YAML [processor](#processor//):

May
The word *may*, or the adjective *optional*, mean that conforming YAML [processors](#processor//) are permitted to, but *need not* behave as described.

Should
The word *should*, or the adjective *recommended*, mean that there could be reasons
 for a YAML [processor](#processor//) to
 deviate from the behavior described, but that such deviation could
 hurt interoperability and should therefore be advertised with
 appropriate notice.

Must
The word *must*, or the term *required* or *shall*, mean that the behavior described
 is an absolute requirement of the specification.

The rest of this document is arranged as follows. Chapter [2](#Preview "Chapter&nbsp;2.&nbsp;Preview") provides a short preview of the main YAML
 features. Chapter [3](#Processing "Chapter&nbsp;3.&nbsp;Processing YAML Information") describes the YAML
 information model, and the processes for converting from and to this
 model and the YAML text format. The bulk of the document, chapters [4](#Syntax "Chapter&nbsp;4.&nbsp;Syntax Conventions") through [9](#YAML "Chapter&nbsp;9.&nbsp;YAML Character Stream"), formally
 define this text format. Finally, chapter [10](#Syntax "Chapter&nbsp;4.&nbsp;Syntax Conventions") recommends basic YAML schemas.

## Chapter 2. Preview

This section provides a quick glimpse into the expressive power of YAML.
 It is not expected that the first-time reader grok all of the examples.
 Rather, these selections are used as motivation for the remainder of the
 specification.

## 2.1. Collections

YAML's [block collections](#style/block/collection) use [indentation](#space/indentation/) for scope
 and begin each entry on its own line. [Block sequences](#style/block/sequence) indicate each entry with a dash and space ( ["**`- `**"](<#- block sequence entry//>)). [Mappings](#mapping//) use a colon and
 space (["**`: `**"](<#: mapping value//>)) to mark each [key: value pair](<#key: value pair//>). [Comments](#comment//) begin with an octothorpe (also
 called a "hash", "sharp",
 "pound", or "number sign" - ["**`#`**"](<## comment//>)).

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
YAML also has [flow styles](#style/flow/), using explicit [indicators](#indicator//) rather than [indentation](#space/indentation/) to denote
 scope. The [flow sequence](#style/flow/sequence) is written as a [comma](<#, end flow entry//>) separated list within [square](<#[ start flow sequence//>) [brackets](<#] end flow sequence//>). In a similar manner,
 the [flow mapping](#style/flow/mapping) uses [curly](<#{ start flow mapping//>) [braces](<#} end flow mapping//>).

| **Example 2.5. Sequence of Sequences**    ``` - [name        , hr, avg  ]   - [Mark McGwire, 65, 0.278] - [Sammy Sosa  , 63, 0.288]    ``` | **Example 2.6. Mapping of Mappings**    ``` Mark McGwire: {hr: 65, avg: 0.278}   Sammy Sosa: {     hr: 63,     avg: 0.288   }  ``` |
| --- | --- |

## 2.2. Structures

YAML uses three dashes (["**`---`**"](<#marker/directives end/>)) to separate [directives](#directive//) from [document](#document//) [content](#content//). This also serves to signal the
 start of a document if no [directives](#directive//) are present. Three dots ( ["**`...`**"](<#marker/document end/>)) indicate the end of a document
 without starting a new one, for use in communication channels.

| **Example 2.7.   Two Documents in a Stream     (each with a leading comment)**    ``` # Ranking of 1998 home runs   --- - Mark McGwire - Sammy Sosa - Ken Griffey  # Team ranking --- - Chicago Cubs - St Louis Cardinals  ``` | **Example 2.8.   Play by Play Feed     from a Game**    ``` ---   time: 20:03:20 player: Sammy Sosa action: strike (miss) ... --- time: 20:03:47 player: Sammy Sosa action: grand slam ...  ``` |
| --- | --- |


Repeated [nodes](#node//) (objects) are first [identified](#alias/identified/) by an [anchor](#anchor//) (marked with the
 ampersand - ["**`&`**"](<#& anchor//>)), and are then [aliased](#alias//) (referenced with an
 asterisk - ["**`*`**"](<#* alias//>)) thereafter.

| **Example 2.9.   Single Document with     Two Comments**    ``` ---   hr: # 1998 hr ranking   - Mark McGwire   - Sammy Sosa rbi:   # 1998 rbi ranking   - Sammy Sosa   - Ken Griffey  ``` | **Example 2.10.   Node for "`Sammy Sosa`"     appears twice in this document**    ``` ---   hr:   - Mark McGwire   # Following node labeled SS   - &SS Sammy Sosa rbi:   - *SS # Subsequent occurrence   - Ken Griffey  ``` |
| --- | --- |


A question mark and space (["**`? `**"](<#? mapping key//>) indicate a complex [mapping](#mapping//) [key](#key//). Within a [block collection](#style/block/collection), [key: value pairs](<#key: value pair//>) can
 start immediately following the [dash](<#- block sequence entry//>), [colon](<#: mapping value//>), or [question mark](<#? mapping key//>).

| **Example 2.11. Mapping between Sequences**    ``` ? - Detroit Tigers     - Chicago cubs :   - 2001-07-23  ? [ New York Yankees,     Atlanta Braves ] : [ 2001-07-02, 2001-08-12,     2001-08-14 ]  ``` | **Example 2.12. Compact Nested Mapping**    ``` ---   # Products purchased - item    : Super Hoop   quantity: 1 - item    : Basketball   quantity: 4 - item    : Big Shoes   quantity: 1   ``` |
| --- | --- |

## 2.3. Scalars

[Scalar content](#scalar//) can be written in [block](#style/block/) notation,
 using a [literal style](#style/block/literal) (indicated by ["**`|`**"](<#| literal style//>)) where all [line breaks](<#line break//>) are significant.
 Alternatively, they can be written with the [folded style](#style/block/folded) [(denoted by "**`>`**"](<#> folded style//>)) where each [line break](<#line break//>) is [folded](<#line folding//>) to a [space](#space//) unless it ends an [empty](<#empty line//>) or a [more-indented](#more-indented//) line.

- **Example 2.13.
 In literals,
newlines are preserved** ```
# ASCII Art
--- |
 \//||\/||
 // || ||__

``` **Example 2.14.
 In the folded scalars,
newlines become spaces** ```
--- >
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
YAML's [flow scalars](#style/flow/scalar) include the [plain style](#style/flow/plain) (most examples thus far) and two quoted styles. The [double-quoted style](#style/flow/double-quoted) provides [escape sequences](<#escaping/in double-quoted scalars/>). The [single-quoted style](#style/flow/) is useful when [escaping](<#escaping/in double-quoted scalars/>) is not needed.
 All [flow scalars](#style/flow/scalar) can span multiple lines; [line breaks](<#line break//>) are always [folded](<#line folding//>).

| **Example 2.17. Quoted Scalars**    ``` unicode: "Sosa did fine.\u263A"   control: "\b1998\t1999\t2000\n" hex esc: "\x0d\x0a is \r\n"  single: '"Howdy!" he cried.' quoted: ' # Not a ''comment''.' tie-fighter: '|\-*-/|'  ``` | **Example 2.18. Multi-line Flow Scalars**    ``` plain:     This unquoted scalar   spans many lines.  quoted: "So does this   quoted scalar.\n"   ``` |
| --- | --- |

## 2.4. Tags

In YAML, [untagged nodes](#tag/non-specific/) are given a type depending on the [application](#application//). The examples in this
 specification generally use the [**`seq`**](#tag/repository/seq), [**`map`**](#tag/repository/map) and [**`str`**](#tag/repository/str) types from the [fail safe schema](#schema/failsafe/). A few
 examples also use the [**`int`**](#tag/repository/int), [**`float`**](#tag/repository/float), and [**`null`**](#tag/repository/null) types from the [JSON schema](#schema/JSON/). The [repository](#tag/repository/) includes additional types such as [**`binary`**](/type/binary.html), [**`omap`**](/type/omap.html), [**`set`**](/type/set.html) and others.

- **Example 2.19. Integers**```
canonical: 12345
decimal: +12345
octal: 0o14
hexadecimal: 0xC


``` **Example 2.20. Floating Point**```
canonical: 1.23015e+3
exponential: 12.3015e+02
fixed: 1230.15
negative infinity: -.inf
not a number: .NaN

```
- **Example 2.21. Miscellaneous**```
null:
booleans: [ true, false ]
string: '012345'

``` **Example 2.22. Timestamps**```
canonical: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14

```
Explicit typing is denoted with a [tag](#tag//) using the exclamation point (["**`!`**"](<#! tag indicator//>)) symbol. [Global tags](#tag/global/) are URIs and may be specified
 in a [tag shorthand](#tag/shorthand/) notation using a [handle](#tag/handle/). [Application](#application//)-specific [local tags](#tag/local/) may also be used.

| **Example 2.23. Various Explicit Tags**    ``` ---   not-date: !!str 2002-04-28  picture: !!binary |  R0lGODlhDAAMAIQAAP//9/X  17unp5WZmZgAAAOfn515eXv  Pz7Y6OjuDg4J+fn5OTk6enp  56enmleECcgggoBADs=  application specific tag: !something |  The semantics of the tag  above may be different for  different documents.   ``` | **Example 2.24. Global Tags**    ``` %TAG ! tag:clarkevans.com,2002:   --- !shape   # Use the ! handle for presenting   # tag:clarkevans.com,2002:circle - !circle   center: &ORIGIN {x: 73, y: 129}   radius: 7 - !line   start: *ORIGIN   finish: { x: 89, y: 102 } - !label   start: *ORIGIN   color: 0xFFEEBB   text: Pretty vector drawing.  ``` |
| --- | --- |

| **Example 2.25. Unordered Sets**    ``` # Sets are represented as a   # Mapping where each key is # associated with a null value --- !!set ? Mark McGwire ? Sammy Sosa ? Ken Griff  ``` | **Example 2.26. Ordered Mappings**    ``` # Ordered maps are represented as   # A sequence of mappings, with # each mapping having one key --- !!omap - Mark McGwire: 65 - Sammy Sosa: 63 - Ken Griffy: 58  ``` |
| --- | --- |

## 2.5. Full Length Example

Below are two full-length examples of YAML. On the left is a sample
 invoice; on the right is a sample log file.

| **Example 2.27. Invoice**    ``` --- !<tag:clarkevans.com,2002:invoice>   invoice: 34843 date   : 2001-01-23 bill-to: &id001     given  : Chris     family : Dumars     address:         lines: |             458 Walkman Dr.             Suite #292         city    : Royal Oak         state   : MI         postal  : 48046 ship-to: *id001 product:     - sku         : BL394D       quantity    : 4       description : Basketball       price       : 450.00     - sku         : BL4438H       quantity    : 1       description : Super Hoop       price       : 2392.00 tax  : 251.42 total: 4443.52 comments:     Late afternoon is best.     Backup contact is Nancy     Billsmer @ 338-4338.  ``` | **Example 2.28. Log File**    ``` ---   Time: 2001-11-23 15:01:42 -5 User: ed Warning:   This is an error message   for the log file --- Time: 2001-11-23 15:02:31 -5 User: ed Warning:   A slightly different error   message. --- Date: 2001-11-23 15:03:17 -5 User: ed Fatal:   Unknown variable "bar" Stack:   - file: TopClass.py     line: 23     code: |       x = MoreObject("345\n")   - file: MoreClass.py     line: 58     code: |-       foo = bar     ``` |
| --- | --- |

## Chapter 3. Processing YAML Information

YAML is both a text format and a method for [presenting](#present//) any [native data structure](<#native data structure//>) in this format. Therefore,
 this specification defines two concepts: a class of data objects called
 YAML [representations](#representation//), and a
 syntax for [presenting](#present//) YAML [representations](#representation//) as a series of
 characters, called a YAML [stream](#stream//). A
 YAML *processor* is a tool for
 converting information between these complementary views. It is assumed
 that a YAML processor does its work on behalf of another module, called
 an *application*. This chapter
 describes the information structures a YAML processor must provide to or
 obtain from the application.

YAML information is used in two ways: for machine processing, and for
 human consumption. The challenge of reconciling these two perspectives is
 best done in three distinct translation stages: [representation](#representation//), [serialization](#serialization//), and [presentation](#presentation//). [Representation](#representation//) addresses how YAML
 views [native data structures](<#native data structure//>) to achieve portability between programming
 environments. [Serialization](#serialization//) concerns itself with turning a YAML [representation](#representation//) into a serial form,
 that is, a form with sequential access constraints. [Presentation](#presentation//) deals with the formatting
 of a YAML [serialization](#serialization//) as a
 series of characters in a human-friendly manner.

## 3.1. Processes

Translating between [native data structures](<#native data structure//>) and a character [stream](#stream//) is done in several logically distinct
 stages, each with a well defined input and output data model, as shown
 in the following diagram:

**Figure 3.1. Processing Overview**

![Processing Overview](overview2.png)


A YAML processor need not expose the [serialization](#serialization//) or [representation](#representation//) stages. It may
 translate directly between [native data structures](<#native data structure//>) and a character [stream](#stream//) ([dump](#dump//) and [load](#load//) in the diagram above). However, such a
 direct translation should take place so that the [native data structures](<#native data structure//>) are [constructed](#construct//) only from
 information available in the [representation](#representation//). In particular, [mapping key order](#key/order/), [comments](#comment//), and [tag handles](#tag/handle/) should not be
 refernced during [composition](#compose//).

### 3.1.1. Dump

*Dumping* native data structures to a
 character [stream](#stream//) is done using
 the following three stages:

Representing Native Data Structures
YAML *represents* any *native data
 structure* using three [node kinds](#kind//): [sequence](#sequence//) - an ordered series of entries; [mapping](#mapping//) - an unordered association
 of [unique](#equality//) [keys](#key//) to [values](#value//); and [scalar](#scalar//) - any datum with opaque
 structure presentable as a series of Unicode characters.
 Combined, these primitives generate directed graph structures.
 These primitives were chosen because they are both powerful and
 familiar: the [sequence](#sequence//) corresponds to a Perl array and a Python list, the [mapping](#mapping//) corresponds to a Perl hash
 table and a Python dictionary. The [scalar](#scalar//) represents strings, integers,
 dates, and other atomic data types.

Each YAML [node](#node//) requires, in
 addition to its [kind](#kind//) and [content](#content//), a [tag](#tag//) specifying its data type. Type
 specifiers are either [global](#tag/global/) URIs, or are [local](#tag/local/) in scope to a
 single [application](#application//).
 For example, an integer is represented in YAML with a [scalar](#scalar//) plus the [global tag](#tag/global/) "**`tag:yaml.org,2002:int`**". Similarly, an invoice
 object, particular to a given organization, could be
 represented as a [mapping](#mapping//) together with the [local tag](#tag/local/) "**`!invoice`**". This simple model can represent any
 data structure independent of programming language.

Serializing the Representation Graph
For sequential access mediums, such as an event callback API, a
 YAML [representation](#representation//) must be *serialized* to an
 ordered tree. Since in a YAML [representation](#representation//), [mapping keys](#key//) are unordered and [nodes](#node//) may be referenced more than once
 (have more than one incoming "arrow"), the
 serialization process is required to impose an [ordering](#key/order/) on the [mapping keys](#key//) and to replace the
 second and subsequent references to a given [node](#node//) with place holders called [aliases](#alias//). YAML does not specify how
 these *serialization details* are chosen. It
 is up to the YAML [processor](#processor//) to come up with
 human-friendly [key order](#key/order/) and [anchor](#anchor//) names, possibly with the help of the [application](#application//). The result of this
 process, a YAML [serialization tree](#serialization//), can then be traversed to produce a series of
 event calls for one-pass processing of YAML data.

Presenting the Serialization Tree
The final output process is *presenting* the YAML [serializations](#serialization//) as a character [stream](#stream//) in a human-friendly
 manner. To maximize human readability, YAML offers a rich set of
 stylistic options which go far beyond the minimal functional
 needs of simple data storage. Therefore the YAML [processor](#processor//) is required to introduce
 various *presentation details* when creating
 the [stream](#stream//), such as the
 choice of [node styles](#style//), how to [format scalar content](<#scalar/content format/>), the amount of [indentation](#space/indentation/), which [tag handles](#tag/handle/) to use,
 the [node tags](#tag//) to leave [unspecified](#tag/non-specific/), the
 set of [directives](#directive//) to
 provide and possibly even what [comments](#comment//) to add. While some of this
 can be done with the help of the [application](#application//), in general this
 process should be guided by the preferences of the user.

### 3.1.2. Load

*Loading* [native data structures](<#native data structure//>) from a character [stream](#stream//) is done using the
 following three stages:

Parsing the Presentation Stream
*Parsing* is the inverse process
 of [presentation](#present//), it takes a [stream](#stream//) of characters and
 produces a series of events. Parsing discards all the [details](#presentation/detail/) introduced in the [presentation](#present//) process, reporting only
 the [serialization](#serialization//) events. Parsing can fail due to [ill-formed](#stream/ill-formed/) input.

Composing the Representation Graph
*Composing* takes a series of [serialization](#serialization//) events
 and produces a [representation graph](#representation//). Composing discards all the [details](#serialization/detail/) introduced in the [serialization](#serialize//) process, producing
 only the [representation graph](#representation//). Composing can fail due to any of several
 reasons, detailed [below](<#load/failure point/>).

Constructing Native Data Structures
The final input process is *constructing* [native data structures](<#native data structure//>) from the YAML [representation](#representation//). Construction
 must be based only on the information available in the [representation](#representation//), and not on
 additional [serialization](#serialization//) or [presentation details](#presentation/detail/) such as [comments](#comment//), [directives](#directive//), [mapping key order](#key/order/), [node styles](#style//), [scalar content format](<#scalar/content format/>), [indentation](#space/indentation/) levels etc.
 Construction can fail due to the [unavailability](#tag/unavailable/) of the required [native data types](<#native data structure//>).

## 3.2. Information Models

This section specifies the formal details of the results of the above
 processes. To maximize data portability between programming languages
 and implementations, users of YAML should be mindful of the distinction
 between [serialization](#serialization//) or [presentation](#presentation//) properties and
 those which are part of the YAML [representation](#representation//). Thus, while imposing
 a [order](#key/order/) on [mapping keys](#key//) is necessary for flattening YAML [representations](#representation//) to a
 sequential access medium, this [serialization detail](#serialization/detail/) must not be used to
 convey [application](#application//) level
 information. In a similar manner, while [indentation](#space/indentation/) technique and a choice of
 a [node style](#style//) are needed for the
 human readability, these [presentation details](#presentation/detail/) are neither part of
 the YAML [serialization](#serialization//) nor
 the YAML [representation](#representation//). By
 carefully separating properties needed for [serialization](#serialization//) and [presentation](#presentation//), YAML [representations](#representation//) of [application](#application//) information will be
 consistent and portable between various programming environments.

The following diagram summarizes the three *information models*. Full arrows
 denote composition, hollow arrows denote inheritance,
 "**`1`**" and "**`*`**" denote "one" and
 "many" relationships. A single "**`+`**" denotes [serialization](#serialization//) details, a
 double "**`++`**" denotes [presentation](#presentation//) details.

**Figure 3.2. Information Models**

![Information Models](model2.png)


### 3.2.1. Representation Graph

YAML's *representation* of [native data structure](<#native data structure//>) is a rooted, connected, directed graph of [tagged](#tag//) [nodes](#node//). By "directed graph" we
 mean a set of [nodes](#node//) and directed
 edges ("arrows"), where each edge connects one [node](#node//) to another (see [a formal definition](http://www.nist.gov/dads/HTML/directedGraph.html)). All the [nodes](#node//) must be reachable from the *root node* via such edges. Note that the
 YAML graph may include cycles, and a [node](#node//) may have more than one incoming edge.

[Nodes](#node//) that are defined in terms of
 other [nodes](#node//) are [collections](#collection//); [nodes](#node//) that are independent of any other [nodes](#node//) are [scalars](#scalar//). YAML supports two [kinds](#kind//) of [collection nodes](#collection//): [sequences](#sequence//) and [mappings](#mapping//). [Mapping nodes](#mapping//) are somewhat tricky because
 their [keys](#key//) are unordered and must be [unique](#equality//).

**Figure 3.3. Representation Model**

![Representation Model](represent2.png)


#### 3.2.1.1. Nodes

A YAML *node* [represents](#representation//) a single [native data structure](<#native data structure//>).
 Such nodes have *content* of one
 of three *kinds*: scalar, sequence,
 or mapping. In addition, each node has a [tag](#tag//) which serves to restrict the set of
 possible values the content can have.

Scalar
The content of a *scalar* node is an opaque datum that can be [presented](#present//) as a series of zero or
 more Unicode characters.

Sequence
The content of a *sequence* node is an ordered series of zero or more nodes. In particular,
 a sequence may contain the same node more than once. It could
 even contain itself (directly or indirectly).

Mapping
The content of a *mapping* node is an unordered set of *key:* *value* node *pairs*, with the restriction that each of
 the keys is [unique](#equality//). YAML
 places no further restrictions on the nodes. In particular,
 keys may be arbitrary nodes, the same node may be used as the
 value of several key: value pairs, and a mapping could
 even contain itself as a key or a value (directly or
 indirectly).

When appropriate, it is convenient to consider sequences and
 mappings together, as *collections*. In this view, sequences
 are treated as mappings with integer keys starting at zero. Having
 a unified collections view for sequences and mappings is helpful
 both for theoretical analysis and for creating practical YAML tools
 and APIs. This strategy is also used by the Javascript programming
 language.

#### 3.2.1.2. Tags

YAML [represents](#represent//) type
 information of [native data structures](<#native data structure//>) with a simple identifier, called a *tag*. *Global tags* are [URIs](http://www.ietf.org/rfc/rfc2396.txt) and hence
 globally unique across all [applications](#application//). The
 "**`tag:`**" [URI scheme](http://www.faqs.org/rfcs/rfc4151.html) is
 recommended for all global YAML tags. In contrast, *local tags* are specific
 to a single [application](#application//).
 Local tags start with *"**`!`**"*, are not URIs
 and are not expected to be globally unique. YAML provides a ["**`TAG`**" directive](#directive/TAG/) to make tag notation less verbose; it also
 offers easy migration from local to global tags. To ensure this,
 local tags are restricted to the URI character set and use URI
 character [escaping](<#% escaping in URI//>).

YAML does not mandate any special relationship between different
 tags that begin with the same substring. Tags ending with URI
 fragments (containing "**`#`**") are no exception; tags
 that share the same base URI but differ in their fragment part are
 considered to be different, independent tags. By convention,
 fragments are used to identify different "variants" of
 a tag, while "**`/`**" is used to define nested tag
 "namespace" hierarchies. However, this is merely a
 convention, and each tag may employ its own rules. For example,
 Perl tags may use "**`::`**" to express namespace
 hierarchies, Java tags may use "**`.`**", etc.

YAML tags are used to associate meta information with each [node](#node//). In particular, each tag must specify
 the expected [node kind](#kind//) ([scalar](#scalar//), [sequence](#sequence//), or [mapping](#mapping//)). [Scalar](#scalar//) tags must also provide a
 mechanism for converting [formatted content](<#scalar/content format/>) to a [canonical form](<#scalar/canonical form/>) for supporting [equality](#equality//) testing. Furthermore, a tag
 may provide additional information such as the set of allowed [content](#content//) values for validation,
 a mechanism for [tag resolution](#tag/resolution/), or any other data that is applicable to all
 of the tag's [nodes](#node//).

#### 3.2.1.3. Node Comparison

Since YAML [mappings](#mapping//) require [key](#key//) uniqueness, [representations](#representation//) must include a
 mechanism for testing the equality of [nodes](#node//). This is non-trivial since YAML
 allows various ways to [format scalar content](<#scalar/content format/>). For example, the integer
 eleven can be written as "**`0o13`**" (octal) or
 "**`0xB`**" (hexadecimal). If both notations are used as [keys](#key//) in the same [mapping](#mapping//), only a YAML [processor](#processor//) which recognizes integer [formats](<#scalar/content format/>) would correctly flag the duplicate [key](#key//) as an error.

Canonical Form
YAML supports the need for [scalar](#scalar//) equality by requiring that
 every [scalar](#scalar//) [tag](#tag//) must specify a mechanism for
 producing the *canonical form* of any [formatted content](<#scalar/content format/>). This
 form is a Unicode character string which also [presents](#present//) the same [content](<#scalar/content format/>),
 and can be used for equality testing. While this requirement is
 stronger than a well defined equality operator, it has other
 uses, such as the production of digital signatures.

Equality
Two [nodes](#node//) must have the
 same [tag](#tag//) and [content](#content//) to be *equal*. Since each [tag](#tag//) applies to exactly one [kind](#kind//), this implies that the two [nodes](#node//) must have the same [kind](#kind//) to be equal. Two [scalars](#scalar//) are equal only
 when their [tags](#tag//) and
 canonical forms are equal character-by-character. Equality
 of [collections](#collection//) is
 defined recursively. Two [sequences](#sequence//) are equal only when
 they have the same [tag](#tag//) and
 length, and each [node](#node//) in
 one [sequence](#sequence//) is equal
 to the corresponding [node](#node//) in the other [sequence](#sequence//).
 Two [mappings](#mapping//) are equal
 only when they have the same [tag](#tag//) and an equal set of [keys](#key//), and each [key](#key//) in this set is associated with
 equal [values](#value//) in both [mappings](#mapping//).

Different URI schemes may define different rules for testing
 the equality of URIs. Since a YAML [processor](#processor//) cannot be reasonably
 expected to be aware of them all, it must resort to a simple
 character-by-character comparison of [tags](#tag//) to ensure consistency. This also
 happens to be the comparison method defined by the
 "**`tag:`**" URI scheme. [Tags](#tag//) in a YAML stream must therefore
 be [presented](#present//) in a
 canonical way so that such comparison would yield the correct
 results.

Identity
Two [nodes](#node//) are *identical* only when they [represent](#represent//) the same [native data structure](<#native data structure//>). Typically, this corresponds to a single
 memory address. Identity should not be confused with equality;
 two equal [nodes](#node//) need not have
 the same identity. A YAML [processor](#processor//) may treat equal [scalars](#scalar//) as if they were
 identical. In contrast, the separate identity of two distinct
 but equal [collections](#collection//) must be preserved.

### 3.2.2. Serialization Tree

To express a YAML [representation](#representation//) using a serial API,
 it is necessary to impose an [order](#key/order/) on [mapping keys](#key//) and employ [alias nodes](#alias//) to indicate a subsequent occurrence of a previously
 encountered [node](#node//). The result of
 this process is a *serialization
 tree*, where each [node](#node//) has
 an ordered set of children. This tree can be traversed for a serial
 event-based API. [Construction](#construct//) of [native data structures](<#native data structure//>) from the serial interface should not use [key order](#key/order/) or [anchor names](#anchor//) for the preservation
 of [application](#application//) data.

**Figure 3.4. Serialization Model**

![Serialization Model](serialize2.png)


#### 3.2.2.1. Keys Order

In the [representation](#representation//) model, [mapping keys](#key//) do not have an
 order. To [serialize](#serialize//) a [mapping](#mapping//), it is necessary to
 impose an *ordering* on its [keys](#key//). This order is a [serialization detail](#serialization/detail/) and should not be
 used when [composing](#compose//) the [representation graph](#representation//) (and hence for the preservation of [application](#application//) data). In every case
 where [node](#node//) order is significant,
 a [sequence](#sequence//) must be used. For
 example, an ordered [mapping](#mapping//) can be [represented](#represent//) as a [sequence](#sequence//) of [mappings](#mapping//), where each [mapping](#mapping//) is a single [key: value pair](<#key: value pair//>). YAML
 provides convenient [compact notation](<#style/single key:value pair mapping/>) for
 this case.

#### 3.2.2.2. Anchors and Aliases

In the [representation graph](#representation//), a [node](#node//) may
 appear in more than one [collection](#collection//). When [serializing](#serialize//) such data, the first
 occurrence of the [node](#node//) is *identified* by an *anchor*. Each subsequent occurrence is [serialized](#serialize//) as an [alias node](#alias//) which refers back to this
 anchor. Otherwise, anchor names are a [serialization detail](#serialization/detail/) and are discarded once [composing](#compose//) is completed. When [composing](#compose//) a [representation graph](#representation//) from [serialized](#serialize//) events, an alias
 node refers to the most recent [node](#node//) in the [serialization](#serialization//) having the
 specified anchor. Therefore, anchors need not be unique within a [serialization](#serialization//)). In
 addition, an anchor need not have an alias node referring to it. It
 is therefore possible to provide an anchor for all [nodes](#node//) in [serialization](#serialization//)).

### 3.2.3. Presentation Stream

A YAML *presentation* is a [stream](#stream//) of Unicode characters
 making use of of [styles](#style//), [scalar content formats](<#scalar/content format/>), [comments](#comment//), [directives](#directive//) and other [presentation details](#presentation/detail/) to [present](#present//) a
 YAML [serialization](#serialization//) in a
 human readable way. Although a YAML [processor](#processor//) may provide these [details](#presentation/detail/) when [parsing](#parse//), they should not be
 reflected in the resulting [serialization](#serialization//). YAML allows several [serialization trees](#serialization//) to be
 contained in the same YAML character stream, as a series of [documents](#document//) separated by [markers](#marker//). Documents appearing in the same
 stream are independent; that is, a [node](#node//) must not appear in more than one [serialization tree](#serialization//) or [representation graph](#representation//).

**Figure 3.5. Presentation Model**

![Presentation Model](present2.png)


#### 3.2.3.1. Node Styles

Each [node](#node//) is presented in some *style*, depending on its [kind](#kind//). The node style is a [presentation detail](#presentation/detail/) and is not reflected in the [serialization tree](#serialization//) or [representation graph](#representation//). There are
 two groups of styles. [Block styles](#style/block/) use [indentation](#space/indentation/) to
 denote structure; In contrast, [flow styles](#style/flow/) styles rely on explicit [indicators](#indicator//).

YAML provides a rich set of *scalar styles*. [Block scalar](#style/block/scalar) styles include the [literal style](#style/block/literal) and
 the [folded style](#style/block/folded). [Flow scalar](#style/flow/scalar) styles
 include the [plain style](#style/flow/plain) and two quoted styles, the [single-quoted style](#style/flow/single-quoted) and the [double-quoted style](#style/flow/double-quoted). These
 styles offer a range of trade-offs between expressive power and
 readability.

Normally, [block sequences](#style/block/sequence) and [mappings](#style/block/mapping) begin on the next line. In
 some cases, YAML also allows nested [block](#style/block/) [collections](#collection//) to start in-line for a
 more [compact notation](<#style/compact block collection/>). In addition, YAML provides a [compact notation](<#style/single key:value pair mapping/>) for [flow mappings](#style/flow/mapping) with a single [key: value pair](<#key: value pair//>), nested
 inside a [flow sequence](#style/flow/sequence). These allow for a
 natural "ordered mapping" notation.

**Figure 3.6. Kind/Style Combinations**

![Kind/Style Combinations](styles2.png)


#### 3.2.3.2. Scalar Formats

YAML allows [scalars](#scalar//) to be [presented](#present//) in several *formats*. For
 example, the integer "**`11`**" might also be written as
 "**`0xB`**". [Tags](#tag//) must
 specify a mechanism for converting the formatted content to a [canonical form](<#scalar/canonical form/>) for use in [equality](#equality//) testing. Like [node style](#style//), the format is a [presentation detail](#presentation/detail/) and is not reflected in the [serialization tree](#serialization//) and [representation graph](#representation//).

#### 3.2.3.3. Comments

[Comments](#comment//) are a [presentation detail](#presentation/detail/) and must not have any effect on the [serialization tree](#serialization//) or [representation graph](#representation//). In
 particular, comments are not associated with a particular [node](#node//). The usual purpose of a comment is to
 communicate between the human maintainers of a file. A typical
 example is comments in a configuration file. Comments must not
 appear inside [scalars](#scalar//), but may
 be interleaved with such [scalars](#scalar//) inside [collections](#collection//).

#### 3.2.3.4. Directives

Each [document](#document//) may be
 associated with a set of [directives](#directive//). A directive has a name
 and an optional sequence of parameters. Directives are instructions
 to the YAML [processor](#processor//), and
 like all other [presentation details](#presentation/detail/) are not reflected
 in the YAML [serialization tree](#serialization//) or [representation graph](#representation//). This version of YAML defines a two directives, ["**`YAML`**"](#directive/YAML/) and ["**`TAG`**"](#directive/TAG/).
 All other directives are [reserved](#directive/reserved/) for future versions of
 YAML.

## 3.3. Loading Failure Points

The process of [loading](#load//) [native data structures](<#native data structure//>) from a
 YAML [stream](#stream//) has several potential *failure
 points*. The character [stream](#stream//) may be [ill-formed](#stream/ill-formed/), [aliases](#alias//) may be [unidentified](#alias/unidentified/), [unspecified tags](#tag/non-specific/) may be [unresolvable](#tag/unresolved/), [tags](#tag//) may be [unrecognized](#tag/unrecognized/), the [content](#content//) may be [invalid](<#invalid content//>), and a native type may be [unavailable](#tag/unavailable/). Each of
 these failures results with an incomplete loading.

A *partial
 representation* need not [resolve](#tag/resolution/) the [tag](#tag//) of each [node](#node//), and the [canonical form](<#scalar/canonical form/>) of [formatted scalar content](<#scalar/content format/>) need not be available. This
 weaker representation is useful for cases of incomplete knowledge of
 the types used in the [document](#document//)).
 In contrast, a *complete representation* specifies the [tag](#tag//) of each [node](#node//), and provides the [canonical form](<#scalar/canonical form/>) of [formatted scalar content](<#scalar/content format/>), allowing for [equality](#equality//) testing. A complete
 representation is required in order to [construct](#construct//) [native data structures](<#native data structure//>).

**Figure 3.7. Loading Failure Points**

![Loading Failure Points](validity2.png)


### 3.3.1. Well-Formed Streams and Identified Aliases

A [well-formed](#stream/well-formed/) character [stream](#stream//) must match the BNF productions
 specified in the following chapters. Successful loading also requires
 that each [alias](#alias//) shall refer to a
 previous [node](#node//) [identified](#alias/identified/) by the [anchor](#anchor//). A YAML [processor](#processor//) should reject *ill-formed streams* and *unidentified
 aliases*. A YAML [processor](#processor//) may recover from syntax
 errors, possibly by ignoring certain parts of the input, but it must
 provide a mechanism for reporting such errors.

### 3.3.2. Resolved Tags

Typically, most [tags](#tag//) are not
 explicitly specified in the character [stream](#stream//). During [parsing](#parse//), [nodes](#node//) lacking an explicit [tag](#tag//) are given a *non-specific tag*: *"**`!`**"* for non-[plain scalars](#style/flow/plain), and *"**`?`**"* for all other [nodes](#node//). [Composing](#compose//) a [complete representation](#representation/complete/) requires each such non-specific tag to be *resolved* to a *specific tag*,
 be it a [global tag](#tag/global/) or a [local tag](#tag/local/).

Resolving the [tag](#tag//) of a [node](#node//) must only depend on the following three
 parameters: (1) the non-specific tag of the [node](#node//), (2) the path leading from the [root](#node/root/) to the [node](#node//), and (3) the [content](#content//) (and hence the [kind](#kind//)) of the [node](#node//). When a [node](#node//) has more than one occurrence (using [aliases](#alias//)), tag resolution must
 depend only on the path to the first ([anchored](#anchor//)) occurrence of the [node](#node//)).

Note that resolution must not consider [presentation details](#presentation/detail/) such as [comments](#comment//), [indentation](#space/indentation/) and [node style](#style//). Also, resolution must not
 consider the [content](#content//) of any
 other [node](#node//), except for the [content](#content//) of the [key nodes](#key//) directly along the path leading from the [root](#node/root/) to the resolved [node](#node//). Finally, resolution must not
 consider the [content](#content//) of a
 sibling [node](#node//) in a [collection](#collection//), or the [content](#content//) of the [value node](#value//) associated with a [key node](#key//) being resolved.

These rules ensure that tag resolution can be performed as soon as a [node](#node//) is first encountered in the [stream](#stream//), typically before its [content](#content//) is [parsed](#parse//). Also, tag resolution only requires
 referring to a relatively small number of previously parsed [nodes](#node//). Thus, in most cases, tag resolution
 in one-pass [processors](#processor//) is both
 possible and practical.

YAML [processors](#processor//) should resolve [nodes](#node//) having the "**`!`**"
 non-specific tag as "**`tag:yaml.org,2002:seq`**",
 "**`tag:yaml.org,2002:map`**" or
 "**`tag:yaml.org,2002:str`**" depending on their [kind](#kind//). This *tag resolution
 convention* allows the author of a YAML character [stream](#stream//) to effectively
 "disable" the tag resolution process. By explicitly
 specifying a "**`!`**" non-specific [tag property](#tag/property/), the [node](#node//) would then be resolved to a
 "vanilla" [sequence](#sequence//), [mapping](#mapping//), or string, according to its [kind](#kind//).

[Application](#application//) specific tag
 resolution rules should be restricted to resolving the
 "**`?`**" non-specific tag, most commonly to resolving [plain scalars](#style/flow/plain). These may be matched against a set of regular
 expressions to provide automatic resolution of integers, floats,
 timestamps, and similar types. An [application](#application//) may also match the [content](#content//) of [mapping nodes](#mapping//) against sets of expected [keys](#key//) to automatically resolve
 points, complex numbers, and similar types. Resolved [sequence node](#sequence//) types such as the
 "ordered mapping" are also possible.

That said, tag resolution is specific to the [application](#application//). YAML [processors](#processor//) should therefore provide a
 mechanism allowing the [application](#application//) to override and expand
 these default tag resolution rules.

If a [document](#document//) contains *unresolved tags*, the
 YAML [processor](#processor//) is unable to [compose](#compose//) a [complete representation](#representation/complete/) graph. In such a case, the YAML [processor](#processor//) may [compose](#compose//) a [partial representation](#representation/partial/), based on each [node's kind](#kind//) and allowing for non-specific
 tags.

### 3.3.3. Recognized and Valid Tags

To be *valid*, a [node](#node//) must have a [tag](#tag//) which is *recognized* by the YAML [processor](#processor//) and its [content](#content//) must satisfy the constraints
 imposed by this [tag](#tag//). If a [document](#document//) contains a [scalar node](#scalar//) with an *unrecognized tag* or *invalid content*, only a [partial representation](#representation/partial/) may be [composed](#compose//). In contrast, a YAML [processor](#processor//) can always [compose](#compose//) a [complete representation](#representation/complete/) for an unrecognized or an invalid [collection](#collection//), since [collection](#collection//) [equality](#equality//) does not depend upon knowledge
 of the [collection's](#collection//) data
 type. However, such a [complete representation](#representation/complete/) cannot be
 used to [construct](#construct//) a [native data structure](<#native data structure//>).

### 3.3.4. Available Tags

In a given processing environment, there need not be an *available* native type
 corresponding to a given [tag](#tag//). If a [node's tag](#tag//) is *unavailable*, a YAML [processor](#processor//) will not be able to [construct](#construct//) a [native data structure](<#native data structure//>) for
 it. In this case, a [complete representation](#representation/complete/) may still be [composed](#compose//), and an [application](#application//) may wish to use this [representation](#representation//) directly.

## Chapter 4. Syntax Conventions

The following chapters formally define the syntax of YAML character [streams](#stream//), using parameterized BNF
 productions. Each BNF production is both named and numbered for easy
 reference. Whenever possible, basic structures are specified before the
 more complex structures using them in a "bottom up" fashion.

The order of alternatives inside a production is significant. Subsequent
 alternatives are only considered when previous ones fails. See for
 example the [**`b-break`**](#b-break) production.
 In addition, production matching is expected to be greedy. Optional
 (**`?`**), zero-or-more (**`*`**) and
 one-or-more (**`+`**) patterns are always expected to
 match as much of the input as possible.

The productions are accompanied by examples, which are given side-by-side
 next to equivalent YAML text in an explanatory format. This format uses
 only [flow collections](#style/flow/collection), [double-quoted scalars](#style/flow/double-quoted), and explicit [tags](#tag//) for each [node](#node//).

A reference implementation using the productions is available as the [YamlReference](http://hackage.haskell.org/cgi-bin/hackage-scripts/package/YamlReference) Haskell package. This reference implementation is
 also available as an interactive web application at [http://dev.yaml.org/ypaste](http://dev.yaml.org/ypaste).

## 4.1. Production Parameters

YAML's syntax is designed for maximal human readability. This
 requires [parsing](#parse//) to depend on the
 surrounding text. For notational compactness, this dependency is
 expressed using parameterized BNF productions.

This sensitivity is the cause of most of the complexity of the YAML
 syntax definition. It is further complicated by struggling with the
 human tendency to look ahead when interpreting text. These
 complications are of course the source of most of YAML's power to [present](#presentation//) data in a very human
 readable way.

Productions use any of the following parameters:

Indentation: `n` or `m`
Many productions use an explicit [indentation](#space/indentation/) level parameter. This
 is less elegant than Python's "indent" and
 "undent" conceptual tokens. However it is required to
 formally express YAML's indentation rules.

Context: `c`
This parameter allows productions to tweak their behavior
 according to their surrounding. YAML supports two groups of *contexts*, distinguishing
 between [block styles](#style/block/) and [flow styles](#style/flow/).

In [block styles](#style/block/), [indentation](#space/indentation/) is used to
 delineate structure. To capture human perception of [indentation](#space/indentation/) the
 rules require special treatment of the ["**`-`**"](<#- block sequence entry//>) character, used in [block sequences](#style/block/sequence). Hence in some
 cases productions need to behave differently inside [block sequences](#style/block/sequence) (*block-in context*) and outside them
 (*block-out
 context*).

In [flow styles](#style/flow/), explicit [indicators](#indicator//) are used to delineate
 structure. These styles can be viewed as the natural extension of
 JSON to cover [tagged](#tag//), [single-quoted](#style/flow/single-quoted) and [plain scalars](#style/flow/plain). Since the latter have no delineating [indicators](#indicator//), they are subject to
 some restrictions to avoid ambiguities. These restrictions depend
 on where they appear: as implicit keys directly inside a [block mapping](#style/block/mapping) (*block-key*); as implicit keys
 inside a [flow mapping](#style/flow/mapping) (*flow-key*); as
 values inside a [flow collection](#style/flow/collection) (*flow-in*); or as
 values inside one (*flow-out*).

(Block) Chomping: `t`
Block scalars offer three possible mechanisms for [chomping](#chomping//) any trailing [line breaks](<#line break//>): [strip](#chomping/strip/), [clip](#chomping/clip/) and [keep](#chomping/keep/). Unlike the
 previous parameters, this only controls interpretation; the [line breaks](<#line break//>) are valid in
 either case.

## 4.2. Production Naming Conventions

To make it easier to follow production combinations, production names
 use a Hungarian-style naming convention. Each production is given a
 prefix based on the type of characters it begins and ends with.

**`e-`**
A production matching no characters.

**`c-`**
A production starting and ending with a special character.

**`b-`**
A production matching a single [line break](<#line break//>).

**`nb-`**
A production starting and ending with a non-[break](<#line break//>) character.

**`s-`**
A production starting and ending with a [white space](#space/white/) character.

**`ns-`**
A production starting and ending with a non-[space](#space/white/) character.

**`l-`**
A production matching complete line(s).

`X`**`-`**`Y`**`-`**
A production starting with an `X`**`-`** character and ending
 with a `Y`**`-`** character,
 where `X`**`-`** and `Y`**`-`** are any of the above
 prefixes.

`X`**`+`**, `X`**`-`**`Y`**`+`**
A production as above, with the additional property that the
 matched content [indentation](#space/indentation/) level is greater than
 the specified `n` parameter.

## Chapter 5. Characters

## 5.1. Character Set

To ensure readability, YAML [streams](#stream//) use only the *printable* subset of the Unicode character set. The allowed character range
 explicitly excludes the C0 control block **`#x0-#x1F`** (except for TAB **`#x9`**, LF **`#xA`**, and CR **`#xD`** which are allowed), DEL **`#x7F`**, the C1 control block **`#x80-#x9F`** (except for NEL **`#x85`** which is allowed), the surrogate block **`#xD800-#xDFFF`**, **`#xFFFE`**,
 and **`#xFFFF`**.

On input, a YAML [processor](#processor//) must
 accept all Unicode characters except those explicitly excluded above.

On output, a YAML [processor](#processor//) must
 only produce acceptable characters. Any excluded characters must be [presented](#present//) using [escape](<#escaping/in double-quoted scalars/>) sequences. In addition, any allowed
 characters known to be non-printable should also be [escaped](<#escaping/in double-quoted scalars/>). This isn't mandatory since a full
 implementation would require extensive character property tables.

- | [1] | c-printable | `::=` | #x9 \| #xA \| #xD \| [#x20-#x7E] /* 8 bit */ \| #x85 \| [#xA0-#xD7FF] \| [#xE000-#xFFFD] /* 16 bit */ \| [#x10000-#x10FFFF] /* 32 bit */ | |
| --- | --- | --- | --- | --- |


To ensure [JSON compatibility](<#JSON compatibility//>), YAML [processors](#processor//) must allow all non-control
 characters inside [quoted scalars](#style/flow/double-quoted). To ensure
 readability, non-printable characters should be [escaped](<#escaping/in double-quoted scalars/>) on output, even inside such [scalars](#style/flow/double-quoted). Note that JSON [quoted scalars](#style/flow/double-quoted) cannot span multiple lines or contain [tabs](#tab//), but YAML [quoted scalars](#style/flow/double-quoted) can.

- | [2] | nb-json | `::=` | #x9 \| [#x20-#x10FFFF] | |
| --- | --- | --- | --- | --- |

## 5.2. Character Encodings

All characters mentioned in this specification are Unicode code points.
 Each such code point is written as one or more bytes depending on the *character encoding* used. Note that in UTF-16, characters above **`#xFFFF`** are written as four bytes, using a
 surrogate pair.

The character encoding is a [presentation detail](#presentation/detail/) and must not be used
 to convey [content](#content//) information.

On input, a YAML [processor](#processor//) must
 support the UTF-8 and UTF-16 character encodings. For [JSON compatibility](<#JSON compatibility//>), the UTF-32
 encodings must also be supported.

If a character [stream](#stream//) begins with a *byte order mark*, the
 character encoding will be taken to be as as indicated by the byte
 order mark. Otherwise, the [stream](#stream//) must begin with an ASCII character. This allows the encoding to be
 deduced by the pattern of null (**`#x00`**)
 characters.

To make it easier to concatenate [streams](#stream//), byte order marks may appear at the
 start of any [document](#document//). However
 all [documents](#document//) in the same [stream](#stream//) must use the same character
 encoding.

To allow for [JSON compatibility](<#JSON compatibility//>), byte order marks are also allowed inside [quoted scalars](#style/flow/double-quoted). For readability,
 such [content](#content//) byte order marks
 should be [escaped](<#escaping/in double-quoted scalars/>) on output.

The encoding can therefore be deduced by matching the first few bytes
 of the [stream](#stream//) with the following
 table rows (in order):

|  | **Byte0** | **Byte1** | **Byte2** | **Byte3** | **Encoding** |
| --- | --- | --- | --- | --- | --- |
| *Explicit BOM* | **`#x00`** | **`#x00`** | **`#xFE`** | **`#xFF`** | UTF-32BE |
| *ASCII first character* | **`#x00`** | **`#x00`** | **`#x00`** | *any* | UTF-32BE |
| *Explicit BOM* | **`#xFF`** | **`#xFE`** | **`#x00`** | **`#x00`** | UTF-32LE |
| *ASCII first character* | *any* | **`#x00`** | **`#x00`** | **`#x00`** | UTF-32LE |
| *Explicit BOM* | **`#xFE`** | **`#xFF`** |  |  | UTF-16BE |
| *ASCII first character* | **`#x00`** | *any* |  |  | UTF-16BE |
| *Explicit BOM* | **`#xFF`** | **`#xFE`** |  |  | UTF-16LE |
| *ASCII first character* | *any* | **`#x00`** |  |  | UTF-16LE |
| *Explicit BOM* | **`#xEF`** | **`#xBB`** | **`#xBF`** |  | UTF-8 |
| *Default* |  |  |  |  | UTF-8 |

The recommended output encoding is UTF-8. If another encoding is used,
 it is recommended that an explicit byte order mark be used, even if the
 first [stream](#stream//) character is ASCII.

For more information about the byte order mark and the Unicode
 character encoding schemes see the [Unicode FAQ](http://www.unicode.org/unicode/faq/utf_bom.html).

- | [3] | c-byte-order-mark | `::=` | #xFEFF | |
| --- | --- | --- | --- | --- |


In the examples, byte order mark characters are displayed as
 "**`⇔`**".

**Example 5.1. Byte Order Mark**

| ``` ⇔# Comment only.    ```  ``` Legend:   [c-byte-order-mark](#c-byte-order-mark)  ``` | ``` # This stream contains no   # documents, only comments.  ``` |
| --- | --- |


**Example 5.2. Invalid Byte Order Mark**

| ``` - Invalid use of BOM   ⇔ - Inside a document.  ``` | ``` ERROR:    A BOM must not appear  inside a document.  ``` |
| --- | --- |


## 5.3. Indicator Characters

*Indicators* are characters that
 have special semantics.

- | [4] | c-sequence-entry | `::=` | "-" | |
| --- | --- | --- | --- | --- | A ["**`-`**"](<#- block sequence entry//>) (**`#2D`**,
 hyphen) denotes a [block sequence](#style/block/sequence) entry.

- | [5] | c-mapping-key | `::=` | "?" | |
| --- | --- | --- | --- | --- | A ["**`?`**"](<#? mapping key//>) (**`#3F`**, question mark) denotes a [mapping key](#key//).

- | [6] | c-mapping-value | `::=` | ":" | |
| --- | --- | --- | --- | --- | A ["**`:`**"](<#: mapping value//>) (**`#3A`**, colon) denotes a [mapping value](#value//).


**Example 5.3. Block Structure Indicators**

- ```
sequence:
- one
- two
mapping:
 ? sky
 : blue
 sea : green

```
```
Legend:
 [c-sequence-entry](#c-sequence-entry)
 [c-mapping-key](#c-mapping-key) [c-mapping-value](#c-mapping-value)

``` ```
%YAML 1.2
---
!!map {
 ? !!str "sequence"
 : !!seq [ !!str "one", !!str "two" ],
 ? !!str "mapping"
 : !!map {
 ? !!str "sky" : !!str "blue",
 ? !!str "sea" : !!str "green",
 },
}

```


- | [7] | c-collect-entry | `::=` | "," | |
| --- | --- | --- | --- | --- | A ["**`,`**"](<#, end flow entry//>) (**`#2C`**, comma) ends a [flow collection](#style/flow/collection) entry.

- | [8] | c-sequence-start | `::=` | "[" | |
| --- | --- | --- | --- | --- | A ["**`[`**"](<#[ start flow sequence//>) (**`#5B`**,
 left bracket) starts a [flow sequence](#style/flow/sequence).

- | [9] | c-sequence-end | `::=` | "]" | |
| --- | --- | --- | --- | --- | A ["**`\]`**"](<#] end flow sequence//>) (**`#5D`**, right bracket) ends a [flow sequence](#style/flow/sequence).

- | [10] | c-mapping-start | `::=` | "{" | |
| --- | --- | --- | --- | --- | A ["**`{`**"](<#{ start flow mapping//>) (**`#7B`**,
 left brace) starts a [flow mapping](#style/flow/mapping).

- | [11] | c-mapping-end | `::=` | "}" | |
| --- | --- | --- | --- | --- | A ["**`}`**"](<#} end flow mapping//>) (**`#7D`**, right brace) ends a [flow mapping](#style/flow/mapping).


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
%YAML 1.2
---
!!map {
 ? !!str "sequence"
 : !!seq [ !!str "one", !!str "two" ],
 ? !!str "mapping"
 : !!map {
 ? !!str "sky" : !!str "blue",
 ? !!str "sea" : !!str "green",
 },
}

```


- | [12] | c-comment | `::=` | "#" | |
| --- | --- | --- | --- | --- | An ["**`#`**"](<## comment//>) (**`#23`**, octothorpe, hash, sharp, pound, number
 sign) denotes a [comment](#comment//).


**Example 5.5. Comment Indicator**

| ``` # Comment only.    ```  ``` Legend:   [c-comment](#c-comment)  ``` | ``` # This stream contains no   # documents, only comments.  ``` |
| --- | --- |


- | [13] | c-anchor | `::=` | "&" | |
| --- | --- | --- | --- | --- | An ["**`&`**"](<#& anchor//>) (**`#26`**, ampersand) denotes a [node's anchor property](#anchor//).

- | [14] | c-alias | `::=` | "*" | |
| --- | --- | --- | --- | --- | An ["**`*`**"](<#* alias//>) (**`#2A`**, asterisk) denotes an [alias node](#alias//).

- | [15] | c-tag | `::=` | "!" | |
| --- | --- | --- | --- | --- | The ["**`!`**"](<#! tag indicator//>) (**`#21`**, exclamation) is heavily overloaded for
 specifying [node tags](#tag//). It is used to
 denote [tag handles](#tag/handle/) used in [tag directives](#directive/TAG/) and [tag properties](#tag/property/); to denote [local tags](#tag/local/); and as the [non-specific tag](#tag/non-specific/) for non-[plain scalars](#style/flow/plain).


**Example 5.6. Node Property Indicators**

- ```
anchored: !local &anchor value
alias: *anchor

```
```
Legend:
 [c-tag](#c-tag) [c-anchor](#c-anchor) [c-alias](#c-alias)

``` ```
%YAML 1.2
---
!!map {
 ? !!str "anchored"
 : !local &A1 "value",
 ? !!str "alias"
 : *A1,
}

```


- | [16] | c-literal | `::=` | "\|" | |
| --- | --- | --- | --- | --- | A ["**`|`**"](<#| literal style//>) (**`7C`**, vertical bar) denotes a [literal block scalar](#style/block/literal).

- | [17] | c-folded | `::=` | ">" | |
| --- | --- | --- | --- | --- | A ["**`>`**"](<#> folded style//>) (**`#3E`**,
 greater than) denotes a [folded block scalar](#style/block/folded).


**Example 5.7. Block Scalar Indicators**

| ``` literal: |     some   text folded: >   some   text  ```  ``` Legend:   [c-literal](#c-literal) [c-folded](#c-folded)  ``` | ``` %YAML 1.2   --- !!map {   ? !!str "literal"   : !!str "some\ntext\n",   ? !!str "folded"   : !!str "some text\n", }  ``` |
| --- | --- |


- | [18] | c-single-quote | `::=` | "'" | |
| --- | --- | --- | --- | --- | An ["**`'`**"](<#' single-quoted style//>) (**`#27`**,
 apostrophe, single quote) surrounds a [single-quoted flow scalar](#style/flow/single-quoted).

- | [19] | c-double-quote | `::=` | """ | |
| --- | --- | --- | --- | --- | A ["**`"`**"](<#" double-quoted style//>) (**`#22`**,
 double quote) surrounds a [double-quoted flow scalar](#style/flow/double-quoted).


**Example 5.8. Quoted Scalar Indicators**

| ``` single: 'text'   double: "text"  ```  ``` Legend:   [c-single-quote](#c-single-quote) [c-double-quote](#c-double-quote)  ``` | ``` %YAML 1.2   --- !!map {   ? !!str "single"   : !!str "text",   ? !!str "double"   : !!str "text", }  ``` |
| --- | --- |


- | [20] | c-directive | `::=` | "%" | |
| --- | --- | --- | --- | --- | A ["**`%`**"](<#% directive//>) (**`#25`**, percent) denotes a [directive](#directive//) line.


**Example 5.9. Directive Indicator**

| ``` %YAML 1.2   --- text  ```  ``` Legend:   [c-directive](#c-directive)  ``` | ``` %YAML 1.2   --- !!str "text"  ``` |
| --- | --- |


- | [21] | c-reserved | `::=` | "@

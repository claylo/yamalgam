# YAML Ain't Markup Language (YAML™) Version 1.2

## Revision 1.2.1 (2009-10-01)

### Oren Ben-Kiki

`<[oren@ben-kiki.org](mailto:oren@ben-kiki.org)>`

### Clark Evans

`<[cce@clarkevans.com](mailto:cce@clarkevans.com)>`

### Ingy döt Net

`<[ingy@ingy.net](mailto:ingy@ingy.net)>`

**Status of this Document**

This document reflects the third version of YAML data serialization
 language. The content of the specification was arrived at by consensus
 of its authors and through user feedback on the yaml-core mailing list. We encourage implementers to please update their software
 with support for this version.

The primary objective of this revision is to bring YAML into compliance
 with JSON as an official subset. YAML 1.2 is compatible with 1.1 for
 most practical applications - this is a minor revision. An expected
 source of incompatibility with prior versions of YAML, especially the
 syck implementation, is the change in implicit typing rules. We have
 removed unique implicit typing rules and have updated these rules to
 align them with JSON's productions. In this version of YAML, boolean
 values may be serialized as "**`true`**" or
 "**`false`**"; the empty scalar as "**`null`**".
 Unquoted numeric values are a superset of JSON's numeric production.
 Other changes in the specification were the removal of the Unicode line
 breaks and production bug fixes. We also define 3 built-in implicit
 typing rule sets: untyped, strict JSON, and a more flexible YAML rule
 set that extends JSON typing.

The difference between late 1.0 drafts which syck 0.55 implements and
 the 1.1 revision of this specification is much more extensive. We fixed
 usability issues with the tagging syntax. In particular, the single
 exclamation was re-defined for private types and a simple prefixing
 mechanism was introduced. This revision also fixed many production edge
 cases and introduced a type repository. Therefore, there are several
 incompatibilities between syck and this revision as well.

Please report errors in this document to the yaml-core mailing list.

We wish to thank implementers who have tirelessly tracked earlier
 versions of this specification, and our fabulous user community whose
 feedback has both validated and clarified our direction.

**Abstract**

YAML™ (rhymes with "camel") is a
 human-friendly, cross language, Unicode based data serialization
 language designed around the common native data types of agile
 programming languages. It is broadly useful for programming needs
 ranging from configuration files to Internet messaging to object
 persistence to data auditing. Together with the [Unicode standard for characters](http://www.unicode.org/),
 this specification provides all the information necessary to understand
 YAML Version 1.2 and to create programs that process YAML information.

---

## Chapter 1. Introduction

"YAML Ain't Markup Language" (abbreviated YAML) is a data
 serialization language designed to be human-friendly and work well with
 modern programming languages for common everyday tasks. This
 specification is both an introduction to the YAML language and the
 concepts supporting it, and also a complete specification of the
 information needed to develop applications for processing YAML.

Open, interoperable and readily understandable tools have advanced
 computing immensely. YAML was designed from the start to be useful and
 friendly to people working with data. It uses Unicode printable characters, some of which provide structural
 information and the rest containing the data itself. YAML achieves a
 unique cleanness by minimizing the amount of structural characters and
 allowing the data to show itself in a natural and meaningful way. For
 example, indentation may be used for structure, colons separate key: value pairs, and dashes are used to create
 "bullet" lists.

There are myriad flavors of data structures, but they can all be adequately represented with three basic primitives: mappings (hashes/dictionaries), sequences (arrays/lists) and scalars (strings/numbers). YAML
 leverages these primitives, and adds a simple typing system and aliasing mechanism to form a complete language
 for serializing any native data structure. While
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
2. YAML data is portable between programming languages.
3. YAML matches the native data structures of agile languages.
4. YAML has a consistent model to support generic tools.
5. YAML supports one-pass processing.
6. YAML is expressive and extensible.
7. YAML is easy to implement and use.

## 1.2. Prior Art

YAML's initial direction was set by the data serialization and
 markup language discussions among SML-DEV members. Later
 on, it directly incorporated experience from Ingy döt Net's
 Perl module Data::Denter. Since then, YAML has matured through ideas and
 support from its user community.

YAML integrates and builds upon concepts described by C, Java, Perl, Python, Ruby, RFC0822 (MAIL), RFC1866 (HTML), RFC2045 (MIME), RFC2396 (URI), XML, SAX, SOAP, and JSON.

The syntax of YAML was motivated by Internet Mail (RFC0822) and remains
 partially compatible with that standard. Further, borrowing from MIME
 (RFC2045), YAML's top-level production is a stream of independent documents, ideal for message-based
 distributed processing systems.

YAML's indentation-based scoping is similar
 to Python's (without the ambiguities caused by tabs). Indented blocks facilitate easy inspection
 of the data's structure. YAML's literal style leverages
 this by enabling formatted text to be cleanly mixed within an indented structure
 without troublesome escaping. YAML also allows the use of
 traditional indicator-based
 scoping similar to JSON's and Perl's. Such flow content can be freely
 nested inside indented blocks.

YAML's double-quoted style uses familiar
 C-style escape sequences. This enables ASCII encoding of
 non-printable or 8-bit
 (ISO 8859-1) characters such as "\x3B". Non-printable 16-bit Unicode and
 32-bit (ISO/IEC 10646) characters are supported with escape sequences such as "\u003B" and "\U0000003B".

Motivated by HTML's end-of-line normalization, YAML's line folding employs an intuitive
 method of handling line breaks. A single line break is folded into a single space, while empty lines are interpreted as line break characters. This technique allows for
 paragraphs to be word-wrapped without affecting the canonical form of
 the scalar content.

YAML's core type system is based on the requirements of agile
 languages such as Perl, Python, and Ruby. YAML directly supports both collections (mappings, sequences) and scalars. Support for these common types
 enables programmers to use their language's native data structures for YAML manipulation,
 instead of requiring a special document object model (DOM).

Like XML's SOAP, YAML supports serializing a graph of native data structures through an aliasing mechanism. Also
 like SOAP, YAML provides for application-defined types. This allows YAML to represent rich data structures required
 for modern distributed computing. YAML provides globally unique type names using a
 namespace mechanism inspired by Java's DNS-based package naming
 convention and XML's URI-based namespaces. In addition, YAML allows
 for private types specific to a single application.

YAML was designed to support incremental interfaces that include both
 input ("getNextEvent()") and output ("sendNextEvent()") one-pass interfaces. Together, these
 enable YAML to support the processing of large documents (e.g. transaction logs) or
 continuous streams (e.g. feeds from
 a production machine).

## 1.3. Relation to JSON

Both JSON and YAML aim to be human readable data interchange formats.
 However, JSON and YAML have different priorities. JSON's foremost
 design goal is simplicity and universality. Thus, JSON is trivial to
 generate and parse, at the cost of reduced human readability. It also
 uses a lowest common denominator information model, ensuring any JSON
 data can be easily processed by every modern programming environment.

In contrast, YAML's foremost design goals are human readability and
 support for serializing arbitrary native data structures. Thus, YAML allows for extremely readable files,
 but is more complex to generate and parse. In addition, YAML ventures
 beyond the lowest common denominator data types, requiring more complex
 processing when crossing between different programming environments.

YAML can therefore be viewed as a natural superset of JSON, offering
 improved human readability and a more complete information model. This
 is also the case in practice; every JSON file is also a valid YAML
 file. This makes it easy to migrate from JSON to YAML if/when the
 additional features are required.

JSON's RFC4627 requires that mapping keys merely "SHOULD" be unique, while YAML insists they
 "MUST" be. Technically, YAML therefore complies with the
 JSON spec, choosing to treat duplicates as an error. In practice, since
 JSON is silent on the semantics of such duplicates, the only portable
 JSON files are those with unique keys, which are therefore valid YAML
 files.

It may be useful to define a intermediate format between YAML and JSON.
 Such a format would be trivial to parse (but not very human readable),
 like JSON. At the same time, it would allow for serializing arbitrary native data structures, like
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
 please visit https://yaml.org/xml.

## 1.5. Terminology

This specification uses key words based on RFC2119 to indicate
 requirement level. In particular, the following words are used to
 describe the actions of a YAML processor:

**May**
The word *may*, or the adjective *optional*, mean that conforming YAML processors are permitted to, but *need not* behave as described.

**Should**
The word *should*, or the adjective *recommended*, mean that there could be reasons
 for a YAML processor to
 deviate from the behavior described, but that such deviation could
 hurt interoperability and should therefore be advertised with
 appropriate notice.

**Must**
The word *must*, or the term *required* or *shall*, mean that the behavior described
 is an absolute requirement of the specification.

## Chapter 2. Preview

This section provides a quick glimpse into the expressive power of YAML.
 It is not expected that the first-time reader grok all of the examples.
 Rather, these selections are used as motivation for the remainder of the
 specification.

## 2.1. Collections

YAML's block collections use indentation for scope
 and begin each entry on its own line. Block sequences indicate each entry with a dash and space ( "- "). Mappings use a colon and
 space ("`: `") to mark each key: value pair. Comments begin with an octothorpe (also
 called a "hash", "sharp",
 "pound", or "number sign" - "#").

**Example 2.1. Sequence of Scalars (ball players)**
```
- Mark McGwire
- Sammy Sosa
- Ken Griffey
```

**Example 2.2. Mapping Scalars to Scalars (player statistics)**
```
hr: 65 # Home runs
avg: 0.278 # Batting average
rbi: 147 # Runs Batted In
```

**Example 2.3. Mapping Scalars to Sequences (ball clubs in each league)**
```
american:
 - Boston Red Sox
 - Detroit Tigers
 - New York Yankees
national:
 - New York Mets
 - Chicago Cubs
 - Atlanta Braves
```

**Example 2.4. Sequence of Mappings (players' statistics)**
```
-
 name: Mark McGwire
 hr: 65
 avg: 0.278
-
 name: Sammy Sosa
 hr: 63
 avg: 0.288
```

YAML also has flow styles, using explicit indicators rather than indentation to denote
 scope. The flow sequence is written as a comma separated list within square brackets. In a similar manner,
 the flow mapping uses curly braces.

**Example 2.5. Sequence of Sequences**
```
- [name        , hr, avg  ]
- [Mark McGwire, 65, 0.278]
- [Sammy Sosa  , 63, 0.288]
```

**Example 2.6. Mapping of Mappings**
```
Mark McGwire: {hr: 65, avg: 0.278}
Sammy Sosa: {
    hr: 63,
    avg: 0.288
}
```

## 2.2. Structures

YAML uses three dashes ("---") to separate directives from document content. This also serves to signal the
 start of a document if no directives are present. Three dots ("...") indicate the end of a document
 without starting a new one, for use in communication channels.

**Example 2.7. Two Documents in a Stream (each with a leading comment)**
```
# Ranking of 1998 home runs
---
- Mark McGwire
- Sammy Sosa
- Ken Griffey

# Team ranking
---
- Chicago Cubs
- St Louis Cardinals
```

**Example 2.8. Play by Play Feed from a Game**
```
---
  time: 20:03:20
  player: Sammy Sosa
  action: strike (miss)
...
---
  time: 20:03:47
  player: Sammy Sosa
  action: grand slam
...
```

Repeated nodes (objects) are first identified by an anchor (marked with the
 ampersand - "&"), and are then aliased (referenced with an
 asterisk - "*") thereafter.

**Example 2.9. Single Document with Two Comments**
```
---
  hr: # 1998 hr ranking
  - Mark McGwire
  - Sammy Sosa
  rbi:
  # 1998 rbi ranking
  - Sammy Sosa
  - Ken Griffey
```

**Example 2.10. Node for "Sammy Sosa" appears twice in this document**
```
---
  hr:
  - Mark McGwire
  # Following node labeled SS
  - &SS Sammy Sosa
  rbi:
  - *SS # Subsequent occurrence
  - Ken Griffey
```

A question mark and space ("? ") indicate a complex mapping key. Within a block collection, key: value pairs can
 start immediately following the dash, colon, or question mark.

**Example 2.11. Mapping between Sequences**
```
? - Detroit Tigers
  - Chicago cubs
:
  - 2001-07-23

? [ New York Yankees,
    Atlanta Braves ]
: [ 2001-07-02, 2001-08-12,
    2001-08-14 ]
```

**Example 2.12. Compact Nested Mapping**
```
---
  # Products purchased
- item    : Super Hoop
  quantity: 1
- item    : Basketball
  quantity: 4
- item    : Big Shoes
  quantity: 1
```

## 2.3. Scalars

Scalar content can be written in block notation,
 using a literal style (indicated by "|") where all line breaks are significant.
 Alternatively, they can be written with the folded style (denoted by ">") where each line break is folded to a space unless it ends an empty or a
 more-indented line.

**Example 2.13. In literals, newlines are preserved**
```
# ASCII Art
---
|
 \//||\/||
 // || ||__
```

**Example 2.14. In the folded scalars, newlines become spaces**
```
---
>
 Mark McGwire's
 year was crippled
 by a knee injury.
```

**Example 2.15. Folded newlines are preserved for "more indented" and blank lines**
```
>
 Sammy Sosa completed another
 fine season with great stats.

 63 Home Runs
 0.288 Batting Average

 What a year!
```

**Example 2.16. Indentation determines scope**
```
name: Mark McGwire
accomplishment: >
 Mark set a major league
 home run record in 1998.
stats: |
 65 Home Runs
 0.278 Batting Average
```

YAML's flow scalars include the plain style (most examples thus far) and two quoted styles. The double-quoted style provides escape sequences. The single-quoted style is useful when escaping is not needed. All flow scalars can span multiple lines; line breaks are always folded.

**Example 2.17. Quoted Scalars**
```
unicode: "Sosa did fine.\u263A"
control: "\b1998\t1999\t2000\n"
hex esc: "\x0d\x0a is \r\n"
single: '"Howdy!" he cried.'
quoted: ' # Not a ''comment''.'
tie-fighter: '|\-*-/|'
```

**Example 2.18. Multi-line Flow Scalars**
```
plain:
  This unquoted scalar
  spans many lines.

quoted: "So does this
quoted scalar.\n"
```

## 2.4. Tags

In YAML, untagged nodes are given a type depending on the application. The examples in this
 specification generally use the `seq`, `map` and `str` types from the fail safe schema. A few
 examples also use the `int`, `float`, and `null` types from the JSON schema. The repository includes additional types such as `binary`, `omap`, `set` and others.

**Example 2.19. Integers**
```
canonical: 12345
decimal: +12345
octal: 0o14
hexadecimal: 0xC
```

**Example 2.20. Floating Point**
```
canonical: 1.23015e+3
exponential: 12.3015e+02
fixed: 1230.15
negative infinity: -.inf
not a number: .NaN
```

**Example 2.21. Miscellaneous**
```
null:
booleans: [ true, false ]
string: '012345'
```

**Example 2.22. Timestamps**
```
canonical: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14
```

Explicit typing is denoted with a tag using the exclamation point ("!") symbol. Global tags are URIs and may be specified
 in a tag shorthand notation using a handle. Application-specific local tags may also be used.

**Example 2.23. Various Explicit Tags**
```
---
  not-date: !!str 2002-04-28

  picture: !!binary |
   R0lGODlhDAAMAIQAAP//9/X
   17unp5WZmZgAAAOfn515eXv
   Pz7Y6OjuDg4J+fn5OTk6enp
   56enmleECcgggoBADs=

  application specific tag: !something |
   The semantics of the tag
   above may be different for
   different documents.
```

**Example 2.24. Global Tags**
```
%TAG ! tag:clarkevans.com,2002:
---
!shape
  # Use the ! handle for presenting
  # tag:clarkevans.com,2002:circle
- !circle
  center: &ORIGIN {x: 73, y: 129}
  radius: 7
- !line
  start: *ORIGIN
  finish: { x: 89, y: 102 }
- !label
  start: *ORIGIN
  color: 0xFFEEBB
  text: Pretty vector drawing.
```

**Example 2.25. Unordered Sets**
```
# Sets are represented as a
# Mapping where each key is
# associated with a null value
---
!!set
? Mark McGwire
? Sammy Sosa
? Ken Griff
```

**Example 2.26. Ordered Mappings**
```
# Ordered maps are represented as
# A sequence of mappings, with
# each mapping having one key
---
!!omap
- Mark McGwire: 65
- Sammy Sosa: 63
- Ken Griffy: 58
```

## 2.5. Full Length Example

Below are two full-length examples of YAML. On the left is a sample invoice; on the right is a sample log file.

**Example 2.27. Invoice**
```
---
!<tag:clarkevans.com,2002:invoice>
invoice: 34843
date   : 2001-01-23
bill-to: &id001
    given  : Chris
    family : Dumars
    address:
        lines: |
            458 Walkman Dr.
            Suite #292
        city    : Royal Oak
        state   : MI
        postal  : 48046
ship-to: *id001
product:
    - sku         : BL394D
      quantity    : 4
      description : Basketball
      price       : 450.00
    - sku         : BL4438H
      quantity    : 1
      description : Super Hoop
      price       : 2392.00
tax  : 251.42
total: 4443.52
comments:
    Late afternoon is best.
    Backup contact is Nancy
    Billsmer @ 338-4338.
```

**Example 2.28. Log File**
```
---
  Time: 2001-11-23 15:01:42 -5
  User: ed
  Warning:
    This is an error message
    for the log file
---
  Time: 2001-11-23 15:02:31 -5
  User: ed
  Warning:
    A slightly different error
    message.
---
  Date: 2001-11-23 15:03:17 -5
  User: ed
  Fatal:
    Unknown variable "bar"
  Stack:
    - file: TopClass.py
      line: 23
      code: |
        x = MoreObject("345\n")
    - file: MoreClass.py
      line: 58
      code: |-
        foo = bar
```

## Chapter 3. Processing YAML Information

YAML is both a text format and a method for presenting any native data structure in this format. Therefore,
 this specification defines two concepts: a class of data objects called
 YAML representations, and a
 syntax for presenting YAML representations as a series of
 characters, called a YAML stream. A
 YAML processor is a tool for
 converting information between these complementary views. It is assumed
 that a YAML processor does its work on behalf of another module, called
 an application. This chapter
 describes the information structures a YAML processor must provide to or
 obtain from the application.

YAML information is used in two ways: for machine processing, and for
 human consumption. The challenge of reconciling these two perspectives is
 best done in three distinct translation stages: representation, serialization, and presentation. Representation addresses how YAML
 views native data structures to achieve portability between programming
 environments. Serialization concerns itself with turning a YAML representation into a serial form,
 that is, a form with sequential access constraints. Presentation deals with the formatting
 of a YAML serialization as a
 series of characters in a human-friendly manner.

## 3.1. Processes

Translating between native data structures and a character stream is done in several logically distinct
 stages, each with a well defined input and output data model.

A YAML processor need not expose the serialization or representation stages. It may
 translate directly between native data structures and a character stream. However, such a
 direct translation should take place so that the native data structures are constructed only from
 information available in the representation. In particular, mapping key order, comments, and tag handles should not be
 referenced during composition.

### 3.1.1. Dump

*Dumping* native data structures to a
 character stream is done using
 the following three stages:

**Representing Native Data Structures**

YAML represents any native data
 structure using three node kinds: sequence - an ordered series of entries; mapping - an unordered association
 of unique keys to values; and scalar - any datum with opaque
 structure presentable as a series of Unicode characters.
 Combined, these primitives generate directed graph structures.
 These primitives were chosen because they are both powerful and
 familiar: the sequence corresponds to a Perl array and a Python list, the mapping corresponds to a Perl hash
 table and a Python dictionary. The scalar represents strings, integers,
 dates, and other atomic data types.

Each YAML node requires, in
 addition to its kind and content, a tag specifying its data type. Type
 specifiers are either global URIs, or are local in scope to a
 single application. For example, an integer is represented in YAML with a scalar plus the global tag "tag:yaml.org,2002:int". Similarly, an invoice
 object, particular to a given organization, could be
 represented as a mapping together with the local tag "!invoice". This simple model can represent any
 data structure independent of programming language.

**Serializing the Representation Graph**

For sequential access mediums, such as an event callback API, a
 YAML representation must be *serialized* to an
 ordered tree. Since in a YAML representation, mapping keys are unordered and nodes may be referenced more than once
 (have more than one incoming "arrow"), the
 serialization process is required to impose an ordering on the mapping keys and to replace the
 second and subsequent references to a given node with place holders called aliases. YAML does not specify how
 these serialization details are chosen. It
 is up to the YAML processor to come up with
 human-friendly key order and anchor names, possibly with the help of the application. The result of this
 process, a YAML serialization tree, can then be traversed to produce a series of
 event calls for one-pass processing of YAML data.

**Presenting the Serialization Tree**

The final output process is *presenting* the YAML serializations as a character stream in a human-friendly
 manner. To maximize human readability, YAML offers a rich set of
 stylistic options which go far beyond the minimal functional
 needs of simple data storage. Therefore the YAML processor is required to introduce
 various presentation details when creating
 the stream, such as the
 choice of node styles, how to format scalar content, the amount of indentation, which tag handles to use,
 the node tags to leave unspecified, the
 set of directives to
 provide and possibly even what comments to add. While some of this
 can be done with the help of the application, in general this
 process should be guided by the preferences of the user.

### 3.1.2. Load

*Loading* native data structures from a character stream is done using the
 following three stages:

**Parsing the Presentation Stream**

*Parsing* is the inverse process
 of presentation, it takes a stream of characters and
 produces a series of events. Parsing discards all the details introduced in the presentation process, reporting only
 the serialization events. Parsing can fail due to ill-formed input.

**Composing the Representation Graph**

*Composing* takes a series of serialization events
 and produces a representation graph. Composing discards all the details introduced in the serialization process, producing
 only the representation graph. Composing can fail due to any of several
 reasons.

**Constructing Native Data Structures**

The final input process is *constructing* native data structures from the YAML representation. Construction
 must be based only on the information available in the representation, and not on
 additional serialization or presentation details such as comments, directives, mapping key order, node styles, scalar content format, indentation levels etc. Construction can fail due to the unavailability of the required native data types.

## 3.2. Information Models

This section specifies the formal details of the results of the above
 processes. To maximize data portability between programming languages
 and implementations, users of YAML should be mindful of the distinction
 between serialization or presentation properties and
 those which are part of the YAML representation. Thus, while imposing
 an order on mapping keys is necessary for flattening YAML representations to a
 sequential access medium, this serialization detail must not be used to convey application level information. In a similar manner, while indentation technique and a choice of
 a node style are needed for the
 human readability, these presentation details are neither part of
 the YAML serialization nor
 the YAML representation. By
 carefully separating properties needed for serialization and presentation, YAML representations of application information will be
 consistent and portable between various programming environments.

### 3.2.1. Representation Graph

YAML's representation of native data structure is a rooted, connected, directed graph of tagged nodes. By "directed graph" we
 mean a set of nodes and directed
 edges ("arrows"), where each edge connects one node to another. All the nodes must be reachable from the root node via such edges. Note that the
 YAML graph may include cycles, and a node may have more than one incoming edge.

Nodes that are defined in terms of
 other nodes are collections; nodes that are independent of any other nodes are scalars. YAML supports two kinds of collection nodes: sequences and mappings. Mapping nodes are somewhat tricky because
 their keys are unordered and must be unique.

#### 3.2.1.1. Nodes

A YAML *node* represents a single native data structure.
 Such nodes have *content* of one
 of three *kinds*: scalar, sequence,
 or mapping. In addition, each node has a tag which serves to restrict the set of
 possible values the content can have.

**Scalar**

The content of a *scalar* node is an opaque datum that can be presented as a series of zero or
 more Unicode characters.

**Sequence**

The content of a *sequence* node is an ordered series of zero or more nodes. In particular,
 a sequence may contain the same node more than once. It could
 even contain itself (directly or indirectly).

**Mapping**

The content of a *mapping* node is an unordered set of *key:* *value* node *pairs*, with the restriction that each of
 the keys is unique. YAML
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

YAML represents type
 information of native data structures with a simple identifier, called a *tag*. *Global tags* are URIs and hence
 globally unique across all applications. The
 "tag:" URI scheme is
 recommended for all global YAML tags. In contrast, *local tags* are specific
 to a single application.
 Local tags start with *"!"*, are not URIs
 and are not expected to be globally unique. YAML provides a "TAG" directive to make tag notation less verbose; it also
 offers easy migration from local to global tags. To ensure this,
 local tags are restricted to the URI character set and use URI
 character escaping.

YAML does not mandate any special relationship between different
 tags that begin with the same substring. Tags ending with URI
 fragments (containing "#") are no exception; tags
 that share the same base URI but differ in their fragment part are
 considered to be different, independent tags. By convention,
 fragments are used to identify different "variants" of
 a tag, while "/" is used to define nested tag
 "namespace" hierarchies. However, this is merely a
 convention, and each tag may employ its own rules. For example,
 Perl tags may use "::" to express namespace
 hierarchies, Java tags may use ".", etc.

YAML tags are used to associate meta information with each node. In particular, each tag must specify
 the expected node kind (scalar, sequence, or mapping). Scalar tags must also provide a
 mechanism for converting formatted content to a canonical form for supporting equality testing. Furthermore, a tag
 may provide additional information such as the set of allowed content values for validation,
 a mechanism for tag resolution, or any other data that is applicable to all
 of the tag's nodes.

#### 3.2.1.3. Node Comparison

Since YAML mappings require key uniqueness, representations must include a
 mechanism for testing the equality of nodes. This is non-trivial since YAML
 allows various ways to format scalar content. For example, the integer
 eleven can be written as "0o13" (octal) or
 "0xB" (hexadecimal). If both notations are used as keys in the same mapping, only a YAML processor which recognizes integer formats would correctly flag the duplicate key as an error.

**Canonical Form**

YAML supports the need for scalar equality by requiring that
 every scalar tag must specify a mechanism for
 producing the *canonical form* of any formatted content. This
 form is a Unicode character string which also presents the same content, and can be used for equality testing. While this requirement is
 stronger than a well defined equality operator, it has other
 uses, such as the production of digital signatures.

**Equality**

Two nodes must have the
 same tag and content to be *equal*. Since each tag applies to exactly one kind, this implies that the two nodes must have the same kind to be equal. Two scalars are equal only
 when their tags and
 canonical forms are equal character-by-character. Equality
 of collections is
 defined recursively. Two sequences are equal only when
 they have the same tag and
 length, and each node in
 one sequence is equal
 to the corresponding node in the other sequence. Two mappings are equal
 only when they have the same tag and an equal set of keys, and each key in this set is associated with
 equal values in both mappings.

Different URI schemes may define different rules for testing
 the equality of URIs. Since a YAML processor cannot be reasonably
 expected to be aware of them all, it must resort to a simple
 character-by-character comparison of tags to ensure consistency. This also
 happens to be the comparison method defined by the
 "tag:" URI scheme. Tags in a YAML stream must therefore
 be presented in a
 canonical way so that such comparison would yield the correct
 results.

**Identity**

Two nodes are *identical* only when they represent the same native data structure. Typically, this corresponds to a single
 memory address. Identity should not be confused with equality;
 two equal nodes need not have
 the same identity. A YAML processor may treat equal scalars as if they were
 identical. In contrast, the separate identity of two distinct
 but equal collections must be preserved.

### 3.2.2. Serialization Tree

To express a YAML representation using a serial API,
 it is necessary to impose an order on mapping keys and employ alias nodes to indicate a subsequent occurrence of a previously
 encountered node. The result of
 this process is a *serialization
 tree*, where each node has
 an ordered set of children. This tree can be traversed for a serial
 event-based API. Construction of native data structures from the serial interface should not use key order or anchor names for the preservation
 of application data.

#### 3.2.2.1. Keys Order

In the representation model, mapping keys do not have an
 order. To serialize a mapping, it is necessary to
 impose an *ordering* on its keys. This order is a serialization detail and should not be
 used when composing the representation graph (and hence for the preservation of application data). In every case
 where node order is significant,
 a sequence must be used. For
 example, an ordered mapping can be represented as a sequence of mappings, where each mapping is a single key: value pair. YAML
 provides convenient compact notation for
 this case.

#### 3.2.2.2. Anchors and Aliases

In the representation graph, a node may
 appear in more than one collection. When serializing such data, the first
 occurrence of the node is *identified* by an *anchor*. Each subsequent occurrence is serialized as an *alias node* which refers back to this
 anchor. Otherwise, anchor names are a serialization detail and are discarded once composing is completed. When composing a representation graph from serialized events, an alias
 node refers to the most recent node in the serialization having the
 specified anchor. Therefore, anchors need not be unique within a serialization. In
 addition, an anchor need not have an alias node referring to it. It
 is therefore possible to provide an anchor for all nodes in serialization.

### 3.2.3. Presentation Stream

A YAML *presentation* is a stream of Unicode characters
 making use of of styles, scalar content formats, comments, directives and other presentation details to present a
 YAML serialization in a
 human readable way. Although a YAML processor may provide these details when parsing, they should not be
 reflected in the resulting serialization. YAML allows several serialization trees to be
 contained in the same YAML character stream, as a series of documents separated by markers. Documents appearing in the same
 stream are independent; that is, a node must not appear in more than one serialization tree or representation graph.

#### 3.2.3.1. Node Styles

Each node is presented in some *style*, depending on its kind. The node style is a presentation detail and is not reflected in the serialization tree or representation graph. There are
 two groups of styles. Block styles use indentation to
 denote structure; In contrast, flow styles styles rely on explicit indicators.

YAML provides a rich set of *scalar styles*. Block scalar styles include the literal style and
 the folded style. Flow scalar styles
 include the plain style and two quoted styles, the single-quoted style and the double-quoted style. These
 styles offer a range of trade-offs between expressive power and
 readability.

Normally, block sequences and mappings begin on the next line. In
 some cases, YAML also allows nested block collections to start in-line for a
 more compact notation. In addition, YAML provides a compact notation for flow mappings with a single key: value pair, nested
 inside a flow sequence. These allow for a
 natural "ordered mapping" notation.

#### 3.2.3.2. Scalar Formats

YAML allows scalars to be presented in several *formats*. For
 example, the integer "11" might also be written as
 "0xB". Tags must
 specify a mechanism for converting the formatted content to a canonical form for use in equality testing. Like node style, the format is a presentation detail and is not reflected in the serialization tree and representation graph.

#### 3.2.3.3. Comments

Comments are a presentation detail and must not have any effect on the serialization tree or representation graph. In
 particular, comments are not associated with a particular node. The usual purpose of a comment is to
 communicate between the human maintainers of a file. A typical
 example is comments in a configuration file. Comments must not
 appear inside scalars, but may
 be interleaved with such scalars inside collections.

#### 3.2.3.4. Directives

Each document may be
 associated with a set of directives. A directive has a name
 and an optional sequence of parameters. Directives are instructions
 to the YAML processor, and
 like all other presentation details are not reflected
 in the YAML serialization tree or representation graph. This version of YAML defines a two directives, "YAML" and "TAG".
 All other directives are reserved for future versions of
 YAML.

## 3.3. Loading Failure Points

The process of loading native data structures from a
 YAML stream has several potential *failure
 points*. The character stream may be ill-formed, aliases may be unidentified, unspecified tags may be unresolvable, tags may be unrecognized, the content may be invalid, and a native type may be unavailable. Each of
 these failures results with an incomplete loading.

A *partial
 representation* need not resolve the tag of each node, and the canonical form of formatted scalar content need not be available. This
 weaker representation is useful for cases of incomplete knowledge of
 the types used in the document. In contrast, a *complete representation* specifies the tag of each node, and provides the canonical form of formatted scalar content, allowing for equality testing. A complete
 representation is required in order to construct native data structures.

### 3.3.1. Well-Formed Streams and Identified Aliases

A *well-formed* character stream must match the BNF productions
 specified in the following chapters. Successful loading also requires
 that each alias shall refer to a
 previous node identified by the anchor. A YAML processor should reject *ill-formed streams* and *unidentified
 aliases*. A YAML processor may recover from syntax
 errors, possibly by ignoring certain parts of the input, but it must
 provide a mechanism for reporting such errors.

### 3.3.2. Resolved Tags

Typically, most tags are not
 explicitly specified in the character stream. During parsing, nodes lacking an explicit tag are given a *non-specific tag*: *"!"* for non-plain scalars, and *"?"* for all other nodes. Composing a complete representation requires each such non-specific tag to be *resolved* to a *specific tag*,
 be it a global tag or a local tag.

Resolving the tag of a node must only depend on the following three
 parameters: (1) the non-specific tag of the node, (2) the path leading from the root to the node, and (3) the content (and hence the kind) of the node. When a node has more than one occurrence (using aliases), tag resolution must
 depend only on the path to the first (anchored) occurrence of the node.

Note that resolution must not consider presentation details such as comments, indentation and node style. Also, resolution must not
 consider the content of any
 other node, except for the content of the key nodes directly along the path leading from the root to the resolved node. Finally, resolution must not
 consider the content of a
 sibling node in a collection, or the content of the value node associated with a key node being resolved.

These rules ensure that tag resolution can be performed as soon as a node is first encountered in the stream, typically before its content is parsed. Also, tag resolution only requires
 referring to a relatively small number of previously parsed nodes. Thus, in most cases, tag resolution
 in one-pass processors is both
 possible and practical.

YAML processors should resolve nodes having the "!" non-specific tag as "tag:yaml.org,2002:seq", "tag:yaml.org,2002:map" or "tag:yaml.org,2002:str" depending on their kind. This *tag resolution convention* allows the author of a YAML character stream to effectively
 "disable" the tag resolution process. By explicitly
 specifying a "!" non-specific tag property, the node would then be resolved to a
 "vanilla" sequence, mapping, or string, according to its kind.

Application specific tag
 resolution rules should be restricted to resolving the
 "?" non-specific tag, most commonly to resolving plain scalars. These may be matched against a set of regular
 expressions to provide automatic resolution of integers, floats,
 timestamps, and similar types. An application may also match the content of mapping nodes against sets of expected keys to automatically resolve
 points, complex numbers, and similar types. Resolved sequence node types such as the
 "ordered mapping" are also possible.

That said, tag resolution is specific to the application. YAML processors should therefore provide a
 mechanism allowing the application to override and expand
 these default tag resolution rules.

If a document contains *unresolved tags*, the
 YAML processor is unable to compose a complete representation graph. In such a case, the YAML processor may compose a partial representation, based on each node's kind and allowing for non-specific
 tags.

### 3.3.3. Recognized and Valid Tags

To be *valid*, a node must have a tag which is *recognized* by the YAML processor and its content must satisfy the constraints
 imposed by this tag. If a document contains a scalar node with an *unrecognized tag* or *invalid content*, only a *partial representation* may be composed. In contrast, a YAML processor can always compose a complete representation for an unrecognized or an invalid collection, since collection equality does not depend upon knowledge
 of the collection's data
 type. However, such a complete representation cannot be
 used to construct a native data structure.

### 3.3.4. Available Tags

In a given processing environment, there need not be an *available* native type
 corresponding to a given tag. If a node's tag is *unavailable*, a YAML processor will not be able to construct a native data structure for
 it. In this case, a complete representation may still be composed, and an application may wish to use this representation directly.

## Chapter 4. Syntax Conventions

The following chapters formally define the syntax of YAML character streams, using parameterized BNF
 productions. Each BNF production is both named and numbered for easy
 reference. Whenever possible, basic structures are specified before the
 more complex structures using them in a "bottom up" fashion.

The order of alternatives inside a production is significant. Subsequent
 alternatives are only considered when previous ones fails. Production matching is expected to be greedy. Optional
 (**`?`**), zero-or-more (**`*`**) and
 one-or-more (**`+`**) patterns are always expected to
 match as much of the input as possible.

The productions are accompanied by examples, which are given side-by-side
 next to equivalent YAML text in an explanatory format. This format uses
 only flow collections, double-quoted scalars, and explicit tags for each node.

A reference implementation using the productions is available as the YamlReference Haskell package. This reference implementation is
 also available as an interactive web application at http://dev.yaml.org/ypaste.

## 4.1. Production Parameters

YAML's syntax is designed for maximal human readability. This
 requires parsing to depend on the
 surrounding text. For notational compactness, this dependency is
 expressed using parameterized BNF productions.

This context sensitivity is the cause of most of the complexity of the
 YAML syntax definition. It is further complicated by struggling with
 the human tendency to look ahead when interpreting text. These
 complications are of course the source of most of YAML's power to present data in a very human
 readable way.

Productions use any of the following parameters:

**Indentation: `n` or `m`**

Many productions use an explicit indentation level parameter. This
 is less elegant than Python's "indent" and
 "undent" conceptual tokens. However it is required to
 formally express YAML's indentation rules.

**Context: `c`**

This parameter allows productions to tweak their behavior
 according to their surrounding. YAML supports two groups of *contexts*, distinguishing
 between block styles and flow styles.

In block styles, indentation is used to
 delineate structure. To capture human perception of indentation the
 rules require special treatment of the "-" character, used in block sequences. Hence in some
 cases productions need to behave differently inside block sequences (*block-in context*) and outside them
 (*block-out context*).

In flow styles, explicit indicators are used to delineate
 structure. These styles can be viewed as the natural extension of
 JSON to cover tagged, single-quoted and plain scalars. Since the latter have no delineating indicators, they are subject to
 some restrictions to avoid ambiguities. These restrictions depend
 on where they appear: as implicit keys directly inside a block mapping (*block-key*); as implicit keys
 inside a flow mapping (*flow-key*); as
 values inside a flow collection (*flow-in*); or as
 values outside one (*flow-out*).

**(Block) Chomping: `t`**

Block scalars offer three possible mechanisms for chomping any trailing line breaks: strip, clip and keep. Unlike the
 previous parameters, this only controls interpretation; the line breaks are valid in
 all cases.

## 4.2. Production Naming Conventions

To make it easier to follow production combinations, production names
 use a Hungarian-style naming convention. Each production is given a
 prefix based on the type of characters it begins and ends with.

**`e-`**
A production matching no characters.

**`c-`**
A production starting and ending with a special character.

**`b-`**
A production matching a single line break.

**`nb-`**
A production starting and ending with a non-break character.

**`s-`**
A production starting and ending with a white space character.

**`ns-`**
A production starting and ending with a non-space character.

**`l-`**
A production matching complete line(s).

**`X-Y-`**
A production starting with an `X-` character and ending with a `Y-` character, where `X-` and `Y-` are any of the above
 prefixes.

**`X+`, `X-Y+`**
A production as above, with the additional property that the matched content indentation level is greater than
 the specified `n` parameter.

## Chapter 5. Characters

## 5.1. Character Set

To ensure readability, YAML streams use only the *printable* subset of the Unicode character set. The allowed character range
 explicitly excludes the C0 control block **`#x0-#x1F`** (except for TAB **`#x9`**, LF **`#xA`**, and CR **`#xD`** which are allowed), DEL **`#x7F`**, the C1 control block **`#x80-#x9F`** (except for NEL **`#x85`** which is allowed), the surrogate block **`#xD800-#xDFFF`**, **`#xFFFE`**,
 and **`#xFFFF`**.

On input, a YAML processor must
 accept all Unicode characters except those explicitly excluded above.

On output, a YAML processor must
 only produce acceptable characters. Any excluded characters must be presented using escape sequences. In addition, any allowed
 characters known to be non-printable should also be escaped. This isn't mandatory since a full
 implementation would require extensive character property tables.

To ensure JSON compatibility, YAML processors must allow all non-control
 characters inside quoted scalars. To ensure
 readability, non-printable characters should be escaped on output, even inside such scalars. Note that JSON quoted scalars cannot span multiple lines or contain tabs, but YAML quoted scalars can.

## 5.2. Character Encodings

All characters mentioned in this specification are Unicode code points.
 Each such code point is written as one or more bytes depending on the *character encoding* used. Note that in UTF-16, characters above **`#xFFFF`** are written as four bytes, using a
 surrogate pair.

The character encoding is a presentation detail and must not be used
 to convey content information.

On input, a YAML processor must
 support the UTF-8 and UTF-16 character encodings. For JSON compatibility, the UTF-32
 encodings must also be supported.

If a character stream begins with a *byte order mark*, the
 character encoding will be taken to be as as indicated by the byte
 order mark. Otherwise, the stream must begin with an ASCII character. This allows the encoding to be
 deduced by the pattern of null (**`#x00`**)
 characters.

To make it easier to concatenate streams, byte order marks may appear at the
 start of any document. However
 all documents in the same stream must use the same character
 encoding.

To allow for JSON compatibility, byte order marks are also allowed inside quoted scalars. For readability,
 such content byte order marks
 should be escaped on output.

The encoding can therefore be deduced by matching the first few bytes
 of the stream with the following
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
 first stream character is ASCII.

For more information about the byte order mark and the Unicode
 character encoding schemes see the [Unicode FAQ](http://www.unicode.org/unicode/faq/utf_bom.html).

## 5.3. Indicator Characters

*Indicators* are characters that
 have special semantics.

The "-" character denotes a block sequence entry.

The "?" character denotes a mapping key.

The ":" character denotes a mapping value.

The "," character ends a flow collection entry.

The "[" character starts a flow sequence.

The "]" character ends a flow sequence.

The "{" character starts a flow mapping.

The "}" character ends a flow mapping.

The "#" character denotes a comment.

The "&" character denotes a node's anchor property.

The "*" character denotes an alias node.

The "!" character is heavily overloaded for specifying node tags. It is used to denote tag handles used in tag directives and tag properties; to denote local tags; and as the non-specific tag for non-plain scalars.

The "|" character denotes a literal block scalar.

The ">" character denotes a folded block scalar.

The "'" character surrounds a single-quoted flow scalar.

The '"' character surrounds a double-quoted flow scalar.

The "%" character denotes a directive line.

The "@" character and the `` ` `` character are reserved for future use.

Any indicator character:

The "[", "]", "{", "}", and "," characters denote structure in flow collections. They are therefore forbidden in some cases, to
 avoid ambiguity in several constructs. This is handled on a
 case-by-case basis by the relevant productions.

## 5.4. Line Break Characters

YAML recognizes the following ASCII *line break* characters:
- LF (#xA)
- CR (#xD)

All other characters, including the form feed (#xC), are considered to be non-break
 characters. Note that these include the *non-ASCII line breaks*: next line
 (#x85), line separator
 (#x2028) and paragraph separator
 (#x2029).

YAML version 1.1 did support the above non-ASCII
 line break characters; however, JSON does not. Hence, to ensure JSON compatibility,
 YAML treats them as non-break characters as of version 1.2. In theory
 this would cause incompatibility with version 1.1; in practice these characters were
 rarely (if ever) used. YAML 1.2 processors parsing a version 1.1 document should therefore treat these line
 breaks as non-break characters, with an appropriate warning.

Line breaks are interpreted differently by different systems, and have
 several widely used formats.

Line breaks inside scalar content must be *normalized* by the YAML processor. Each such line break must be parsed into a single line feed
 character. The original line break format is a presentation detail and must not be used to convey content information.

Outside scalar content, YAML allows
 any line break to be used to terminate lines.

On output, a YAML processor is
 free to emit line breaks using whatever convention is most appropriate.

## 5.5. White Space Characters

YAML recognizes two *white space* characters: *space* and *tab*.

The rest of the (printable) non-break characters are considered to be non-space
 characters.

In the examples, tab characters are displayed as the glyph "→". Space characters are sometimes displayed as
 the glyph "·" for clarity.

## 5.6. Miscellaneous Characters

The YAML syntax productions make use of the following additional
 character classes:

A decimal digit for numbers:
- [#x30-#x39] /* 0-9 */

A hexadecimal digit for escape sequences:
- [ns-dec-digit](#ns-dec-digit) | [#x41-#x46] /* A-F */ | [#x61-#x66] /* a-f */

ASCII letter (alphabetic) characters:
- [#x41-#x5A] /* A-Z */ | [#x61-#x7A] /* a-z */

Word (alphanumeric) characters for identifiers:
- [ns-dec-digit](#ns-dec-digit) | [ns-ascii-letter](#ns-ascii-letter) | "-"

URI characters for tags, as specified in RFC2396, with the
 addition of the "[" and "]" for
 presenting IPv6 addresses as proposed in RFC2732.

By convention, any URI characters other than the allowed printable ASCII characters are first *encoded* in UTF-8, and then each byte
 is *escaped* using the *"%" character*. The YAML processor must not expand such
 escaped characters. Tag characters
 must be preserved and compared exactly as presented in the YAML stream, without any processing.

The "!" character is used to indicate the end of a named tag handle; hence its use in tag shorthands is restricted. In addition, such shorthands must not contain the "[", "]", "{", "}", and "," characters. These
 characters would cause ambiguity with flow collection structures.

## 5.7. Escaped Characters

All non-printable characters must be *escaped*. YAML escape sequences use the *"\"* notation common to most modern
 computer languages. Each escape sequence must be parsed into the appropriate Unicode
 character. The original escape sequence is a presentation detail and must not be used to convey content information.

Note that escape sequences are only interpreted in double-quoted scalars. In all other scalar styles, the "\"
 character has no special meaning and non-printable characters are not available.

YAML escape sequences are a superset of C's escape sequences:

- "0" - Escaped ASCII null (#x0) character.
- "a" - Escaped ASCII bell (#x7) character.
- "b" - Escaped ASCII backspace (#x8) character.
- "t" or #x9 - Escaped ASCII horizontal tab (#x9) character. This is useful at the start or the end of a line to force a leading or trailing tab to become part of the content.
- "n" - Escaped ASCII line feed (#xA) character.
- "v" - Escaped ASCII vertical tab (#xB) character.
- "f" - Escaped ASCII form feed (#xC) character.
- "r" - Escaped ASCII carriage return (#xD) character.
- "e" - Escaped ASCII escape (#x1B) character.
- (space) - Escaped ASCII space (#x20) character. This is useful at the start or the end of a line to force a leading or trailing space to become part of the content.
- """ - Escaped ASCII double quote (#x22).
- "/" - Escaped ASCII slash (#x2F), for JSON compatibility.
- "\" - Escaped ASCII back slash (#x5C).
- "N" - Escaped Unicode next line (#x85) character.
- "_" - Escaped Unicode non-breaking space (#xA0) character.
- "L" - Escaped Unicode line separator (#x2028) character.
- "P" - Escaped Unicode paragraph separator (#x2029) character.
- "x" - Escaped 8-bit Unicode character.
- "u" - Escaped 16-bit Unicode character.
- "U" - Escaped 32-bit Unicode character.

## Chapter 6. Basic Structures

## 6.1. Indentation Spaces

In YAML block styles, structure is determined by *indentation*. In general, indentation
 is defined as a zero or more space characters at the start of a line.

To maintain portability, tab characters must not be used in indentation, since different systems
 treat tabs differently. Note that most
 modern editors may be configured so that pressing the tab key results in the insertion of an
 appropriate number of spaces.

The amount of indentation is a presentation detail and must not be used
 to convey content information.

A block style construct is terminated when encountering a line which is less indented
 than the construct.

Each node must be indented further
 than its parent node. All sibling nodes must use the exact same
 indentation level. However the content of each sibling node may be further indented independently.

**Example 6.1. Indentation Spaces**

```
# Leading comment line spaces are
# neither content nor indentation.
    Not indented:
     By one space: |
        By four
          spaces
     Flow style: [
        By two,
         Also by two,
         Still by two
      ]
```

The "-", "?", and ":" characters used to denote block collection entries are perceived by people to be part of the
 indentation. This is handled on a case-by-case basis by the relevant
 productions.

**Example 6.2. Indentation Indicators**

```
? a
: - b
  - - c
    - d
```

## 6.2. Separation Spaces

Outside indentation and scalar content, YAML uses white space characters for *separation* between tokens within a line. Note that such white space may safely include tab characters.

Separation spaces are a presentation detail and must not be used
 to convey content information.

**Example 6.3. Separation Spaces**

```
- foo: bar
- - baz
  - baz
```

## 6.3. Line Prefixes

Inside scalar content, each line
 begins with a non-content *line prefix*. This prefix always
 includes the indentation. For flow scalar styles it additionally includes all leading white space, which may
 contain tab characters.

Line prefixes are a presentation detail and must not be used
 to convey content information.

**Example 6.4. Line Prefixes**

```
plain: text
  lines
quoted: "text
  lines"
block: |
  text
   lines
```

## 6.4. Empty Lines

An *empty line* consists of
 the non-content prefix followed by a line break.

The semantics of empty lines depend on the scalar style they appear in. This is
 handled on a case-by-case basis by the relevant productions.

**Example 6.5. Empty Lines**

```
Folding:
  "Empty line
   as a line feed"
Chomping: |
  Clipped empty lines

```

## 6.5. Line Folding

*Line folding* allows long
 lines to be broken for readability, while retaining the semantics of
 the original long line. If a line break is followed by an empty line, it is *trimmed*; the first line break is discarded and the rest are retained as content.

Otherwise (the following line is not empty), the line break is converted to a single space.

A folded non-empty line may end
 with either of the above line breaks.

The above rules are common to both the folded block style and
 the scalar flow styles. Folding does distinguish between these cases in
 the following way:

**Block Folding**

In the folded block style, the final line break and trailing empty lines are subject
 to chomping, and are never
 folded. In addition, folding does not apply to line breaks surrounding text lines
 that contain leading white space. Note that such a more-indented line may
 consist only of such leading white space.

The combined effect of the *block line folding* rules is that each
 "paragraph" is interpreted as a line, empty lines are interpreted as a
 line feed, and the formatting of more-indented lines is
 preserved.

**Example 6.6. Line Folding**

```
>-
  trimmed

  as
  space
```

**Flow Folding**

Folding in flow styles provides more relaxed semantics. Flow styles typically
 depend on explicit indicators rather than indentation to
 convey structure. Hence spaces preceding or following the text in
 a line are a presentation detail and must not be
 used to convey content information. Once all such spaces have been discarded, all line breaks are folded,
 without exception.

The combined effect of the *flow line folding* rules is that each
 "paragraph" is interpreted as a line, empty lines are interpreted as
 line feeds, and text can be freely more-indented without affecting
 the content information.

**Example 6.7. Flow Folding**

```
"
  foo

  bar

  baz
"
```

## 6.6. Comments

An explicit *comment* is marked by a *"#" indicator*. Comments are a presentation detail and must not be used
 to convey content information.

Comments must be separated from other tokens by white space characters. To ensure JSON compatibility, YAML processors must allow for the omission of
 the final comment line break of the input stream. However, as this
 confuses many tools, YAML processors should terminate the stream with an explicit line break on output.

Outside scalar content, comments
 may appear on a line of their own, independent of the indentation level. Note that outside scalar content, a
 line containing only white space characters is taken to be a comment line.

In most cases, when a line may end with a comment, YAML allows it to be
 followed by additional comment lines.

**Example 6.8. Comment Lines**

```
  # Comment

```

**Example 6.9. Multi-Line Comments**

```
key:   # Comment
         # lines
  value
```

## 6.7. Separation Lines

Implicit keys are
 restricted to a single line. In all other cases, YAML allows tokens to
 be separated by multi-line (possibly empty) comments.

Note that structures following multi-line comment separation must be
 properly indented, even though there is no
 such restriction on the separation comment lines themselves.

**Example 6.10. Separation Spaces**

```
{ first: Sammy, last: Sosa }:
# Statistics:
  hr:   # Home runs
     65
  avg:  # Average
    0.278
```

## 6.8. Directives

*Directives* are instructions to
 the YAML processor. This
 specification defines two directives, "YAML" and "TAG", and *reserves* all other directives for future use. There is no way to define private
 directives. This is intentional.

Directives are a presentation detail and must not be used
 to convey content information.

Each directive is specified on a separate non-indented line starting with the *"%" indicator*,
 followed by the directive name and a list of parameters. The semantics
 of these parameters depends on the specific directive. A YAML processor should ignore unknown
 directives with an appropriate warning.

**Example 6.11. Reserved Directives**

```
%FOO  bar baz # Should be ignored
                # with a warning.
---
"foo"
```

### 6.8.1. "YAML" Directives

The *"YAML" directive* specifies
 the version of YAML the document conforms to. This specification
 defines version "1.2", including recommendations for *YAML 1.1 processing*.

A version 1.2 YAML processor must accept documents with an
 explicit "%YAML 1.2" directive, as well as documents lacking a
 "YAML" directive. Such documents are assumed to conform to the
 1.2 version specification. Documents with a "YAML"
 directive specifying a higher minor version (e.g.
 "%YAML 1.3") should be processed with an
 appropriate warning. Documents with a "YAML" directive specifying a higher major
 version (e.g. "%YAML 2.0") should be rejected
 with an appropriate error message.

A version 1.2 YAML processor must also accept documents with
 an explicit "%YAML 1.1" directive. Note that version
 1.2 is mostly a superset of version 1.1, defined for the purpose of
 ensuring JSON compatibility. Hence a version 1.2 processor should process version 1.1 documents as if they were
 version 1.2, giving a warning on points of incompatibility.

It is an error to specify more than one "YAML"
 directive for the same document, even if both occurrences give the
 same version number.

**Example 6.12. Invalid Repeated YAML directive**

```
%YAML 1.2
%YAML 1.1
foo
```

### 6.8.2. "TAG" Directives

The *"TAG" directive* establishes a tag shorthand notation for specifying node tags. Each "TAG"
 directive associates a handle with a prefix. This allows for compact and
 readable tag notation.

**Example 6.13. "TAG" directive**

```
%TAG !yaml! tag:yaml.org,2002:
---
!yaml!str "foo"
```

It is an error to specify more than one "TAG"
 directive for the same handle in the same document, even if
 both occurrences give the same prefix.

**Example 6.14. Invalid Repeated TAG directive**

```
%TAG ! !foo
%TAG ! !foo bar
```

#### 6.8.2.1. Tag Handles

The *tag handle* exactly matches the prefix of the affected tag shorthand. There are three tag
 handle variants:

**Primary Handle**

The *primary tag handle* is a single *"!"* character. This allows
 using the most compact possible notation for a single
 "primary" name space. By default, the prefix
 associated with this handle is "!". Thus, by default, shorthands using this handle are interpreted as local tags.

It is possible to override the default behavior by providing
 an explicit "TAG" directive, associating a
 different prefix for this handle. This provides smooth
 migration from using local tags to using global tags, by
 the simple addition of a single "TAG" directive.

**Example 6.15. Primary Tag Handle**

```
# Private
!foo "bar"
...
# Global
%TAG ! tag:example.com,2000:app/
---
!foo "bar"
```

**Secondary Handle**

The *secondary tag handle* is
 written as *"!!"*. This
 allows using a compact notation for a single
 "secondary" name space. By default, the prefix
 associated with this handle is "tag:yaml.org,2002:". This prefix is used by
 the YAML tag repository.

It is possible to override this default behavior by providing
 an explicit "TAG" directive associating a
 different prefix for this handle.

**Example 6.16. Secondary Tag Handle**

```
%TAG !! tag:example.com,2000:app/
---
!!int 1 - 3 # Interval, not integer
```

**Named Handles**

A *named tag handle* surrounds a
 non-empty name with *"!"* characters. A handle
 name must not be used in a tag shorthand unless an
 explicit "TAG" directive has associated some
 prefix with it.

The name of the handle is a presentation detail and must not
 be used to convey content information. In
 particular, the YAML processor need not preserve the
 handle name once parsing is completed.

**Example 6.17. Tag Handles**

```
%TAG !e! tag:example.com,2000:app/
---
!e!foo "bar"
```

#### 6.8.2.2. Tag Prefixes

There are two *tag prefix* variants:

**Local Tag Prefix**

If the prefix begins with a "!" character, shorthands using the handle are expanded
 to a local tag. Note that such a tag is intentionally not a valid URI,
 and its semantics are specific to the application. In particular, two documents in the same stream may assign different
 semantics to the same local tag.

**Example 6.18. Local Tag Prefix**

```
%TAG !m! !my-
---
# Bulb here
!m!light fluorescent
...
%TAG !m! !my-
---
# Color here
!m!light green
```

**Global Tag Prefix**

If the prefix begins with a character other than "!", it must to be a valid URI
 prefix, and should contain at least the scheme and the
 authority. Shorthands using the associated handle are expanded to globally unique URI tags, and their semantics is consistent across applications. In particular, every documents in every stream must assign the same
 semantics to the same global tag.

**Example 6.19. Global Tag Prefix**

```
%TAG !e! tag:example.com,2000:app/
---
- !e!foo "bar"
```

## 6.9. Node Properties

Each node may have two optional *properties*, anchor and tag, in addition to its content. Node properties may be specified
 in any order before the node's content. Either or both may be omitted.

**Example 6.20. Node Properties**

```
!!str &a1 "foo":
  !!str bar
&a2 baz : *a1
```

### 6.9.1. Node Tags

The *tag property* identifies the type of the native data structure presented by the node. A tag is denoted by the *"!" indicator*.

**Verbatim Tags**

A tag may be written *verbatim* by surrounding it with
 the *"<" and ">"* characters. In this case, the YAML processor must deliver the verbatim
 tag as-is to the application. In particular,
 verbatim tags are not subject to tag resolution. A verbatim tag
 must either begin with a "!" (a local tag) or be
 a valid URI (a global tag).

**Example 6.21. Verbatim Tags**

```
!<tag:yaml.org,2002:str> foo :
    !<!bar> baz
```

**Example 6.22. Invalid Verbatim Tags**

```
- !<!> foo
- !<$:?> bar
```

**Tag Shorthands**

A *tag shorthand* consists of a valid tag handle followed by a non-empty
 suffix. The tag handle must be associated with a prefix, either by
 default or by using a "TAG" directive. The
 resulting parsed tag is the concatenation of the prefix and
 the suffix, and must either begin with "!" (a local tag) or be a valid URI (a global tag).

The choice of tag handle is a presentation detail and must not
 be used to convey content information. In particular, the tag handle may be discarded once parsing is completed.

The suffix must not contain any "!" character. This would
 cause the tag shorthand to be interpreted as having a named tag handle. In addition, the suffix must not contain the "[", "]", "{", "}", and "," characters. These
 characters would cause ambiguity with flow collection structures. If the suffix needs to specify
 any of the above restricted characters, they must be escaped using the "%" character. This behavior is
 consistent with the URI character escaping rules.

**Example 6.23. Tag Shorthands**

```
%TAG !e! tag:example.com,2000:app/
---
- !local foo
- !!str bar
- !e!tag%21 baz
```

**Example 6.24. Invalid Tag Shorthands**

```
%TAG !e! tag:example,2000:app/
---
- !e! foo
- !h!bar baz
```

**Non-Specific Tags**

If a node has no tag property, it is assigned a non-specific tag that needs
 to be resolved to a specific one. This non-specific tag is "!" for
 non-plain scalars and "?" for
 all other nodes. This is the
 only case where the node style has any effect on the content information.

It is possible for the tag property to be explicitly set to the "!" non-specific tag. By convention, this
 "disables" tag resolution, forcing the node to be interpreted as
 "tag:yaml.org,2002:seq", "tag:yaml.org,2002:map", or
 "tag:yaml.org,2002:str", according to its kind.

There is no way to explicitly specify the "?" non-specific tag. This is intentional.

**Example 6.25. Non-Specific Tags**

```
# Assuming conventional resolution:
- "12"
- 12
- ! 12
```

### 6.9.2. Node Anchors

An anchor is denoted by the *"&" indicator*. It marks a node for future reference. An alias node can then be used to
 indicate additional inclusions of the anchored node. An anchored node need not be referenced by any alias nodes; in particular, it is valid for
 all nodes to be anchored.

Note that as a serialization detail, the anchor name is
 preserved in the serialization tree. However, it is not reflected in the representation graph and must not
 be used to convey content information. In particular, the YAML processor need not preserve the anchor
 name once the representation is composed.

Anchor names must not contain the "[", "]", "{", "}", and "," characters. These
 characters would cause ambiguity with flow collection structures.

**Example 6.26. Node Anchors**

```
First occurrence: &anchor Value
Second occurrence: *anchor
```

## Chapter 7. Flow Styles

YAML's *flow styles* can be thought of as the natural extension of JSON to cover folding long content lines for
 readability, tagging nodes to control construction of native data structures, and
 using anchors and aliases to reuse constructed object instances.

## 7.1. Alias Nodes

Subsequent occurrences of a previously serialized node are presented as *alias nodes*. The first occurrence of the node must be marked by an anchor to allow subsequent occurrences to be presented as alias nodes.

An alias node is denoted by the *"*" indicator*. The alias refers to the
 most recent preceding node having the
 same anchor. It is an error for an
 alias node to use an anchor that
 does not previously occur in the document. It is not an error to specify an anchor that is not used by any
 alias node.

Note that an alias node must not specify any properties or content, as these were already specified at the first
 occurrence of the node.

**Example 7.1. Alias Nodes**

```
First occurrence: &anchor Foo
Second occurrence: *anchor
Override anchor: &anchor Bar
Reuse anchor: *anchor
```

## 7.2. Empty Nodes

YAML allows the node content to be
 omitted in many cases. Nodes with
 empty content are interpreted as
 if they were plain scalars with an empty value. Such nodes are commonly resolved to a "null" value.

In the examples, empty scalars are
 sometimes displayed as the glyph "°" for clarity.
 Note that this glyph corresponds to a position in the characters stream rather than to an actual
 character.

**Example 7.2. Empty Content**

```
{
  foo : !!str,
  !!str : bar,
}
```

Both the node's properties and node content are optional. This allows for a *completely empty
 node*. Completely empty nodes are only valid when following
 some explicit indication for their existence.

**Example 7.3. Completely Empty Flow Nodes**

```
{
  ? foo :,
  : bar,
}
```

## 7.3. Flow Scalar Styles

YAML provides three *flow scalar styles*: double-quoted, single-quoted and plain (unquoted). Each provides a different trade-off between readability and
 expressive power.

The scalar style is a presentation detail and must not be used to convey content information, with the exception
 that plain scalars are distinguished for the purpose of tag resolution.

### 7.3.1. Double-Quoted Style

The *double-quoted style* is specified
 by surrounding *'"'* indicators. This is the only style capable of expressing
 arbitrary strings, by using "\" escape sequences. This comes at the cost of having to escape the "\" and '"' characters.

Double-quoted scalars are restricted to a single line when contained
 inside an implicit key.

**Example 7.4. Double Quoted Implicit Keys**

```
"implicit block key" : [
    "implicit flow key" : value,
]
```

In a multi-line double-quoted scalar, line breaks are subject to flow line folding, which discards any trailing white space characters. It is also
 possible to *escape* the line break character. In this case, the line break is excluded from the content, and the trailing white space characters are preserved. Combined with the ability to escape white space characters, this allows
 double-quoted lines to be broken at arbitrary positions.

**Example 7.5. Double Quoted Line Breaks**

```
"folded
to a space,

to a line feed, or  \
  \  non-content"
```

All leading and trailing white space characters are excluded from
 the content. Each continuation
 line must therefore contain at least one non-space character. Empty lines, if any, are
 consumed as part of the line folding.

**Example 7.6. Double Quoted Lines**

```
"  1st non-empty

  2nd non-empty  
  3rd non-empty  "
```

### 7.3.2. Single-Quoted Style

The *single-quoted style* is specified
 by surrounding *"'"* indicators. Therefore, within a
 single-quoted scalar, such characters need to be repeated. This is
 the only form of *escaping* performed in single-quoted
 scalars. In particular, the "\" and '"' characters may be freely used. This restricts single-quoted scalars
 to printable characters. In addition, it is only

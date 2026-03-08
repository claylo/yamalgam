<div class="book" lang="en">

<div class="titlepage">

<div>

<div>

# <span id="id664002"></span> YAML Ain’t Markup Language (<span class="trademark">YAML</span>™) Version 1.1

</div>

<div>

## Final Draft -- 2005-01-18

</div>

<div>

<div class="authorgroup">

<div class="author">

### <span class="firstname">Oren</span> <span class="surname">Ben-Kiki</span>

`<`[`oren@ben-kiki.org`](mailto:oren@ben-kiki.org)`>`

</div>

<div class="author">

### <span class="firstname">Clark</span> <span class="surname">Evans</span>

`<`[`cce@clarkevans.com`](mailto:cce@clarkevans.com)`>`

</div>

<div class="author">

### <span class="firstname">Ingy</span> <span class="surname">döt Net</span>

`<`[`ingy@ingy.net`](mailto:ingy@ingy.net)`>`

</div>

</div>

</div>

<div>

<span class="emphasis">*This version:*</span> <a href="/spec/cvs/current.html" target="_top">html</a>, <a href="/spec/cvs/current.ps" target="_top">ps</a>, <a href="/spec/cvs/current.pdf" target="_top">pdf</a>.\
<span class="emphasis">*Latest version:*</span> <a href="/spec/current.html" target="_top">html</a>, <a href="/spec/current.ps" target="_top">ps</a>, <a href="/spec/current.pdf" target="_top">pdf</a>.

</div>

<div>

Copyright © 2001-2008 Oren Ben-Kiki, Clark Evans, Ingy döt Net

</div>

<div>

<div class="legalnotice">

<span id="id838350"></span> This document may be freely copied, provided it is not modified.

</div>

</div>

<div>

<div class="abstract">

**Status of this Document**

This specification is a draft reflecting consensus reached by members of the <a href="http://lists.sourceforge.net/lists/listinfo/yaml-core" target="_top">yaml-core mailing list</a>. Any questions regarding this draft should be raised on this list. We expect all further changes to be strictly limited to wording corrections and fixing production bugs.

We wish to thank implementers, who have tirelessly tracked earlier versions of this specification, as well as our fabulous user community whose feedback has both validated and clarified our direction.

</div>

</div>

<div>

<div class="abstract">

**Abstract**

<span class="trademark">YAML</span>™ (rhymes with “<span class="quote">camel</span>”) is a human-friendly, cross language, Unicode based data serialization language designed around the common native data structures of agile programming languages. It is broadly useful for programming needs ranging from configuration files to Internet messaging to object persistence to data auditing. Together with the <a href="http://www.unicode.org/" target="_top">Unicode standard for characters</a>, this specification provides all the information necessary to understand YAML Version 1.1 and to create programs that process YAML information.

</div>

</div>

</div>

------------------------------------------------------------------------

</div>

<div class="toc">

**Table of Contents**

<span class="chapter"> [1. Introduction](#id838426) </span>

<span class="sect1"> [1.1. Goals](#id838638) </span>

<span class="sect1"> [1.2. Prior Art](#id838686) </span>

<span class="sect1"> [1.3. Relation to XML](#id856927) </span>

<span class="sect1"> [1.4. Terminology](#id856957) </span>

<span class="chapter"> [2. Preview](#id857168) </span>

<span class="sect1"> [2.1. Collections](#id857181) </span>

<span class="sect1"> [2.2. Structures](#id857577) </span>

<span class="sect1"> [2.3. Scalars](#id858081) </span>

<span class="sect1"> [2.4. Tags](#id858600) </span>

<span class="sect1"> [2.5. Full Length Example](#id859040) </span>

<span class="chapter"> [3. Processing YAML Information](#id859109) </span>

<span class="sect1"> [3.1. Processes](#id859458) </span>

<span class="sect2"> [3.1.1. Represent](#id859497) </span>

<span class="sect2"> [3.1.2. Serialize](#id859873) </span>

<span class="sect2"> [3.1.3. Present](#id860109) </span>

<span class="sect2"> [3.1.4. Parse](#id860341) </span>

<span class="sect2"> [3.1.5. Compose](#id860452) </span>

<span class="sect2"> [3.1.6. Construct](#id860557) </span>

<span class="sect1"> [3.2. Information Models](#id860735) </span>

<span class="sect2"> [3.2.1. Representation Graph](#id861060) </span>

<span class="sect3"> [3.2.1.1. Nodes](#id861435) </span>

<span class="sect3"> [3.2.1.2. Tags](#id861700) </span>

<span class="sect3"> [3.2.1.3. Nodes Comparison](#id862121) </span>

<span class="sect2"> [3.2.2. Serialization Tree](#id862929) </span>

<span class="sect3"> [3.2.2.1. Keys Order](#id863110) </span>

<span class="sect3"> [3.2.2.2. Anchors and Aliases](#id863390) </span>

<span class="sect2"> [3.2.3. Presentation Stream](#id863676) </span>

<span class="sect3"> [3.2.3.1. Node Styles](#id863975) </span>

<span class="sect3"> [3.2.3.2. Scalar Formats](#id864510) </span>

<span class="sect3"> [3.2.3.3. Comments](#id864687) </span>

<span class="sect3"> [3.2.3.4. Directives](#id864824) </span>

<span class="sect1"> [3.3. Loading Failure Points](#id864977) </span>

<span class="sect2"> [3.3.1. Well-Formed and Identified](#id865423) </span>

<span class="sect2"> [3.3.2. Resolved](#id865585) </span>

<span class="sect2"> [3.3.3. Recognized and Valid](#id866900) </span>

<span class="sect2"> [3.3.4. Available](#id867229) </span>

<span class="chapter"> [4. Productions Conventions](#id867381) </span>

<span class="sect1"> [4.1. Production Prefixes](#id867562) </span>

<span class="sect1"> [4.2. Production Parameters](#id867808) </span>

<span class="chapter"> [5. Characters](#id868518) </span>

<span class="sect1"> [5.1. Character Set](#id868524) </span>

<span class="sect1"> [5.2. Character Encoding](#id868742) </span>

<span class="sect1"> [5.3. Indicator Characters](#id868988) </span>

<span class="sect1"> [5.4. Line Break Characters](#id871136) </span>

<span class="sect1"> [5.5. Miscellaneous Characters](#id871856) </span>

<span class="sect1"> [5.6. Escape Sequences](#id872840) </span>

<span class="chapter"> [6. Syntax Primitives](#id891745) </span>

<span class="sect1"> [6.1. Indentation Spaces](#id891751) </span>

<span class="sect1"> [6.2. Comments](#id892353) </span>

<span class="sect1"> [6.3. Separation Spaces](#id893014) </span>

<span class="sect1"> [6.4. Ignored Line Prefix](#id893482) </span>

<span class="sect1"> [6.5. Line Folding](#id894136) </span>

<span class="chapter"> [7. YAML Character Stream](#id895107) </span>

<span class="sect1"> [7.1. Directives](#id895217) </span>

<span class="sect2"> [7.1.1. “<span class="quote">**`YAML`**</span>” Directive](#id895631) </span>

<span class="sect2"> [7.1.2. “<span class="quote">**`TAG`**</span>” Directive](#id896044) </span>

<span class="sect3"> [7.1.2.1. Tag Prefixes](#id896411) </span>

<span class="sect3"> [7.1.2.2. Tag Handles](#id896876) </span>

<span class="sect1"> [7.2. Document Boundary Markers](#id897596) </span>

<span class="sect1"> [7.3. Documents](#id898031) </span>

<span class="sect1"> [7.4. Complete Stream](#id898785) </span>

<span class="chapter"> [8. Nodes](#id899622) </span>

<span class="sect1"> [8.1. Node Anchors](#id899912) </span>

<span class="sect1"> [8.2. Node Tags](#id900262) </span>

<span class="sect1"> [8.3. Node Content](#id901659) </span>

<span class="sect1"> [8.4. Alias Nodes](#id902561) </span>

<span class="sect1"> [8.5. Complete Nodes](#id902919) </span>

<span class="sect2"> [8.5.1. Flow Nodes](#id902924) </span>

<span class="sect2"> [8.5.2. Block Nodes](#id903421) </span>

<span class="chapter"> [9. Scalar Styles](#id903915) </span>

<span class="sect1"> [9.1. Flow Scalar Styles](#id904158) </span>

<span class="sect2"> [9.1.1. Double Quoted](#id904245) </span>

<span class="sect2"> [9.1.2. Single Quoted](#id905860) </span>

<span class="sect2"> [9.1.3. Plain](#id907281) </span>

<span class="sect1"> [9.2. Block Scalar Header](#id926597) </span>

<span class="sect2"> [9.2.1. Block Style Indicator](#id926836) </span>

<span class="sect2"> [9.2.2. Block Indentation Indicator](#id927035) </span>

<span class="sect2"> [9.2.3. Block Chomping Indicator](#id927557) </span>

<span class="sect1"> [9.3. Block Scalar Styles](#id928806) </span>

<span class="sect2"> [9.3.1. Literal](#id928909) </span>

<span class="sect2"> [9.3.2. Folded](#id929764) </span>

<span class="chapter"> [10. Collection Styles](#id930798) </span>

<span class="sect1"> [10.1. Sequence Styles](#id931088) </span>

<span class="sect2"> [10.1.1. Flow Sequences](#id931285) </span>

<span class="sect2"> [10.1.2. Block Sequences](#id931893) </span>

<span class="sect1"> [10.2. Mapping Styles](#id932806) </span>

<span class="sect2"> [10.2.1. Flow Mappings](#id933010) </span>

<span class="sect2"> [10.2.2. Block Mappings](#id934537) </span>

<span class="appendix"> [Index](#id935693) </span>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id838426"></span>Chapter 1. Introduction

</div>

</div>

</div>

“<span class="quote">YAML Ain’t Markup Language</span>” (abbreviated YAML) is a data serialization language designed to be human-friendly and work well with modern programming languages for common everyday tasks. This specification is both an introduction to the YAML language and the concepts supporting it; it is also a complete reference of the information needed to develop <span id="id838445" class="indexterm"></span>[applications](#application/) for processing YAML.

Open, interoperable and readily understandable tools have advanced computing immensely. YAML was designed from the start to be useful and friendly to people working with data. It uses Unicode <span id="id838465" class="indexterm"></span>[printable](#printable%20character/) characters, some of which provide structural information and the rest containing the data itself. YAML achieves a unique cleanness by minimizing the amount of structural characters and allowing the data to show itself in a natural and meaningful way. For example, <span id="id838485" class="indexterm"></span>[indentation](#indentation%20space/) may be used for structure, colons separate “<span class="quote"><span id="id838503" class="indexterm"></span>[mapping key:](#key/information%20model) <span id="id838518" class="indexterm"></span>[value](#value/information%20model)</span>” pairs, and dashes are used to create “<span class="quote">bullet</span>” lists.

There are myriad flavors of data structures, but they can all be adequately <span id="id838545" class="indexterm"></span>[represented](#represent/) with three basic primitives: <span id="id838557" class="indexterm"></span>[mappings](#mapping/information%20model) (hashes/dictionaries), <span id="id838576" class="indexterm"></span>[sequences](#sequence/information%20model) (arrays/lists) and <span id="id838592" class="indexterm"></span>[scalars](#scalar/information%20model) (strings/numbers). YAML leverages these primitives and adds a simple typing system and <span id="id838612" class="indexterm"></span>[aliasing](#alias/information%20model) mechanism to form a complete language for serializing any data structure. While most programming languages can use YAML for data serialization, YAML excels in working with those languages that are fundamentally built around the three basic primitives. These include the new wave of agile languages such as Perl, Python, PHP, Ruby, and Javascript.

There are hundreds of different languages for programming, but only a handful of languages for storing and transferring data. Even though its potential is virtually boundless, YAML was specifically created to work well for common use cases such as: configuration files, log files, interprocess messaging, cross-language data sharing, object persistence, and debugging of complex data structures. When data is easy to view and understand, programming becomes a simpler task.

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id838638"></span>1.1. Goals

</div>

</div>

</div>

The design goals for YAML are:

<div class="orderedlist">

1.  YAML is easily readable by humans.
2.  YAML matches the native data structures of agile languages.
3.  YAML data is portable between programming languages.
4.  YAML has a consistent model to support generic tools.
5.  YAML supports one-pass processing.
6.  YAML is expressive and extensible.
7.  YAML is easy to implement and use.

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id838686"></span>1.2. Prior Art

</div>

</div>

</div>

YAML’s initial direction was set by the data serialization and markup language discussions among <a href="http://www.docuverse.com/smldev/" target="_top">SML-DEV members</a>. Later on, it directly incorporated experience from Brian Ingerson’s Perl module <a href="http://search.cpan.org/doc/INGY/Data-Denter-0.13/Denter.pod" target="_top">Data::Denter</a>. Since then, YAML has matured through ideas and support from its user community.

YAML integrates and builds upon concepts described by <a href="http://cm.bell-labs.com/cm/cs/cbook/index.html" target="_top">C</a>, <a href="http://java.sun.com/" target="_top">Java</a>, <a href="http://www.perl.org/" target="_top">Perl</a>, <a href="http://www.python.org/" target="_top">Python</a>, <a href="http://www.ruby-lang.org/" target="_top">Ruby</a>, <a href="http://www.ietf.org/rfc/rfc0822.txt" target="_top">RFC0822</a> (MAIL), <a href="http://www.ics.uci.edu/pub/ietf/html/rfc1866.txt" target="_top">RFC1866</a> (HTML), <a href="http://www.ietf.org/rfc/rfc2045.txt" target="_top">RFC2045</a> (MIME), <a href="http://www.ietf.org/rfc/rfc2396.txt" target="_top">RFC2396</a> (URI), <a href="http://www.w3.org/TR/REC-xml.html" target="_top">XML</a>, <a href="http://www.saxproject.org/" target="_top">SAX</a> and <a href="http://www.w3.org/TR/SOAP" target="_top">SOAP</a>.

The syntax of YAML was motivated by Internet Mail (RFC0822) and remains partially compatible with that standard. Further, borrowing from MIME (RFC2045), YAML’s top-level production is a <span id="id838704" class="indexterm"></span>[stream](#stream/information%20model) of independent <span id="id838821" class="indexterm"></span>[documents](#document/information%20model); ideal for message-based distributed processing systems.

YAML’s <span id="id838828" class="indexterm"></span>[indentation](#indentation%20space/)-based scoping is similar to Python’s (without the ambiguities caused by <span id="id838859" class="indexterm"></span>[tabs](#tab/)). <span id="id856349" class="indexterm"></span>[Indented blocks](#block%20style/information%20model) facilitate easy inspection of the data’s structure. YAML’s <span id="id856368" class="indexterm"></span>[literal style](#literal%20style/information%20model) leverages this by enabling formatted text to be cleanly mixed within an <span id="id856385" class="indexterm"></span>[indented](#indentation%20space/) structure without troublesome <span id="id856400" class="indexterm"></span>[escaping](#escaping%20in%20double-quoted%20style/). YAML also allows the use of traditional <span id="id856418" class="indexterm"></span>[indicator](#indicator/)-based scoping similar to Perl’s. Such <span id="id856431" class="indexterm"></span>[flow content](#flow%20style/information%20model) can be freely nested inside <span id="id856448" class="indexterm"></span>[indented blocks](#block%20style/information%20model).

YAML’s <span id="id856456" class="indexterm"></span>[double-quoted style](#double-quoted%20style/information%20model) uses familiar C-style <span id="id856488" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/). This enables ASCII encoding of non-<span id="id856500" class="indexterm"></span>[printable](#printable%20character/) or 8-bit (ISO 8859-1) characters such as [“<span class="quote">**`\x3B`**</span>”](#ns-esc-8-bit). Non-<span id="id856526" class="indexterm"></span>[printable](#printable%20character/) 16-bit Unicode and 32-bit (ISO/IEC 10646) characters are supported with <span id="id856541" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/) such as [“<span class="quote">**`\u003B`**</span>”](#ns-esc-16-bit) and [“<span class="quote">**`\U0000003B`**</span>”](#ns-esc-32-bit).

Motivated by HTML’s end-of-line normalization, YAML’s <span id="id856583" class="indexterm"></span>[line folding](#line%20folding/) employs an intuitive method of handling <span id="id856600" class="indexterm"></span>[line breaks](#line%20break%20character/). A single <span id="id856614" class="indexterm"></span>[line break](#line%20break%20character/) is <span id="id856628" class="indexterm"></span>[folded](#line%20folding/) into a single space, while <span id="id856641" class="indexterm"></span>[empty lines](#empty%20line/) are interpreted as <span id="id856655" class="indexterm"></span>[line break](#line%20break%20character/) characters. This technique allows for paragraphs to be word-wrapped without affecting the <span id="id856670" class="indexterm"></span>[canonical form](#canonical%20form/) of the <span id="id856683" class="indexterm"></span>[content](#content/information%20model).

YAML’s core type system is based on the requirements of agile languages such as Perl, Python, and Ruby. YAML directly supports both <span id="id856711" class="indexterm"></span>[collection](#collection/information%20model) (<span id="id856728" class="indexterm"></span>[mapping](#mapping/information%20model), <span id="id856742" class="indexterm"></span>[sequence](#sequence/information%20model)) and <span id="id856758" class="indexterm"></span>[scalar content](#scalar/information%20model). Support for common types enables programmers to use their language’s native data structures for YAML manipulation, instead of requiring a special document object model (DOM).

Like XML’s SOAP, YAML supports <span id="id856780" class="indexterm"></span>[serializing](#serialize/) native graph data structures through an <span id="id856793" class="indexterm"></span>[aliasing](#alias/information%20model) mechanism. Also like SOAP, YAML provides for <span id="id856812" class="indexterm"></span>[application](#application/)-defined <span id="id856822" class="indexterm"></span>[types](#tag/information%20model). This allows YAML to <span id="id856839" class="indexterm"></span>[represent](#represent/) rich data structures required for modern distributed computing. YAML provides globally unique <span id="id856853" class="indexterm"></span>[type names](#global%20tag/) using a namespace mechanism inspired by Java’s DNS-based package naming convention and XML’s URI-based namespaces.

YAML was designed to support incremental interfaces that include both input (“<span class="quote">**`getNextEvent()`**</span>”) and output “<span class="quote">**`sendNextEvent()`**</span>”) one-pass interfaces. Together, these enable YAML to support the processing of large <span id="id856894" class="indexterm"></span>[documents](#document/information%20model) (e.g. transaction logs) or continuous <span id="id856909" class="indexterm"></span>[streams](#stream/information%20model) (e.g. feeds from a production machine).

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id856927"></span>1.3. Relation to XML

</div>

</div>

</div>

Newcomers to YAML often search for its correlation to the eXtensible Markup Language (XML). Although the two languages may actually compete in several application domains, there is no direct correlation between them.

YAML is primarily a data serialization language. XML was designed to be backwards compatible with the Standard Generalized Markup Language (SGML) and thus had many design constraints placed on it that YAML does not share. Inheriting SGML’s legacy, XML is designed to support structured documentation, where YAML is more closely targeted at data structures and messaging. Where XML is a pioneer in many domains, YAML is the result of lessons learned from XML and other technologies.

It should be mentioned that there are ongoing efforts to define standard XML/YAML mappings. This generally requires that a subset of each language be used. For more information on using both XML and YAML, please visit <a href="/xml/index.html" target="_top">https://yaml.org/xml/index.html</a>.

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id856957"></span>1.4. Terminology

</div>

</div>

</div>

This specification uses key words based on <a href="http://www.ietf.org/rfc/rfc2119.txt" target="_top">RFC2119</a> to indicate requirement level. In particular, the following words are used to describe the actions of a YAML <span id="id856974" class="indexterm"></span>[processor](#processor/):

<div class="variablelist">

<span class="term">May</span>  
The word <span id="id856999" class="indexterm"></span><span id="may/"></span>*may*, or the adjective <span id="id857013" class="indexterm"></span><span id="optional/"></span>*optional*, mean that conforming YAML <span id="id857027" class="indexterm"></span>[processors](#processor/) are permitted, but <span id="id857040" class="indexterm"></span><span id="need not/"></span>*need not* behave as described.

<span class="term">Should</span>  
The word <span id="id857065" class="indexterm"></span><span id="should/"></span>*should*, or the adjective <span id="id857079" class="indexterm"></span><span id="recommended/"></span>*recommended*, mean that there could be reasons for a YAML <span id="id857093" class="indexterm"></span>[processor](#processor/) to deviate from the behavior described, but that such deviation could hurt interoperability and should therefore be advertised with appropriate notice.

<span class="term">Must</span>  
The word <span id="id857120" class="indexterm"></span><span id="must/"></span>*must*, or the term <span id="id857133" class="indexterm"></span><span id="required/"></span>*required* or <span id="id857147" class="indexterm"></span><span id="shall/"></span>*shall*, mean that the behavior described is an absolute requirement of the specification.

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id857168"></span>Chapter 2. Preview

</div>

</div>

</div>

This section provides a quick glimpse into the expressive power of YAML. It is not expected that the first-time reader grok all of the examples. Rather, these selections are used as motivation for the remainder of the specification.

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id857181"></span>2.1. Collections

</div>

</div>

</div>

YAML’s <span id="id857190" class="indexterm"></span>[block collections](#block%20collection%20style/information%20model) use <span id="id857209" class="indexterm"></span>[indentation](#indentation%20space/) for scope and begin each entry on its own line. <span id="id857222" class="indexterm"></span>[Block sequences](#block%20sequence%20style/information%20model) indicate each entry with a dash and space ( <span id="id857230" class="indexterm"></span>[“<span class="quote">**`-`**</span>”](#-%20block%20sequence%20entry/)). <span id="id857262" class="indexterm"></span>[Mappings](#mapping/information%20model) use a colon and space (<span id="id857279" class="indexterm"></span>[“<span class="quote">**`: `**</span>”](#:%20mapping%20value/)) to mark each <span id="id857298" class="indexterm"></span>[mapping key](#key/information%20model): <span id="id857312" class="indexterm"></span>[value](#value/information%20model) pair.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id857339"></span>
<p><strong>Example 2.1.  Sequence of Scalars<br />
(ball players)</strong></p>
<pre class="programlisting"><code>- Mark McGwire
- Sammy Sosa
- Ken Griffey</code></pre>
</div></td>
<td><div class="example">
<span id="id857364"></span>
<p><strong>Example 2.2.  Mapping Scalars to Scalars<br />
(player statistics)</strong></p>
<pre class="programlisting"><code>hr:  65    # Home runs
avg: 0.278 # Batting average
rbi: 147   # Runs Batted In</code></pre>
</div></td>
</tr>
<tr>
<td><div class="example">
<span id="id857390"></span>
<p><strong>Example 2.3.  Mapping Scalars to Sequences<br />
(ball clubs in each league)</strong></p>
<pre class="programlisting"><code>american:
  - Boston Red Sox
  - Detroit Tigers
  - New York Yankees
national:
  - New York Mets
  - Chicago Cubs
  - Atlanta Braves</code></pre>
</div></td>
<td><div class="example">
<span id="id857416"></span>
<p><strong>Example 2.4.  Sequence of Mappings<br />
(players’ statistics)</strong></p>
<pre class="programlisting"><code>-
  name: Mark McGwire
  hr:   65
  avg:  0.278
-
  name: Sammy Sosa
  hr:   63
  avg:  0.288</code></pre>
</div></td>
</tr>
</tbody>
</table>

YAML also has <span id="id857442" class="indexterm"></span>[flow styles](#flow%20style/information%20model), using explicit <span id="id857460" class="indexterm"></span>[indicators](#indicator/) rather than <span id="id857471" class="indexterm"></span>[indentation](#indentation%20space/) to denote scope. The <span id="id857485" class="indexterm"></span>[flow sequence](#flow%20sequence%20style/information%20model) is written as a comma separated list within square brackets. In a similar manner, the <span id="id857505" class="indexterm"></span>[flow mapping](#flow%20mapping%20style/information%20model) uses curly braces.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id857532"></span>
<p><strong>Example 2.5. Sequence of Sequences</strong></p>
<pre class="programlisting"><code>- [name        , hr, avg  ]
- [Mark McGwire, 65, 0.278]
- [Sammy Sosa  , 63, 0.288]
&#10;</code></pre>
</div></td>
<td><div class="example">
<span id="id857555"></span>
<p><strong>Example 2.6. Mapping of Mappings</strong></p>
<pre class="programlisting"><code>Mark McGwire: {hr: 65, avg: 0.278}
Sammy Sosa: {
    hr: 63,
    avg: 0.288
  }</code></pre>
</div></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id857577"></span>2.2. Structures

</div>

</div>

</div>

YAML uses three dashes (<span id="id857587" class="indexterm"></span>[“<span class="quote">**`---`**</span>”](#document%20boundary%20marker/)) to separate <span id="id857607" class="indexterm"></span>[documents](#document/information%20model) within a <span id="id857621" class="indexterm"></span>[stream](#stream/information%20model). Three dots ( <span id="id857629" class="indexterm"></span>[“<span class="quote">**`...`**</span>”](#document%20boundary%20marker/)) indicate the end of a document without starting a new one, for use in communication channels. <span id="id857658" class="indexterm"></span>[Comment](#comment/information%20model) lines begin with the Octothorpe (also called “<span class="quote">hash</span>”, “<span class="quote">sharp</span>”, or “<span class="quote">number sign</span>” - <span id="id857687" class="indexterm"></span>[“<span class="quote">**`#`**</span>”](##%20comment/)).

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id857714"></span>
<p><strong>Example 2.7.  Two Documents in a Stream<br />
(each with a leading comment)</strong></p>
<pre class="programlisting"><code># Ranking of 1998 home runs
---
- Mark McGwire
- Sammy Sosa
- Ken Griffey
&#10;# Team ranking
---
- Chicago Cubs
- St Louis Cardinals</code></pre>
</div></td>
<td><div class="example">
<span id="id857738"></span>
<p><strong>Example 2.8.  Play by Play Feed<br />
from a Game</strong></p>
<pre class="programlisting"><code>---
time: 20:03:20
player: Sammy Sosa
action: strike (miss)
...
---
time: 20:03:47
player: Sammy Sosa
action: grand slam
...</code></pre>
</div></td>
</tr>
</tbody>
</table>

Repeated <span id="id857766" class="indexterm"></span>[nodes](#node/information%20model) are first <span id="id857785" class="indexterm"></span>[identified](#identified/) by an <span id="id857797" class="indexterm"></span>[anchor](#anchor/information%20model) (marked with the ampersand - <span id="id857816" class="indexterm"></span>[“<span class="quote">**`&`**</span>”](#&%20anchor/)), and are then <span id="id857834" class="indexterm"></span>[aliased](#alias/information%20model) (referenced with an asterisk - <span id="id857852" class="indexterm"></span>[“<span class="quote">**`*`**</span>”](#*%20alias/)) thereafter.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id857879"></span>
<p><strong>Example 2.9.  Single Document with<br />
Two Comments</strong></p>
<pre class="programlisting"><code>---
hr: # 1998 hr ranking
  - Mark McGwire
  - Sammy Sosa
rbi:
  # 1998 rbi ranking
  - Sammy Sosa
  - Ken Griffey</code></pre>
</div></td>
<td><div class="example">
<span id="id857905"></span>
<p><strong>Example 2.10.  Node for “<span class="quote"><strong><code>Sammy Sosa</code></strong></span>”<br />
appears twice in this document</strong></p>
<pre class="programlisting"><code>---
hr:
  - Mark McGwire
  # Following node labeled SS
  - &amp;SS Sammy Sosa
rbi:
  - *SS # Subsequent occurrence
  - Ken Griffey</code></pre>
</div></td>
</tr>
</tbody>
</table>

A question mark and space <span id="id857930" class="indexterm"></span>[(“<span class="quote">**`? `**</span>”](#?%20mapping%20key/)) indicate a complex <span id="id857962" class="indexterm"></span>[mapping key](#key/information%20model). Within a <span id="id857977" class="indexterm"></span>[block collection](#block%20collection%20style/information%20model), <span id="id857993" class="indexterm"></span>[key:](#key/information%20model) <span id="id858010" class="indexterm"></span>[value](#value/information%20model) pairs can start immediately following the dash, colon, or question mark.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id858035"></span>
<p><strong>Example 2.11. Mapping between Sequences</strong></p>
<pre class="programlisting"><code>? - Detroit Tigers
  - Chicago cubs
:
  - 2001-07-23
&#10;? [ New York Yankees,
    Atlanta Braves ]
: [ 2001-07-02, 2001-08-12,
    2001-08-14 ]</code></pre>
</div></td>
<td><div class="example">
<span id="id858058"></span>
<p><strong>Example 2.12. In-Line Nested Mapping</strong></p>
<pre class="programlisting"><code>---
# products purchased
- item    : Super Hoop
  quantity: 1
- item    : Basketball
  quantity: 4
- item    : Big Shoes
  quantity: 1
</code></pre>
</div></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id858081"></span>2.3. Scalars

</div>

</div>

</div>

<span id="id858089" class="indexterm"></span>[Scalar content](#scalar/information%20model) can be written in <span id="id858105" class="indexterm"></span>[block](#block%20style/information%20model) form, using a <span id="id858121" class="indexterm"></span>[literal style](#literal%20style/information%20model) (<span id="id858137" class="indexterm"></span>[“<span class="quote">**`|`**</span>”](#%7C%20literal%20style/)) where all <span id="id858158" class="indexterm"></span>[line breaks](#line%20break%20character/) are significant. Alternatively, they can be written with the <span id="id858173" class="indexterm"></span>[folded style](#folded%20style/information%20model) <span id="id858190" class="indexterm"></span>[(“<span class="quote">**`>`**</span>”](#%3E%20folded%20style/)) where each <span id="id858210" class="indexterm"></span>[line break](#line%20break%20character/) is <span id="id858222" class="indexterm"></span>[folded](#line%20folding/) to a space unless it ends an <span id="id858234" class="indexterm"></span>[empty](#empty%20line/) or a <span id="id858247" class="indexterm"></span>[“<span class="quote">more indented</span>” line](#more%20indented%20line/).

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id858273"></span>
<p><strong>Example 2.13.  In literals,<br />
newlines are preserved</strong></p>
<pre class="programlisting"><code># ASCII Art
--- |
  \//||\/||
  // ||  ||__</code></pre>
</div></td>
<td><div class="example">
<span id="id858298"></span>
<p><strong>Example 2.14.  In the plain scalar,<br />
newlines become spaces</strong></p>
<pre class="programlisting"><code>---
  Mark McGwire&#39;s
  year was crippled
  by a knee injury.</code></pre>
</div></td>
</tr>
<tr>
<td><div class="example">
<span id="id858323"></span>
<p><strong>Example 2.15.  Folded newlines are preserved<br />
for "more indented" and blank lines</strong></p>
<pre class="programlisting"><code>&gt;
 Sammy Sosa completed another
 fine season with great stats.
&#10;   63 Home Runs
   0.288 Batting Average
&#10; What a year!</code></pre>
</div></td>
<td><div class="example">
<span id="id858350"></span>
<p><strong>Example 2.16.  Indentation determines scope<br />
 </strong></p>
<pre class="programlisting"><code>name: Mark McGwire
accomplishment: &gt;
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average
</code></pre>
</div></td>
</tr>
</tbody>
</table>

YAML’s <span id="id858382" class="indexterm"></span>[flow scalars](#flow%20scalar%20style/information%20model) include the <span id="id858402" class="indexterm"></span>[plain style](#plain%20style/information%20model) (most examples thus far) and <span id="id858420" class="indexterm"></span>[quoted styles](#quoted%20style/information%20model). The <span id="id858436" class="indexterm"></span>[double-quoted style](#double-quoted%20style/information%20model) provides <span id="id858453" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/). The <span id="id858467" class="indexterm"></span>[single-quoted style](#single-quoted%20style/information%20model) is useful when <span id="id858486" class="indexterm"></span>[escaping](#escaping%20in%20double-quoted%20style/) is not needed. All <span id="id858499" class="indexterm"></span>[flow scalars](#flow%20scalar%20style/information%20model) can span multiple lines; <span id="id858516" class="indexterm"></span>[line breaks](#line%20break%20character/) are always <span id="id858529" class="indexterm"></span>[folded](#line%20folding/).

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id858553"></span>
<p><strong>Example 2.17. Quoted Scalars</strong></p>
<pre class="programlisting"><code>unicode: &quot;Sosa did fine.\u263A&quot;
control: &quot;\b1998\t1999\t2000\n&quot;
hexesc:  &quot;\x13\x10 is \r\n&quot;
&#10;single: &#39;&quot;Howdy!&quot; he cried.&#39;
quoted: &#39; # not a &#39;&#39;comment&#39;&#39;.&#39;
tie-fighter: &#39;|\-*-/|&#39;</code></pre>
</div></td>
<td><div class="example">
<span id="id858577"></span>
<p><strong>Example 2.18. Multi-line Flow Scalars</strong></p>
<pre class="programlisting"><code>plain:
  This unquoted scalar
  spans many lines.
&#10;quoted: &quot;So does this
  quoted scalar.\n&quot;
</code></pre>
</div></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id858600"></span>2.4. Tags

</div>

</div>

</div>

In YAML, <span id="id858608" class="indexterm"></span>[untagged nodes](#non-specific%20tag/) are given an type depending on the <span id="id858622" class="indexterm"></span>[application](#application/). The examples in this specification generally use the <a href="/type/seq.html" target="_top">“<span class="quote"><strong><code>seq</code></strong></span>”</a>, <a href="/type/map.html" target="_top">“<span class="quote"><strong><code>map</code></strong></span>”</a> and <a href="/type/str.html" target="_top">“<span class="quote"><strong><code>str</code></strong></span>”</a> types from the <a href="/type/index.html" target="_top">YAML tag repository</a>. A few examples also use the <a href="/type/int.html" target="_top">“<span class="quote"><strong><code>int</code></strong></span>”</a> and <a href="/type/float.html" target="_top">“<span class="quote"><strong><code>float</code></strong></span>”</a> types. The repository includes additional types such as <a href="/type/null.html" target="_top">“<span class="quote"><strong><code>null</code></strong></span>”</a>, <a href="/type/bool.html" target="_top">“<span class="quote"><strong><code>bool</code></strong></span>”</a>, <a href="/type/set.html" target="_top">“<span class="quote"><strong><code>set</code></strong></span>”</a> and others.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id858734"></span>
<p><strong>Example 2.19. Integers</strong></p>
<pre class="programlisting"><code>canonical: 12345
decimal: +12,345
sexagesimal: 3:25:45
octal: 014
hexadecimal: 0xC
</code></pre>
</div></td>
<td><div class="example">
<span id="id858757"></span>
<p><strong>Example 2.20. Floating Point</strong></p>
<pre class="programlisting"><code>canonical: 1.23015e+3
exponential: 12.3015e+02
sexagesimal: 20:30.15
fixed: 1,230.15
negative infinity: -.inf
not a number: .NaN</code></pre>
</div></td>
</tr>
<tr>
<td><div class="example">
<span id="id858780"></span>
<p><strong>Example 2.21. Miscellaneous</strong></p>
<pre class="programlisting"><code>null: ~
true: y
false: n
string: &#39;12345&#39;</code></pre>
</div></td>
<td><div class="example">
<span id="id858801"></span>
<p><strong>Example 2.22. Timestamps</strong></p>
<pre class="programlisting"><code>canonical: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14</code></pre>
</div></td>
</tr>
</tbody>
</table>

Explicit typing is denoted with a <span id="id858826" class="indexterm"></span>[tag](#tag/information%20model) using the exclamation point (<span id="id858842" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20tag%20indicator/)) symbol. <span id="id858862" class="indexterm"></span>[Global tags](#global%20tag/) are URIs and may be specified in a <span id="id858873" class="indexterm"></span>[shorthand](#tag%20shorthand/) form using a <span id="id858888" class="indexterm"></span>[handle](#tag%20handle/). <span id="id858901" class="indexterm"></span>[Application](#application/)-specific <span id="id858913" class="indexterm"></span>[local tags](#local%20tag/) may also be used.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id858936"></span>
<p><strong>Example 2.23. Various Explicit Tags</strong></p>
<pre class="programlisting"><code>---
not-date: !!str 2002-04-28
&#10;picture: !!binary |
 R0lGODlhDAAMAIQAAP//9/X
 17unp5WZmZgAAAOfn515eXv
 Pz7Y6OjuDg4J+fn5OTk6enp
 56enmleECcgggoBADs=
&#10;application specific tag: !something |
 The semantics of the tag
 above may be different for
 different documents.
</code></pre>
</div></td>
<td><div class="example">
<span id="id858961"></span>
<p><strong>Example 2.24. Global Tags</strong></p>
<pre class="programlisting"><code>%TAG ! tag:clarkevans.com,2002:
--- !shape
  # Use the ! handle for presenting
  # tag:clarkevans.com,2002:circle
- !circle
  center: &amp;ORIGIN {x: 73, y: 129}
  radius: 7
- !line
  start: *ORIGIN
  finish: { x: 89, y: 102 }
- !label
  start: *ORIGIN
  color: 0xFFEEBB
  text: Pretty vector drawing.</code></pre>
</div></td>
</tr>
</tbody>
</table>

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id858993"></span>
<p><strong>Example 2.25. Unordered Sets</strong></p>
<pre class="programlisting"><code># sets are represented as a
# mapping where each key is
# associated with the empty string
--- !!set
? Mark McGwire
? Sammy Sosa
? Ken Griff</code></pre>
</div></td>
<td><div class="example">
<span id="id859017"></span>
<p><strong>Example 2.26. Ordered Mappings</strong></p>
<pre class="programlisting"><code># ordered maps are represented as
# a sequence of mappings, with
# each mapping having one key
--- !!omap
- Mark McGwire: 65
- Sammy Sosa: 63
- Ken Griffy: 58</code></pre>
</div></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id859040"></span>2.5. Full Length Example

</div>

</div>

</div>

Below are two full-length examples of YAML. On the left is a sample invoice; on the right is a sample log file.

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><div class="example">
<span id="id859060"></span>
<p><strong>Example 2.27. Invoice</strong></p>
<pre class="programlisting"><code>--- !&lt;tag:clarkevans.com,2002:invoice&gt;
invoice: 34843
date   : 2001-01-23
bill-to: &amp;id001
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
    Billsmer @ 338-4338.</code></pre>
</div></td>
<td><div class="example">
<span id="id859081"></span>
<p><strong>Example 2.28. Log File</strong></p>
<pre class="programlisting"><code>---
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
  Unknown variable &quot;bar&quot;
Stack:
  - file: TopClass.py
    line: 23
    code: |
      x = MoreObject(&quot;345\n&quot;)
  - file: MoreClass.py
    line: 58
    code: |-
      foo = bar
&#10;
</code></pre>
</div></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id859109"></span>Chapter 3. Processing YAML Information

</div>

</div>

</div>

YAML is both a text format and a method for <span id="id859118" class="indexterm"></span>[presenting](#present/) any data structure in this format. Therefore, this specification defines two concepts: a class of data objects called YAML <span id="id859132" class="indexterm"></span>[representations](#representation/), and a syntax for <span id="id859145" class="indexterm"></span>[presenting](#present/) YAML <span id="id859158" class="indexterm"></span>[representations](#representation/) as a series of characters, called a YAML <span id="id859171" class="indexterm"></span>[stream](#stream/information%20model). A YAML <span id="id859187" class="indexterm"></span><span id="processor/"></span>*processor* is a tool for converting information between these complementary views. It is assumed that a YAML processor does its work on behalf of another module, called an <span id="id859203" class="indexterm"></span><span id="application/"></span>*application*. This chapter describes the information structures a YAML processor must provide to or obtain from the application.

YAML information is used in two ways: for machine processing, and for human consumption. The challenge of reconciling these two perspectives is best done in three distinct translation stages: <span id="id859226" class="indexterm"></span>[representation](#representation/), <span id="id859238" class="indexterm"></span>[serialization](#serialization/), and <span id="id859251" class="indexterm"></span>[presentation](#presentation/). <span id="id859264" class="indexterm"></span>[Representation](#representation/) addresses how YAML views native data structures to achieve portability between programming environments. <span id="id859279" class="indexterm"></span>[Serialization](#serialization/) concerns itself with turning a YAML <span id="id859292" class="indexterm"></span>[representation](#representation/) into a serial form, that is, a form with sequential access constraints. <span id="id859305" class="indexterm"></span>[Presentation](#presentation/) deals with the formatting of a YAML <span id="id859318" class="indexterm"></span>[serialization](#serialization/) as a series of characters in a human-friendly manner.

<div class="figure">

<span id="id859333"></span>

**Figure 3.1. Processing Overview**

<div class="mediaobject">

![Processing Overview](overview2.png)

</div>

</div>

A YAML processor need not expose the <span id="id859358" class="indexterm"></span>[serialization](#serialization/) or <span id="id859370" class="indexterm"></span>[representation](#representation/) stages. It may translate directly between native data structures and a character <span id="id859384" class="indexterm"></span>[stream](#stream/information%20model) (<span id="id859400" class="indexterm"></span><span id="dump/"></span>*dump* and <span id="id859414" class="indexterm"></span><span id="load/"></span>*load* in the diagram above). However, such a direct translation should take place so that the native data structures are <span id="id859430" class="indexterm"></span>[constructed](#construct/) only from information available in the <span id="id859444" class="indexterm"></span>[representation](#representation/).

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id859458"></span>3.1. Processes

</div>

</div>

</div>

This section details the processes shown in the diagram above. Note a YAML <span id="id859467" class="indexterm"></span>[processor](#processor/) need not provide all these processes. For example, a YAML library may provide only YAML input ability, for loading configuration files, or only output ability, for sending data to other <span id="id859483" class="indexterm"></span>[applications](#application/).

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id859497"></span>3.1.1. Represent

</div>

</div>

</div>

YAML <span id="id859505" class="indexterm"></span><span id="represent/"></span>*represents* any native data structure using three <span id="id859519" class="indexterm"></span>[node kinds](#kind/): <span id="id859533" class="indexterm"></span>[sequence](#sequence/information%20model) - an ordered series of entries; <span id="id859551" class="indexterm"></span>[mapping](#mapping/information%20model) - an unordered association of <span id="id859566" class="indexterm"></span>[unique](#equality/) <span id="id859579" class="indexterm"></span>[keys](#key/information%20model) to <span id="id859595" class="indexterm"></span>[values](#value/information%20model); and <span id="id859610" class="indexterm"></span>[scalar](#scalar/information%20model) - any datum with opaque structure <span id="id859629" class="indexterm"></span>[presentable](#present/) as a series of Unicode characters. Combined, these primitives generate directed graph structures. These primitives were chosen because they are both powerful and familiar: the <span id="id859643" class="indexterm"></span>[sequence](#sequence/information%20model) corresponds to a Perl array and a Python list, the <span id="id859659" class="indexterm"></span>[mapping](#mapping/information%20model) corresponds to a Perl hash table and a Python dictionary. The <span id="id859676" class="indexterm"></span>[scalar](#scalar/information%20model) represents strings, integers, dates, and other atomic data types.

Each YAML <span id="id859696" class="indexterm"></span>[node](#node/information%20model) requires, in addition to its <span id="id859714" class="indexterm"></span>[kind](#kind/) and <span id="id859725" class="indexterm"></span>[content](#content/information%20model), a <span id="id859741" class="indexterm"></span>[tag](#tag/information%20model) specifying its data type. Type specifiers are either <span id="id859757" class="indexterm"></span>[global](#global%20tag/) URIs, or are <span id="id859772" class="indexterm"></span>[local](#local%20tag/) in scope to a single <span id="id859785" class="indexterm"></span>[application](#application/). For example, an integer is represented in YAML with a <span id="id859797" class="indexterm"></span>[scalar](#scalar/information%20model) plus the <span id="id859813" class="indexterm"></span>[global tag](#global%20tag/) “<span class="quote">**`tag:yaml.org,2002:int`**</span>”. Similarly, an invoice object, particular to a given organization, could be represented as a <span id="id859833" class="indexterm"></span>[mapping](#mapping/information%20model) together with the <span id="id859852" class="indexterm"></span>[local tag](#local%20tag/) “<span class="quote">**`!invoice`**</span>”. This simple model can represent any data structure independent of programming language.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id859873"></span>3.1.2. Serialize

</div>

</div>

</div>

For sequential access mediums, such as an event callback API, a YAML <span id="id859882" class="indexterm"></span>[representation](#representation/) must be <span id="id859895" class="indexterm"></span><span id="serialize/"></span>*serialized* to an ordered tree. Since in a YAML <span id="id859909" class="indexterm"></span>[representation](#representation/), <span id="id859921" class="indexterm"></span>[mapping keys](#key/information%20model) are unordered and <span id="id859937" class="indexterm"></span>[nodes](#node/information%20model) may be referenced more than once (have more than one incoming “<span class="quote">arrow</span>”), the serialization process is required to impose an <span id="id859960" class="indexterm"></span>[ordering](#key%20order/) on the <span id="id859972" class="indexterm"></span>[mapping keys](#key/information%20model) and to replace the second and subsequent references to a given <span id="id859989" class="indexterm"></span>[node](#node/information%20model) with place holders called <span id="id860007" class="indexterm"></span>[aliases](#alias/information%20model). YAML does not specify how these <span id="id860022" class="indexterm"></span><span id="serialization detail/"></span>*serialization details* are chosen. It is up to the YAML <span id="id860039" class="indexterm"></span>[processor](#processor/) to come up with human-friendly <span id="id860050" class="indexterm"></span>[key order](#key%20order/) and <span id="id860062" class="indexterm"></span>[anchor](#anchor/information%20model) names, possibly with the help of the <span id="id860081" class="indexterm"></span>[application](#application/). The result of this process, a YAML <span id="id860092" class="indexterm"></span>[serialization tree](#serialization/), can then be traversed to produce a series of event calls for one-pass processing of YAML data.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id860109"></span>3.1.3. Present

</div>

</div>

</div>

The final output process is <span id="id860117" class="indexterm"></span><span id="present/"></span>*presenting* the YAML <span id="id860130" class="indexterm"></span>[serializations](#serialization/) as a character <span id="id860143" class="indexterm"></span>[stream](#stream/information%20model) in a human-friendly manner. To maximize human readability, YAML offers a rich set of stylistic options which go far beyond the minimal functional needs of simple data storage. Therefore the YAML <span id="id860165" class="indexterm"></span>[processor](#processor/) is required to introduce various <span id="id860176" class="indexterm"></span><span id="presentation detail/"></span>*presentation details* when creating the <span id="id860193" class="indexterm"></span>[stream](#stream/information%20model), such as the choice of <span id="id860208" class="indexterm"></span>[node styles](#style/), how to <span id="id860221" class="indexterm"></span>[format content](#format/), the amount of <span id="id860234" class="indexterm"></span>[indentation](#indentation%20space/), which <span id="id860248" class="indexterm"></span>[tag handles](#tag%20handle/) to use, the <span id="id860261" class="indexterm"></span>[node tags](#tag/information%20model) to leave <span id="id860277" class="indexterm"></span>[unspecified](#non-specific%20tag/), the set of <span id="id860292" class="indexterm"></span>[directives](#directive/information%20model) to provide and possibly even what <span id="id860308" class="indexterm"></span>[comments](#comment/information%20model) to add. While some of this can be done with the help of the <span id="id860326" class="indexterm"></span>[application](#application/), in general this process should be guided by the preferences of the user.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id860341"></span>3.1.4. Parse

</div>

</div>

</div>

<span id="id860348" class="indexterm"></span><span id="parse/"></span>*Parsing* is the inverse process of <span id="id860362" class="indexterm"></span>[presentation](#present/), it takes a <span id="id860375" class="indexterm"></span>[stream](#stream/information%20model) of characters and produces a series of events. Parsing discards all the <span id="id860394" class="indexterm"></span>[details](#presentation%20detail/) introduced in the <span id="id860409" class="indexterm"></span>[presentation](#present/) process, reporting only the <span id="id860420" class="indexterm"></span>[serialization](#serialization/) events. Parsing can fail due to <span id="id860433" class="indexterm"></span>[ill-formed](#ill-formed%20stream/) input.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id860452"></span>3.1.5. Compose

</div>

</div>

</div>

<span id="id860459" class="indexterm"></span><span id="compose/"></span>*Composing* takes a series of <span id="id860471" class="indexterm"></span>[serialization](#serialization/) events and produces a <span id="id860484" class="indexterm"></span>[representation graph](#representation/). Composing discards all the <span id="id860497" class="indexterm"></span>[serialization details](#serialization%20detail/) introduced in the <span id="id860512" class="indexterm"></span>[serialization](#serialize/) process, producing only the <span id="id860525" class="indexterm"></span>[representation graph](#representation/). Composing can fail due to any of several reasons, detailed <span id="id860539" class="indexterm"></span>[below](#load%20failure%20point/).

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id860557"></span>3.1.6. Construct

</div>

</div>

</div>

The final input process is <span id="id860565" class="indexterm"></span><span id="construct/"></span>*constructing* native data structures from the YAML <span id="id860579" class="indexterm"></span>[representation](#representation/). Construction must be based only on the information available in the <span id="id860593" class="indexterm"></span>[representation](#representation/), and not on additional <span id="id860606" class="indexterm"></span>[serialization](#serialization/) or <span id="id860618" class="indexterm"></span>[presentation details](#presentation%20detail/) such as <span id="id860632" class="indexterm"></span>[comments](#comment/information%20model), <span id="id860648" class="indexterm"></span>[directives](#directive/information%20model), <span id="id860665" class="indexterm"></span>[mapping key order](#key%20order/), <span id="id860676" class="indexterm"></span>[node styles](#style/), <span id="id860688" class="indexterm"></span>[content format](#format/), <span id="id860700" class="indexterm"></span>[indentation](#indentation%20space/) levels etc. Construction can fail due to the <span id="id860715" class="indexterm"></span>[unavailability](#unavailable%20tag/) of the required native data types.

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id860735"></span>3.2. Information Models

</div>

</div>

</div>

This section specifies the formal details of the results of the above processes. To maximize data portability between programming languages and implementations, users of YAML should be mindful of the distinction between <span id="id860747" class="indexterm"></span>[serialization](#serialization/) or <span id="id860758" class="indexterm"></span>[presentation](#presentation/) properties and those which are part of the YAML <span id="id860771" class="indexterm"></span>[representation](#representation/). Thus, while imposing a <span id="id860784" class="indexterm"></span>[order](#key%20order/) on <span id="id860796" class="indexterm"></span>[mapping keys](#key/information%20model) is necessary for flattening YAML <span id="id860813" class="indexterm"></span>[representations](#representation/) to a sequential access medium, this <span id="id860825" class="indexterm"></span>[serialization detail](#serialization%20detail/) must not be used to convey <span id="id860841" class="indexterm"></span>[application](#application/) level information. In a similar manner, while <span id="id860854" class="indexterm"></span>[indentation](#indentation%20space/) technique and a choice of a <span id="id860869" class="indexterm"></span>[node style](#style/) are needed for the human readability, these <span id="id860881" class="indexterm"></span>[presentation details](#presentation%20detail/) are neither part of the YAML <span id="id860894" class="indexterm"></span>[serialization](#serialization/) nor the YAML <span id="id860906" class="indexterm"></span>[representation](#representation/). By carefully separating properties needed for <span id="id860920" class="indexterm"></span>[serialization](#serialization/) and <span id="id860932" class="indexterm"></span>[presentation](#presentation/), YAML <span id="id860945" class="indexterm"></span>[representations](#representation/) of <span id="id860957" class="indexterm"></span>[application](#application/) information will be consistent and portable between various programming environments.

The following diagram summarizes the three information models. Full arrows denote composition, hollow arrows denote inheritance, “<span class="quote">**`1`**</span>” and “<span class="quote">**`*`**</span>” denote “<span class="quote">one</span>” and “<span class="quote">many</span>” relationships. A single “<span class="quote">**`+`**</span>” denotes <span id="id861005" class="indexterm"></span>[serialization](#serialization/) details, a double “<span class="quote">**`++`**</span>” denotes <span id="id861025" class="indexterm"></span>[presentation](#presentation/) details.

<div class="figure">

<span id="id861038"></span>

**Figure 3.2. Information Models**

<div class="mediaobject">

![Information Models](model2.png)

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id861060"></span>3.2.1. Representation Graph

</div>

</div>

</div>

YAML’s <span id="id861069" class="indexterm"></span><span id="representation/"></span>*representation* of native data is a rooted, connected, directed graph of <span id="id861083" class="indexterm"></span>[tagged](#tag/information%20model) <span id="id861099" class="indexterm"></span>[nodes](#node/information%20model). By “<span class="quote">directed graph</span>” we mean a set of <span id="id861118" class="indexterm"></span>[nodes](#node/information%20model) and directed edges (“<span class="quote">arrows</span>”), where each edge connects one <span id="id861138" class="indexterm"></span>[node](#node/information%20model) to another (see <a href="http://www.nist.gov/dads/HTML/directedGraph.html" target="_top">a formal definition</a>). All the <span id="id861161" class="indexterm"></span>[nodes](#node/information%20model) must be reachable from the <span id="id861178" class="indexterm"></span><span id="root node/"></span>*root node* via such edges. Note that the YAML graph may include cycles, and a <span id="id861192" class="indexterm"></span>[node](#node/information%20model) may have more than one incoming edge.

<span id="id861213" class="indexterm"></span>[Nodes](#node/information%20model) that are defined in terms of other <span id="id861229" class="indexterm"></span>[nodes](#node/information%20model) are <span id="id861245" class="indexterm"></span>[collections](#collection/information%20model) and <span id="id861262" class="indexterm"></span>[nodes](#node/information%20model) that are independent of any other <span id="id861277" class="indexterm"></span>[nodes](#node/information%20model) are <span id="id861294" class="indexterm"></span>[scalars](#scalar/information%20model). YAML supports two <span id="id861308" class="indexterm"></span>[kinds](#kind/) of <span id="id861320" class="indexterm"></span>[collection nodes](#collection/information%20model), <span id="id861336" class="indexterm"></span>[sequences](#sequence/information%20model) and <span id="id861354" class="indexterm"></span>[mappings](#mapping/information%20model). <span id="id861368" class="indexterm"></span>[Mapping nodes](#mapping/information%20model) are somewhat tricky because their <span id="id861384" class="indexterm"></span>[keys](#key/information%20model) are unordered and must be <span id="id861400" class="indexterm"></span>[unique](#equality/).

<div class="figure">

<span id="id861413"></span>

**Figure 3.3. Representation Model**

<div class="mediaobject">

![Representation Model](represent2.png)

</div>

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id861435"></span>3.2.1.1. Nodes

</div>

</div>

</div>

YAML <span id="id861443" class="indexterm"></span><span id="node/information model"></span>*nodes* have <span id="id861462" class="indexterm"></span><span id="content/information model"></span>*content* of one of three <span id="id861478" class="indexterm"></span><span id="kind/"></span>*kinds*: scalar, sequence, or mapping. In addition, each node has a <span id="id861492" class="indexterm"></span>[tag](#tag/information%20model) which serves to restrict the set of possible values which the node’s content can have.

<div class="variablelist">

<span class="term">Scalar</span>  
The content of a <span id="id861523" class="indexterm"></span><span id="scalar/information model"></span>*scalar* node is an opaque datum that can be <span id="id861541" class="indexterm"></span>[presented](#present/) as a series of zero or more Unicode characters.

<span class="term">Sequence</span>  
The content of a <span id="id861565" class="indexterm"></span><span id="sequence/information model"></span>*sequence* node is an ordered series of zero or more nodes. In particular, a sequence may contain the same node more than once or it could even contain itself (directly or indirectly).

<span class="term">Mapping</span>  
The content of a <span id="id861597" class="indexterm"></span><span id="mapping/information model"></span>*mapping* node is an unordered set of <span id="id861614" class="indexterm"></span><span id="key/information model"></span>*key:* <span id="id861633" class="indexterm"></span><span id="value/information model"></span>*value* node pairs, with the restriction that each of the keys is <span id="id861650" class="indexterm"></span>[unique](#equality/). YAML places no further restrictions on the nodes. In particular, keys may be arbitrary nodes, the same node may be used as the value of several key: value pairs, and a mapping could even contain itself as a key or a value (directly or indirectly).

</div>

When appropriate, it is convenient to consider sequences and mappings together, as <span id="id861679" class="indexterm"></span><span id="collection/information model"></span>*collections*. In this view, sequences are treated as mappings with integer keys starting at zero. Having a unified collections view for sequences and mappings is helpful both for creating practical YAML tools and APIs and for theoretical analysis.

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id861700"></span>3.2.1.2. Tags

</div>

</div>

</div>

YAML <span id="id861708" class="indexterm"></span>[represents](#represent/) type information of native data structures with a simple identifier, called a <span id="id861722" class="indexterm"></span><span id="tag/information model"></span>*tag*. <span id="id861741" class="indexterm"></span><span id="global tag/"></span>*Global tags* are <a href="http://www.ietf.org/rfc/rfc2396.txt" target="_top">URIs</a> and hence globally unique across all <span id="id861760" class="indexterm"></span>[applications](#application/). The “<span class="quote">**`tag`**</span>”: <a href="http://www.taguri.org" target="_top">URI scheme</a> (<a href="/spec/taguri.txt" target="_top">mirror</a>) is recommended for all global YAML tags. In contrast, <span id="id861792" class="indexterm"></span><span id="local tag/"></span>*local tags* are specific to a single <span id="id861806" class="indexterm"></span>[application](#application/). Local tags start with <span id="id861819" class="indexterm"></span><span id="! local tag/"></span>*“<span class="quote">**`!`**</span>”*, are not URIs and are not expected to be globally unique. YAML provides a <span id="id861841" class="indexterm"></span>[“<span class="quote">**`TAG`**</span>” directive](#TAG%20directive/) to make tag notation less verbose; it also offers easy migration from local to global tags. To ensure this, local tags are restricted to the URI character set and use URI character <span id="id861861" class="indexterm"></span>[escaping](#escaping%20in%20URI/).

YAML does not mandate any special relationship between different tags that begin with the same substring. Tags ending with URI fragments (containing <span id="id861882" class="indexterm"></span>[“<span class="quote">**`#`**</span>”](##%20comment/)) are no exception; tags that share the same base URI but differ in their fragment part are considered to be different, independent tags. By convention, fragments are used to identify different “<span class="quote">variants</span>” of a tag, while “<span class="quote">**`/`**</span>” is used to define nested tag “<span class="quote">namespace</span>” hierarchies. However, this is merely a convention, and each tag may employ its own rules. For example, Perl tags may use “<span class="quote">**`::`**</span>” to express namespace hierarchies, Java tags may use “<span class="quote">**`.`**</span>”, etc.

YAML tags are used to associate meta information with each <span id="id861941" class="indexterm"></span>[node](#node/information%20model). In particular, each tag must specify the expected <span id="id861957" class="indexterm"></span>[node kind](#kind/) (<span id="id861968" class="indexterm"></span>[scalar](#scalar/information%20model), <span id="id861984" class="indexterm"></span>[sequence](#sequence/information%20model), or <span id="id862001" class="indexterm"></span>[mapping](#mapping/information%20model)). <span id="id862016" class="indexterm"></span>[Scalar](#scalar/information%20model) tags must also provide mechanism for converting <span id="id862032" class="indexterm"></span>[formatted content](#format/) to a <span id="id862044" class="indexterm"></span>[canonical form](#canonical%20form/) for supporting <span id="id862057" class="indexterm"></span>[equality](#equality/) testing. Furthermore, a tag may provide additional information such as the set of allowed <span id="id862071" class="indexterm"></span>[content values](#content/information%20model) for validation, a mechanism for <span id="id862090" class="indexterm"></span>[tag resolution](#tag%20resolution/), or any other data that is applicable to all of the tag’s <span id="id862105" class="indexterm"></span>[nodes](#node/information%20model).

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id862121"></span>3.2.1.3. Nodes Comparison

</div>

</div>

</div>

Since YAML <span id="id862129" class="indexterm"></span>[mappings](#mapping/information%20model) require <span id="id862147" class="indexterm"></span>[key](#key/information%20model) uniqueness, <span id="id862161" class="indexterm"></span>[representations](#representation/) must include a mechanism for testing the equality of <span id="id862174" class="indexterm"></span>[nodes](#node/information%20model). This is non-trivial since YAML allows various ways to <span id="id862191" class="indexterm"></span>[format](#format/) a given <span id="id862203" class="indexterm"></span>[scalar content](#scalar/information%20model). For example, the integer eleven can be written as “<span class="quote">**`013`**</span>” (octal) or “<span class="quote">**`0xB`**</span>” (hexadecimal). If both forms are used as <span id="id862234" class="indexterm"></span>[keys](#key/information%20model) in the same <span id="id862252" class="indexterm"></span>[mapping](#mapping/information%20model), only a YAML <span id="id862266" class="indexterm"></span>[processor](#processor/) which recognizes integer <span id="id862279" class="indexterm"></span>[formats](#format/) would correctly flag the duplicate <span id="id862292" class="indexterm"></span>[key](#key/information%20model) as an error.

<div class="variablelist">

<span class="term">Canonical Form</span>  
YAML supports the need for <span id="id862321" class="indexterm"></span>[scalar](#scalar/information%20model) equality by requiring that every <span id="id862337" class="indexterm"></span>[scalar](#scalar/information%20model)<span id="id862352" class="indexterm"></span>[tag](#tag/information%20model) must specify a mechanism to producing the <span id="id862368" class="indexterm"></span><span id="canonical form/"></span>*canonical form* of any <span id="id862381" class="indexterm"></span>[formatted content](#format/). This form is a Unicode character string which <span id="id862396" class="indexterm"></span>[presents](#present/) the <span id="id862408" class="indexterm"></span>[content](#content/information%20model) and can be used for equality testing. While this requirement is stronger than a well defined equality operator, it has other uses, such as the production of digital signatures.

<span class="term">Equality</span>  
Two <span id="id862440" class="indexterm"></span>[nodes](#node/information%20model) must have the same <span id="id862457" class="indexterm"></span>[tag](#tag/information%20model) and <span id="id862471" class="indexterm"></span>[content](#content/information%20model) to be <span id="id862488" class="indexterm"></span><span id="equality/"></span>*equal*. Since each <span id="id862501" class="indexterm"></span>[tag](#tag/information%20model) applies to exactly one <span id="id862516" class="indexterm"></span>[kind](#kind/), this implies that the two <span id="id862529" class="indexterm"></span>[nodes](#node/information%20model) must have the same <span id="id862546" class="indexterm"></span>[kind](#kind/) to be equal. Two <span id="id862558" class="indexterm"></span>[scalars](#scalar/information%20model) are equal only when their <span id="id862577" class="indexterm"></span>[tags](#tag/information%20model) and canonical forms are equal character-by-character. Equality of <span id="id862593" class="indexterm"></span>[collections](#collection/information%20model) is defined recursively. Two <span id="id862610" class="indexterm"></span>[sequences](#sequence/information%20model) are equal only when they have the same <span id="id862627" class="indexterm"></span>[tag](#tag/information%20model) and length, and each <span id="id862642" class="indexterm"></span>[node](#node/information%20model) in one <span id="id862657" class="indexterm"></span>[sequence](#sequence/information%20model) is equal to the corresponding <span id="id862676" class="indexterm"></span>[node](#node/information%20model) in the other <span id="id862691" class="indexterm"></span>[sequence](#sequence/information%20model). Two <span id="id862706" class="indexterm"></span>[mappings](#mapping/information%20model) are equal only when they have the same <span id="id862724" class="indexterm"></span>[tag](#tag/information%20model) and an equal set of <span id="id862740" class="indexterm"></span>[keys](#key/information%20model), and each <span id="id862754" class="indexterm"></span>[key](#key/information%20model) in this set is associated with equal <span id="id862772" class="indexterm"></span>[values](#value/information%20model) in both <span id="id862788" class="indexterm"></span>[mappings](#mapping/information%20model).

<span class="term">Identity</span>  
Two <span id="id862812" class="indexterm"></span>[nodes](#node/information%20model) are <span id="id862830" class="indexterm"></span><span id="identity/"></span>*identical* only when they <span id="id862842" class="indexterm"></span>[represent](#represent/) the same native data structure. Typically, this corresponds to a single memory address. Identity should not be confused with equality; two equal <span id="id862858" class="indexterm"></span>[nodes](#node/information%20model) need not have the same identity. A YAML <span id="id862877" class="indexterm"></span>[processor](#processor/) may treat equal <span id="id862888" class="indexterm"></span>[scalars](#scalar/information%20model) as if they were identical. In contrast, the separate identity of two distinct but equal <span id="id862908" class="indexterm"></span>[collections](#collection/information%20model) must be preserved.

</div>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id862929"></span>3.2.2. Serialization Tree

</div>

</div>

</div>

To express a YAML <span id="id862938" class="indexterm"></span>[representation](#representation/) using a serial API, it necessary to impose an <span id="id862950" class="indexterm"></span>[order](#key%20order/) on <span id="id862964" class="indexterm"></span>[mapping keys](#key/information%20model) and employ <span id="id862978" class="indexterm"></span>[alias nodes](#alias/information%20model) to indicate a subsequent occurrence of a previously encountered <span id="id862995" class="indexterm"></span>[node](#node/information%20model). The result of this process is a <span id="id863013" class="indexterm"></span><span id="serialization/"></span>*serialization tree*, where each <span id="id863026" class="indexterm"></span>[node](#node/information%20model) has an ordered set of children. This tree can be traversed for a serial event-based API. <span id="id863043" class="indexterm"></span>[Construction](#construct/) of native structures from the serial interface should not use <span id="id863056" class="indexterm"></span>[key order](#key%20order/) or <span id="id863069" class="indexterm"></span>[anchors](#anchor/information%20model) for the preservation of important data.

<div class="figure">

<span id="id863089"></span>

**Figure 3.4. Serialization Model**

<div class="mediaobject">

![Serialization Model](serialize2.png)

</div>

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id863110"></span>3.2.2.1. Keys Order

</div>

</div>

</div>

In the <span id="id863118" class="indexterm"></span>[representation](#representation/) model, <span id="id863129" class="indexterm"></span>[mapping keys](#key/information%20model) do not have an order. To <span id="id863145" class="indexterm"></span>[serialize](#serialize/) a <span id="id863157" class="indexterm"></span>[mapping](#mapping/information%20model), it is necessary to impose an <span id="id863174" class="indexterm"></span><span id="key order/"></span>*ordering* on its <span id="id863189" class="indexterm"></span>[keys](#key/information%20model). This order is a <span id="id863204" class="indexterm"></span>[serialization detail](#serialization%20detail/) and should not be used when <span id="id863217" class="indexterm"></span>[composing](#compose/) the <span id="id863231" class="indexterm"></span>[representation graph](#representation/) (and hence for the preservation of important data). In every case where <span id="id863246" class="indexterm"></span>[node](#node/information%20model) order is significant, a <span id="id863264" class="indexterm"></span>[sequence](#sequence/information%20model) must be used. For example, an ordered <span id="id863279" class="indexterm"></span>[mapping](#mapping/information%20model) can be <span id="id863294" class="indexterm"></span>[represented](#represent/) as a <span id="id863306" class="indexterm"></span>[sequence](#sequence/information%20model) of <span id="id863322" class="indexterm"></span>[mappings](#mapping/information%20model), where each <span id="id863340" class="indexterm"></span>[mapping](#mapping/information%20model) is a single <span id="id863354" class="indexterm"></span>[key:](#key/information%20model) <span id="id863371" class="indexterm"></span>[value](#value/information%20model) pair. YAML provides convenient compact notation for this case.

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id863390"></span>3.2.2.2. Anchors and Aliases

</div>

</div>

</div>

In the <span id="id863397" class="indexterm"></span>[representation graph](#representation/), a <span id="id863410" class="indexterm"></span>[node](#node/information%20model) may appear in more than one <span id="id863428" class="indexterm"></span>[collection](#collection/information%20model). When <span id="id863444" class="indexterm"></span>[serializing](#serialize/) such data, the first occurrence of the <span id="id863456" class="indexterm"></span>[node](#node/information%20model) is <span id="id863473" class="indexterm"></span><span id="identified/"></span>*identified* by an <span id="id863486" class="indexterm"></span><span id="anchor/information model"></span>*anchor* and each subsequent occurrence is <span id="id863503" class="indexterm"></span>[serialized](#serialize/) as an <span id="id863515" class="indexterm"></span><span id="alias/information model"></span>*alias node* which refers back to this anchor. Otherwise, anchor names are a <span id="id863534" class="indexterm"></span>[serialization detail](#serialization%20detail/) and are discarded once <span id="id863548" class="indexterm"></span>[composing](#compose/) is completed. When <span id="id863561" class="indexterm"></span>[composing](#compose/) a <span id="id863573" class="indexterm"></span>[representation graph](#representation/) from <span id="id863586" class="indexterm"></span>[serialized](#serialize/) events, an alias node refers to the most recent <span id="id863599" class="indexterm"></span>[node](#node/information%20model) in the <span id="id863616" class="indexterm"></span>[serialization](#serialization/) having the specified anchor. Therefore, anchors need not be unique within a <span id="id863629" class="indexterm"></span>[serialization](#serialization/). In addition, an anchor need not have an alias node referring to it. It is therefore possible to provide an anchor for all <span id="id863644" class="indexterm"></span>[nodes](#node/information%20model) in <span id="id863660" class="indexterm"></span>[serialization](#serialization/).

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id863676"></span>3.2.3. Presentation Stream

</div>

</div>

</div>

A YAML <span id="id863684" class="indexterm"></span><span id="presentation/"></span>*presentation* is a <span id="id863698" class="indexterm"></span><span id="stream/information model"></span>*stream* of Unicode characters making use of of <span id="id863718" class="indexterm"></span>[styles](#style/), <span id="id863729" class="indexterm"></span>[formats](#format/), <span id="id863741" class="indexterm"></span>[comments](#comment/information%20model), <span id="id863757" class="indexterm"></span>[directives](#directive/information%20model) and other <span id="id863774" class="indexterm"></span>[presentation details](#presentation%20detail/) to <span id="id863787" class="indexterm"></span>[present](#present/) a YAML <span id="id863798" class="indexterm"></span>[serialization](#serialization/) in a human readable way. Although a YAML <span id="id863811" class="indexterm"></span>[processor](#processor/) may provide these <span id="id863824" class="indexterm"></span>[details](#presentation%20detail/) when <span id="id863839" class="indexterm"></span>[parsing](#parse/), they should not be reflected in the resulting <span id="id863852" class="indexterm"></span>[serialization](#serialization/). YAML allows several <span id="id863865" class="indexterm"></span>[serializations](#serialization/) to be contained in the same YAML character stream as a series of <span id="id863878" class="indexterm"></span><span id="document/information model"></span>*documents* separated by <span id="id863896" class="indexterm"></span>[document boundary markers](#document%20boundary%20marker/). Documents appearing in the same stream are independent; that is, a <span id="id863912" class="indexterm"></span>[node](#node/information%20model) must not appear in more than one <span id="id863928" class="indexterm"></span>[serialization tree](#serialization/) or <span id="id863940" class="indexterm"></span>[representation graph](#representation/).

<div class="figure">

<span id="id863954"></span>

**Figure 3.5. Presentation Model**

<div class="mediaobject">

![Presentation Model](present2.png)

</div>

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id863975"></span>3.2.3.1. Node Styles

</div>

</div>

</div>

Each <span id="id863984" class="indexterm"></span>[node](#node/information%20model) is presented in some <span id="id864002" class="indexterm"></span><span id="style/"></span>*style*, depending on its <span id="id864014" class="indexterm"></span>[kind](#kind/). The node style is a <span id="id864026" class="indexterm"></span>[presentation detail](#presentation%20detail/) and is not reflected in the <span id="id864040" class="indexterm"></span>[serialization tree](#serialization/) or <span id="id864053" class="indexterm"></span>[representation graph](#representation/). There are two groups of styles, <span id="id864067" class="indexterm"></span><span id="block style/information model"></span>*block* and <span id="id864084" class="indexterm"></span><span id="flow style/information model"></span>*flow*. Block styles use <span id="id864104" class="indexterm"></span>[indentation](#indentation%20space/) to denote nesting and scope within the <span id="id864117" class="indexterm"></span>[document](#document/information%20model). In contrast, flow styles rely on explicit <span id="id864133" class="indexterm"></span>[indicators](#indicator/) to denote nesting and scope.

YAML provides a rich set of <span id="id864149" class="indexterm"></span>[scalar styles](#scalar/information%20model). <span id="id864165" class="indexterm"></span><span id="block scalar style/information model"></span>*Block scalar styles* include the <span id="id864184" class="indexterm"></span><span id="literal style/information model"></span>*literal style* and the <span id="id864204" class="indexterm"></span><span id="folded style/information model"></span>*folded style*; <span id="id864220" class="indexterm"></span><span id="flow scalar style/information model"></span>*flow scalar styles* include the <span id="id864238" class="indexterm"></span><span id="plain style/information model"></span>*plain style* and two <span id="id864257" class="indexterm"></span><span id="quoted style/information model"></span>*quoted styles*, the <span id="id864275" class="indexterm"></span><span id="single-quoted style/information model"></span>*single-quoted style* and the <span id="id864293" class="indexterm"></span><span id="double-quoted style/information model"></span>*double-quoted style*. These styles offer a range of trade-offs between expressive power and readability.

Normally, the <span id="id864315" class="indexterm"></span>[content](#content/information%20model) of <span id="id864332" class="indexterm"></span><span id="block collection style/information model"></span>*block collections* begins on the next line. In most cases, YAML also allows block collections to start <span id="id864351" class="indexterm"></span><span id="in-line style/information model"></span>*in-line* for more compact notation when nesting <span id="id864369" class="indexterm"></span><span id="block sequence style/information model"></span>*block sequences* and <span id="id864388" class="indexterm"></span><span id="block mapping style/information model"></span>*block mappings* inside each other. When nesting <span id="id864408" class="indexterm"></span><span id="flow collection style/information model"></span>*flow collections*, a <span id="id864427" class="indexterm"></span><span id="flow mapping style/information model"></span>*flow mapping* with a <span id="id864445" class="indexterm"></span><span id="single pair style/information model"></span>*single key: value pair* may be specified directly inside a <span id="id864464" class="indexterm"></span><span id="flow sequence style/information model"></span>*flow sequence*, allowing for a compact “<span class="quote">ordered mapping</span>” notation.

<div class="figure">

<span id="id864487"></span>

**Figure 3.6. Kind/Style Combinations**

<div class="mediaobject">

![Kind/Style Combinations](styles2.png)

</div>

</div>

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id864510"></span>3.2.3.2. Scalar Formats

</div>

</div>

</div>

YAML allows <span id="id864518" class="indexterm"></span>[scalar content](#scalar/information%20model) to be <span id="id864536" class="indexterm"></span>[presented](#present/) in several <span id="id864547" class="indexterm"></span><span id="format/"></span>*formats*. For example, the boolean “<span class="quote">**`true`**</span>” might also be written as “<span class="quote">**`yes`**</span>”. <span id="id864574" class="indexterm"></span>[Tags](#tag/information%20model) must specify a mechanism for converting any formatted <span id="id864593" class="indexterm"></span>[scalar content](#scalar/information%20model) to a <span id="id864609" class="indexterm"></span>[canonical form](#canonical%20form/) for use in <span id="id864621" class="indexterm"></span>[equality](#equality/) testing. Like <span id="id864633" class="indexterm"></span>[node style](#style/), the format is a <span id="id864645" class="indexterm"></span>[presentation detail](#presentation%20detail/) and is not reflected in the <span id="id864659" class="indexterm"></span>[serialization tree](#serialization/) and <span id="id864672" class="indexterm"></span>[representation graph](#representation/).

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id864687"></span>3.2.3.3. Comments

</div>

</div>

</div>

<span id="id864695" class="indexterm"></span><span id="comment/information model"></span>*Comments* are a <span id="id864714" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not have any effect on the <span id="id864728" class="indexterm"></span>[serialization tree](#serialization/) or <span id="id864739" class="indexterm"></span>[representation graph](#representation/). In particular, comments are not associated with a particular <span id="id864753" class="indexterm"></span>[node](#node/information%20model). The usual purpose of a comment is to communicate between the human maintainers of a file. A typical example is comments in a configuration file. Comments may not appear inside <span id="id864774" class="indexterm"></span>[scalars](#scalar/information%20model), but may be interleaved with such <span id="id864791" class="indexterm"></span>[scalars](#scalar/information%20model) inside <span id="id864805" class="indexterm"></span>[collections](#collection/information%20model).

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id864824"></span>3.2.3.4. Directives

</div>

</div>

</div>

Each <span id="id864833" class="indexterm"></span>[document](#document/information%20model) may be associated with a set of <span id="id864849" class="indexterm"></span><span id="directive/information model"></span>*directives*. A directive has a name and an optional sequence of parameters. Directives are instructions to the YAML <span id="id864869" class="indexterm"></span>[processor](#processor/), and like all other <span id="id864880" class="indexterm"></span>[presentation details](#presentation%20detail/) are not reflected in the YAML <span id="id864894" class="indexterm"></span>[serialization tree](#serialization/) or <span id="id864907" class="indexterm"></span>[representation graph](#representation/). This version of YAML defines a two directives, <span id="id864921" class="indexterm"></span>[“<span class="quote">**`YAML`**</span>”](#YAML%20directive/) and <span id="id864940" class="indexterm"></span>[“<span class="quote">**`TAG`**</span>”](#TAG%20directive/). All other directives are <span id="id864955" class="indexterm"></span>[reserved](#reserved%20directive/) for future versions of YAML.

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id864977"></span>3.3. Loading Failure Points

</div>

</div>

</div>

The process of <span id="id864985" class="indexterm"></span>[loading](#load/) native data structures from a YAML <span id="id864997" class="indexterm"></span>[stream](#stream/information%20model) has several potential <span id="id865016" class="indexterm"></span><span id="load failure point/"></span>*failure points*. The character <span id="id865031" class="indexterm"></span>[stream](#stream/information%20model) may be <span id="id865045" class="indexterm"></span>[ill-formed](#ill-formed%20stream/), <span id="id865058" class="indexterm"></span>[aliases](#alias/information%20model) may be <span id="id865074" class="indexterm"></span>[unidentified](#unidentified%20alias/), <span id="id865089" class="indexterm"></span>[unspecified tags](#non-specific%20tag/) may be <span id="id865102" class="indexterm"></span>[unresolvable](#unresolved%20tag/), <span id="id865115" class="indexterm"></span>[tags](#tag/information%20model) may be <span id="id865131" class="indexterm"></span>[unrecognized](#unrecognized%20tag/), the <span id="id865146" class="indexterm"></span>[content](#content/information%20model) may be <span id="id865161" class="indexterm"></span>[invalid](#invalid%20content/), and a native type may be <span id="id865177" class="indexterm"></span>[unavailable](#unavailable%20tag/). Each of these failures results with an incomplete loading.

A <span id="id865194" class="indexterm"></span><span id="partial representation/"></span>*partial representation* need not <span id="id865209" class="indexterm"></span>[resolve](#tag%20resolution/) the <span id="id865224" class="indexterm"></span>[tag](#tag/information%20model) of each <span id="id865238" class="indexterm"></span>[node](#node/information%20model), and the <span id="id865253" class="indexterm"></span>[canonical form](#canonical%20form/) of <span id="id865266" class="indexterm"></span>[scalar content](#scalar/information%20model) need not be available. This weaker representation is useful for cases of incomplete knowledge of the types used in the <span id="id865284" class="indexterm"></span>[document](#document/information%20model). In contrast, a <span id="id865300" class="indexterm"></span><span id="complete representation/"></span>*complete representation* specifies the <span id="id865315" class="indexterm"></span>[tag](#tag/information%20model) of each <span id="id865330" class="indexterm"></span>[node](#node/information%20model), and provides the <span id="id865346" class="indexterm"></span>[canonical form](#canonical%20form/) of <span id="id865358" class="indexterm"></span>[scalar content](#scalar/information%20model), allowing for <span id="id865374" class="indexterm"></span>[equality](#equality/) testing. A complete representation is required in order to <span id="id865388" class="indexterm"></span>[construct](#construct/) native data structures.

<div class="figure">

<span id="id865402"></span>

**Figure 3.7. Loading Failure Points**

<div class="mediaobject">

![Loading Failure Points](validity2.png)

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id865423"></span>3.3.1. Well-Formed and Identified

</div>

</div>

</div>

A <span id="id865432" class="indexterm"></span><span id="well-formed stream/"></span>*well-formed* character <span id="id865446" class="indexterm"></span>[stream](#stream/information%20model) must match the productions specified in the next chapter. Successful loading also requires that each <span id="id865466" class="indexterm"></span>[alias](#alias/information%20model) shall refer to a previous <span id="id865481" class="indexterm"></span>[node](#node/information%20model) <span id="id865498" class="indexterm"></span>[identified](#identified/) by the <span id="id865509" class="indexterm"></span>[anchor](#anchor/information%20model). A YAML <span id="id865524" class="indexterm"></span>[processor](#processor/) should reject <span id="id865537" class="indexterm"></span><span id="ill-formed stream/"></span>*ill-formed streams* and <span id="id865552" class="indexterm"></span><span id="unidentified alias/"></span>*unidentified aliases*. A YAML <span id="id865568" class="indexterm"></span>[processor](#processor/) may recover from syntax errors, possibly by ignoring certain parts of the input, but it must provide a mechanism for reporting such errors.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id865585"></span>3.3.2. Resolved

</div>

</div>

</div>

It is not required that all the <span id="id865594" class="indexterm"></span>[tags](#tag/information%20model) of the <span id="id865610" class="indexterm"></span>[complete representation](#complete%20representation/) be explicitly specified in the character <span id="id865625" class="indexterm"></span>[stream](#stream/information%20model). During <span id="id865641" class="indexterm"></span>[parsing](#parse/), <span id="id865653" class="indexterm"></span>[nodes](#node/information%20model) that omit the <span id="id865669" class="indexterm"></span>[tag](#tag/information%20model) are given a <span id="id865684" class="indexterm"></span><span id="non-specific tag/"></span>*non-specific tag*: <span id="id865699" class="indexterm"></span><span id="? non-specific tag/"></span>*“<span class="quote">**`?`**</span>”* for <span id="id865718" class="indexterm"></span>[plain scalars](#plain%20style/information%20model) and <span id="id865735" class="indexterm"></span><span id="! non-specific tag/"></span>*“<span class="quote">**`!`**</span>”* for all other <span id="id865754" class="indexterm"></span>[nodes](#node/information%20model). These non-specific tags must be <span id="id865769" class="indexterm"></span><span id="tag resolution/"></span>*resolved* to a <span id="id865784" class="indexterm"></span><span id="specific tag/"></span>*specific tag* (either a <span id="id865798" class="indexterm"></span>[local tag](#local%20tag/) or a <span id="id865810" class="indexterm"></span>[global tag](#global%20tag/)) for a <span id="id865822" class="indexterm"></span>[complete representation](#complete%20representation/) to be <span id="id865837" class="indexterm"></span>[composed](#compose/).

Resolving the <span id="id865852" class="indexterm"></span>[tag](#tag/information%20model) of a <span id="id865869" class="indexterm"></span>[node](#node/information%20model) must only depend on the following three parameters: the non-specific tag of the <span id="id865885" class="indexterm"></span>[node](#node/information%20model), the path leading from the <span id="id865901" class="indexterm"></span>[root node](#root%20node/) to the <span id="id865914" class="indexterm"></span>[node](#node/information%20model), and the <span id="id865931" class="indexterm"></span>[content](#content/information%20model) (and hence the <span id="id865946" class="indexterm"></span>[kind](#kind/)) of the <span id="id865958" class="indexterm"></span>[node](#node/information%20model). In particular, resolution must not consider <span id="id865975" class="indexterm"></span>[presentation details](#presentation%20detail/) such as <span id="id865988" class="indexterm"></span>[comments](#comment/information%20model), <span id="id866006" class="indexterm"></span>[indentation](#indentation%20space/) and <span id="id866019" class="indexterm"></span>[node style](#style/). Also, resolution must not consider the <span id="id866031" class="indexterm"></span>[content](#content/information%20model) of any other <span id="id866046" class="indexterm"></span>[node](#node/information%20model), except for the <span id="id866065" class="indexterm"></span>[content](#content/information%20model) of the <span id="id866079" class="indexterm"></span>[key nodes](#key/information%20model) directly along the path leading from the <span id="id866095" class="indexterm"></span>[root node](#root%20node/) to the resolved <span id="id866109" class="indexterm"></span>[node](#node/information%20model). In particular, resolution must not consider the <span id="id866125" class="indexterm"></span>[content](#content/information%20model) of a sibling <span id="id866140" class="indexterm"></span>[node](#node/information%20model) in a <span id="id866156" class="indexterm"></span>[collection](#collection/information%20model) or the <span id="id866173" class="indexterm"></span>[content](#content/information%20model) of the <span id="id866188" class="indexterm"></span>[value node](#value/information%20model) associated with a resolved <span id="id866204" class="indexterm"></span>[key node](#key/information%20model).

Tag resolution is specific to the <span id="id866224" class="indexterm"></span>[application](#application/), hence a YAML <span id="id866236" class="indexterm"></span>[processor](#processor/) should provide a mechanism allowing the <span id="id866249" class="indexterm"></span>[application](#application/) to specify the tag resolution rules. It is recommended that <span id="id866262" class="indexterm"></span>[nodes](#node/information%20model) having the “<span class="quote">**`!`**</span>” non-specific tag should be resolved as “<span class="quote">**`tag:yaml.org,2002:seq`**</span>”, “<span class="quote">**`tag:yaml.org,2002:map`**</span>” or “<span class="quote">**`tag:yaml.org,2002:str`**</span>” depending on the <span id="id866308" class="indexterm"></span>[node’s kind](#node/information%20model). This convention allows the author of a YAML character <span id="id866326" class="indexterm"></span>[stream](#stream/information%20model) to exert some measure of control over the tag resolution process. By explicitly specifying a <span id="id866343" class="indexterm"></span>[plain scalar](#plain%20style/information%20model) has the “<span class="quote">**`!`**</span>” non-specific tag, the <span id="id866366" class="indexterm"></span>[node](#node/information%20model) is resolved as a string, as if it was <span id="id866383" class="indexterm"></span>[quoted](#quoted%20style/information%20model) or written in a <span id="id866398" class="indexterm"></span>[block style](#block%20style/information%20model). Note, however, that each <span id="id866415" class="indexterm"></span>[application](#application/) may override this behavior. For example, an <span id="id866428" class="indexterm"></span>[application](#application/) may automatically detect the type of programming language used in source code <span id="id866442" class="indexterm"></span>[presented](#present/) as a non-<span id="id866454" class="indexterm"></span>[plain](#plain%20style/information%20model) <span id="id866470" class="indexterm"></span>[scalar](#scalar/information%20model) and resolve it accordingly.

When a <span id="id866492" class="indexterm"></span>[node](#node/information%20model) has more than one occurrence (using an <span id="id866508" class="indexterm"></span>[anchor](#anchor/information%20model) and <span id="id866523" class="indexterm"></span>[alias nodes](#alias/information%20model)), tag resolution must depend only on the path to the first occurrence of the <span id="id866539" class="indexterm"></span>[node](#node/information%20model). Typically, the path leading to a <span id="id866558" class="indexterm"></span>[node](#node/information%20model) is sufficient to determine its specific tag. In cases where the path does not imply a single specific tag, the resolution also needs to consider the <span id="id866575" class="indexterm"></span>[node content](#content/information%20model) to select amongst the set of possible <span id="id866593" class="indexterm"></span>[tags](#tag/information%20model). Thus, <span id="id866607" class="indexterm"></span>[plain scalars](#plain%20style/information%20model) may be matched against a set of regular expressions to provide automatic resolution of integers, floats, timestamps, and similar types. Similarly, the <span id="id866627" class="indexterm"></span>[content](#content/information%20model) of <span id="id866641" class="indexterm"></span>[mapping nodes](#mapping/information%20model) may be matched against sets of expected <span id="id866657" class="indexterm"></span>[keys](#key/information%20model) to automatically resolve points, complex numbers, and similar types.

The combined effect of these rules is to ensure that tag resolution can be performed as soon as a <span id="id866680" class="indexterm"></span>[node](#node/information%20model) is first encountered in the <span id="id866696" class="indexterm"></span>[stream](#stream/information%20model), typically before its <span id="id866714" class="indexterm"></span>[content](#content/information%20model) is <span id="id866728" class="indexterm"></span>[parsed](#parse/). Also, tag resolution only requires referring to a relatively small number of previously parsed <span id="id866742" class="indexterm"></span>[nodes](#node/information%20model). Thus, tag resolution in one-pass <span id="id866760" class="indexterm"></span>[processors](#processor/) is both possible and practical.

If a <span id="id866775" class="indexterm"></span>[document](#document/information%20model) contains <span id="id866793" class="indexterm"></span><span id="unresolved tag/"></span>*unresolved tags*, the YAML <span id="id866806" class="indexterm"></span>[processor](#processor/) is unable to <span id="id866818" class="indexterm"></span>[compose](#compose/) a <span id="id866829" class="indexterm"></span>[complete representation](#complete%20representation/) graph. In such a case, the YAML <span id="id866846" class="indexterm"></span>[processor](#processor/) may <span id="id866857" class="indexterm"></span>[compose](#compose/) an <span id="id866869" class="indexterm"></span>[partial representation](#partial%20representation/), based on each <span id="id866884" class="indexterm"></span>[node’s kind](#kind/) and allowing for non-specific tags.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id866900"></span>3.3.3. Recognized and Valid

</div>

</div>

</div>

To be <span id="id866908" class="indexterm"></span><span id="valid content/"></span>*valid*, a <span id="id866922" class="indexterm"></span>[node](#node/information%20model) must have a <span id="id866938" class="indexterm"></span>[tag](#tag/information%20model) which is <span id="id866953" class="indexterm"></span><span id="recognized tag/"></span>*recognized* by the YAML <span id="id866967" class="indexterm"></span>[processor](#processor/) and its <span id="id866980" class="indexterm"></span>[content](#content/information%20model) must satisfy the constraints imposed by this <span id="id866999" class="indexterm"></span>[tag](#tag/information%20model). If a <span id="id867013" class="indexterm"></span>[document](#document/information%20model) contains a <span id="id867030" class="indexterm"></span>[scalar node](#scalar/information%20model) with an <span id="id867044" class="indexterm"></span><span id="unrecognized tag/"></span>*unrecognized tag* or <span id="id867058" class="indexterm"></span><span id="invalid content/"></span>*invalid content*, only a <span id="id867074" class="indexterm"></span>[partial representation](#partial%20representation/) may be <span id="id867087" class="indexterm"></span>[composed](#compose/). In contrast, a YAML <span id="id867100" class="indexterm"></span>[processor](#processor/) can always <span id="id867113" class="indexterm"></span>[compose](#compose/) a <span id="id867125" class="indexterm"></span>[complete representation](#complete%20representation/) for an unrecognized or an invalid <span id="id867141" class="indexterm"></span>[collection](#collection/information%20model), since <span id="id867157" class="indexterm"></span>[collection](#collection/information%20model) <span id="id867172" class="indexterm"></span>[equality](#equality/) does not depend upon knowledge of the <span id="id867184" class="indexterm"></span>[collection’s](#collection/information%20model) data type. However, such a <span id="id867201" class="indexterm"></span>[complete representation](#complete%20representation/) can not be used to <span id="id867214" class="indexterm"></span>[construct](#construct/) a native data structure.

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id867229"></span>3.3.4. Available

</div>

</div>

</div>

In a given processing environment, there need not be an <span id="id867238" class="indexterm"></span><span id="available tag/"></span>*available* native type corresponding to a given <span id="id867253" class="indexterm"></span>[tag](#tag/information%20model). If a <span id="id867271" class="indexterm"></span>[node’s tag](#tag/information%20model) is <span id="id867285" class="indexterm"></span><span id="unavailable tag/"></span>*unavailable*, a YAML <span id="id867299" class="indexterm"></span>[processor](#processor/) will not be able to <span id="id867311" class="indexterm"></span>[construct](#construct/) a native data structure for it. In this case, a <span id="id867325" class="indexterm"></span>[complete representation](#complete%20representation/) may still be <span id="id867341" class="indexterm"></span>[composed](#compose/), and an <span id="id867352" class="indexterm"></span>[application](#application/) may wish to use this <span id="id867364" class="indexterm"></span>[representation](#representation/) directly.

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id867381"></span>Chapter 4. Productions Conventions

</div>

</div>

</div>

The following chapters describe the syntax of YAML character <span id="id867391" class="indexterm"></span>[streams](#stream/syntax) in detail using a series of BNF productions. In most cases, productions are introduced in a “<span class="quote">bottom-up</span>” order; basic productions are specified before the more complex productions using them. Examples accompanying the productions display sample YAML text side-by-side with equivalent YAML text using only <span id="id867415" class="indexterm"></span>[flow collections](#flow%20collection%20style/syntax) and <span id="id867432" class="indexterm"></span>[double-quoted scalars](#double-quoted%20style/syntax). For improved readability, the equivalent YAML text uses the “<span class="quote">**`!!seq`**</span>”, “<span class="quote">**`!!map`**</span>”, and “<span class="quote">**`!!str`**</span>” <span id="id867471" class="indexterm"></span>[shorthands](#tag%20shorthand/) instead of the <span id="id867486" class="indexterm"></span>[verbatim](#verbatim%20tag/) “<span class="quote">**`!<tag:yaml.org,2002:seq>`**</span>”, “<span class="quote">**`!<tag:yaml.org,2002:map>`**</span>” and “<span class="quote">**`!<tag:yaml.org,2002:str>`**</span>” forms. These types are used to <span id="id867521" class="indexterm"></span>[resolve](#tag%20resolution/) all <span id="id867533" class="indexterm"></span>[untagged nodes](#non-specific%20tag/), except for a few examples that use the “<span class="quote">**`!!int`**</span>” and “<span class="quote">**`!!float`**</span>” types.

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id867562"></span>4.1. Production Prefixes

</div>

</div>

</div>

To make the syntax easier to follow, production names use Hungarian-style notation. Each production is given one of the following prefix based on the type of characters it matches.

<div class="variablelist">

<span class="term"> **`e-`** </span>  
A production matching no characters.

<span class="term"> **`c-`** </span>  
A production matching one or more characters starting and ending with a special (non-space) character.

<span class="term"> **`b-`** </span>  
A production matching a single <span id="id867618" class="indexterm"></span>[line break](#line%20break%20character/).

<span class="term"> **`nb-`** </span>  
A production matching one or more characters starting and ending with a non-<span id="id867646" class="indexterm"></span>[break](#line%20break%20character/) character.

<span class="term"> **`s-`** </span>  
A production matching one or more characters starting and ending with a space character.

<span class="term"> **`ns-`** </span>  
A production matching one or more characters starting and ending with a non-space character.

<span class="term"> `X`**`-`**`Y`**`-`** </span>  
A production matching a sequence of one or more characters, starting with an `X`**`-`** character and ending with a `Y`**`-`** character.

<span class="term"> **`l-`** </span>  
A production matching one or more lines (shorthand for **`s-b-`**).

<span class="term"> `X`**`+`**, `X`**`-`**`Y`**`+`** </span>  
A production as above, with the additional property that the <span id="id867785" class="indexterm"></span>[indentation](#indentation%20space/) level used is greater than the specified `n` parameter.

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id867808"></span>4.2. Production Parameters

</div>

</div>

</div>

As YAML’s syntax is designed for maximal readability, it makes heavy use of the context that each syntactical entity appears in. For notational compactness, this is expressed using parameterized BNF productions. The set of parameters and the range of allowed values depend on the specific production. The full list of possible parameters and their values is:

<div class="variablelist">

<span class="term"> Indentation: `n` or `m` </span>  
Since the character <span id="id867836" class="indexterm"></span>[stream](#stream/syntax) depends upon <span id="id867851" class="indexterm"></span>[indentation](#indentation%20space/) level to delineate blocks, many productions are parameterized by it. In some cases, the notations “<span class="quote">**`production(<n)`**</span>”, “<span class="quote">**`production(≤n)`**</span>” and “<span class="quote">**`production(>n)`**</span>” are used; these are shorthands for “<span class="quote">**`production(m)`**</span>” for some specific `m` where 0 ≤ `m` \< `n`, 0 ≤ `m` ≤ `n` and `m` \> `n`, respectively.

<span class="term">Context: `c`</span>  
YAML supports two groups of <span id="id867935" class="indexterm"></span><span id="context/"></span>*contexts*, distinguishing between <span id="id867949" class="indexterm"></span>[block styles](#block%20style/syntax) and <span id="id867967" class="indexterm"></span>[flow styles](#flow%20style/syntax). In the <span id="id867984" class="indexterm"></span>[block styles](#block%20style/syntax), <span id="id868000" class="indexterm"></span>[indentation](#indentation%20space/) is used to delineate structure. Due to the fact that the <span id="id868014" class="indexterm"></span>[“<span class="quote">**`-`**</span>”](#-%20block%20sequence%20entry/) character denoting a <span id="id868032" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax) entry is perceived as an <span id="id868049" class="indexterm"></span>[indentation](#indentation%20space/) character, some productions distinguish between the <span id="id868064" class="indexterm"></span>[block-in](#block-in%20context/) context (inside a <span id="id868078" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax)) and the <span id="id868094" class="indexterm"></span>[block-out](#block-out%20context/) context (outside one). In the <span id="id868111" class="indexterm"></span>[flow styles](#flow%20style/syntax), explicit <span id="id868125" class="indexterm"></span>[indicators](#indicator/) are used to delineate structure. As <span id="id868138" class="indexterm"></span>[plain scalars](#plain%20style/syntax) have no such <span id="id868155" class="indexterm"></span>[indicators](#indicator/), they are the most context sensitive, distinguishing between being nested inside a <span id="id868169" class="indexterm"></span>[flow collection](#flow%20collection%20style/syntax) (<span id="id868185" class="indexterm"></span>[flow-in](#flow-in%20context/) context) or being outside one (<span id="id868202" class="indexterm"></span>[flow-out](#flow-out%20context/) context). YAML also provides a terse and intuitive syntax for <span id="id868217" class="indexterm"></span>[simple keys](#simple%20key/). <span id="id868231" class="indexterm"></span>[Plain scalars](#plain%20style/syntax) in this (<span id="id868246" class="indexterm"></span>[flow-key](#flow-key%20context/)) context are the most restricted, for readability and implementation reasons.

<span class="term">(Scalar) Style: `s`</span>  
<span id="id868274" class="indexterm"></span>[Scalar content](#scalar/syntax) may be <span id="id868290" class="indexterm"></span>[presented](#present/) in one of five <span id="id868303" class="indexterm"></span>[styles](#scalar/syntax): the <span id="id868318" class="indexterm"></span>[plain](#plain%20style/syntax), <span id="id868334" class="indexterm"></span>[double-quoted](#double-quoted%20style/syntax) and <span id="id868350" class="indexterm"></span>[single-quoted](#single-quoted%20style/syntax)<span id="id868367" class="indexterm"></span>[flow styles](#flow%20style/syntax), and the <span id="id868384" class="indexterm"></span>[literal](#literal%20style/syntax) and <span id="id868399" class="indexterm"></span>[folded](#folded%20style/syntax)<span id="id868416" class="indexterm"></span>[block styles](#block%20style/syntax).

<span class="term">(Block) Chomping: `t`</span>  
Block scalars offer three possible mechanisms for <span id="id868444" class="indexterm"></span>[chomping](#chomping/) any trailing <span id="id868458" class="indexterm"></span>[line breaks](#line%20break%20character/): <span id="id868471" class="indexterm"></span>[strip](#strip%20chomping/), <span id="id868484" class="indexterm"></span>[clip](#clip%20chomping/) and <span id="id868499" class="indexterm"></span>[keep](#keep%20chomping/).

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id868518"></span>Chapter 5. Characters

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id868524"></span>5.1. Character Set

</div>

</div>

</div>

YAML <span id="id868532" class="indexterm"></span>[streams](#stream/syntax) use the <span id="id868546" class="indexterm"></span><span id="printable character/"></span>*printable* subset of the Unicode character set. On input, a YAML <span id="id868562" class="indexterm"></span>[processor](#processor/) must accept all printable ASCII characters, the space, <span id="id868575" class="indexterm"></span>[tab](#tab/), <span id="id868587" class="indexterm"></span>[line break](#line%20break%20character/), and all Unicode characters beyond \#x9F. On output, a YAML <span id="id868602" class="indexterm"></span>[processor](#processor/) must only produce these acceptable characters, and should also <span id="id868615" class="indexterm"></span>[escape](#escaping%20in%20double-quoted%20style/) all non-printable Unicode characters. The allowed character range explicitly excludes the surrogate block **`#xD800-#xDFFF`**, DEL **`#x7F`**, the C0 control block **`#x0-#x1F`** (except for **`#x9`**, **`#xA`**, and **`#xD`**), the C1 control block **`#x80-#x9F`**, **`#xFFFE`**, and **`#xFFFF`**. Any such characters must be <span id="id868686" class="indexterm"></span>[presented](#present/) using <span id="id868698" class="indexterm"></span>[escape](#escaping%20in%20double-quoted%20style/) sequences.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[1]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-printable"></span>c-printable</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  #x9 | #xA | #xD | [#x20-#x7E]          /* 8 bit */<br />
| #x85 | [#xA0-#xD7FF] | [#xE000-#xFFFD] /* 16 bit */<br />
| [#x10000-#x10FFFF]                     /* 32 bit */</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id868742"></span>5.2. Character Encoding

</div>

</div>

</div>

All characters mentioned in this specification are Unicode code points. Each such code point is written as one or more octets depending on the <span id="id868753" class="indexterm"></span><span id="character encoding/"></span>*character encoding* used. Note that in UTF-16, characters above **`#xFFFF`** are written as four octets, using a surrogate pair. A YAML <span id="id868774" class="indexterm"></span>[processor](#processor/) must support the UTF-16 and UTF-8 character encodings. If a character <span id="id868788" class="indexterm"></span>[stream](#stream/syntax) does not begin with a <span id="id868803" class="indexterm"></span><span id="byte order mark/"></span>*byte order mark* (**`#FEFF`**), the character encoding shall be UTF-8. Otherwise it shall be either UTF-8, UTF-16 LE, or UTF-16 BE as indicated by the byte order mark. On output, it is recommended that a byte order mark should only be emitted for UTF-16 character encodings. Note that the UTF-32 encoding is explicitly not supported. For more information about the byte order mark and the Unicode character encoding schemes see the Unicode <a href="http://www.unicode.org/unicode/faq/utf_bom.html" target="_top">FAQ</a>.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[2]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-byte-order-mark"></span>c-byte-order-mark</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#xFEFF</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In the examples, byte order mark characters are displayed as “<span class="quote">**`⇔`**</span>”.

<div class="example">

<span id="id868866"></span>

**Example 5.1. Byte Order Mark**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>⇔# Comment only.</code></pre>
<pre class="synopsis"><code>Legend:
  c-byte-order-mark</code></pre></td>
<td><pre class="programlisting"><code># This stream contains no
# documents, only comments.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id868932"></span>

**Example 5.2. Invalid Byte Order Mark**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code># Invalid use of BOM
⇔# inside a
# document.</code></pre></td>
<td><pre class="screen"><code>ERROR:
 A BOM must not appear
 inside a document.</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id868988"></span>5.3. Indicator Characters

</div>

</div>

</div>

<span id="id868996" class="indexterm"></span><span id="indicator/"></span>*Indicators* are characters that have special semantics used to describe the structure and <span id="id869011" class="indexterm"></span>[content](#content/syntax) of a YAML <span id="id869027" class="indexterm"></span>[document](#document/syntax).

<div class="itemizedlist">

- A <span id="id869048" class="indexterm"></span>[“<span class="quote">**`-`**</span>”](#-%20block%20sequence%20entry/) (**`#2D`**, hyphen) denotes a <span id="id869073" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax) entry.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[3]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-sequence-entry"></span>c-sequence-entry</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">-</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id869113" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20mapping%20key/) (**`#3F`**, question mark) denotes a <span id="id869136" class="indexterm"></span>[mapping key](#key/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[4]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-mapping-key"></span>c-mapping-key</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">?</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id869176" class="indexterm"></span>[“<span class="quote">**`:`**</span>”](#:%20mapping%20value/) (**`#3A`**, colon) denotes a <span id="id869199" class="indexterm"></span>[mapping value](#value/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[5]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-mapping-value"></span>c-mapping-value</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">:</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id869235"></span>

**Example 5.3. Block Structure Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>sequence:
- one
- two
mapping:
  ? sky
  : blue
  ? sea : green</code></pre>
<pre class="synopsis"><code>Legend:
  c-sequence-entry
  c-mapping-key
  c-mapping-value</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;sequence&quot;
  : !!seq [
    !!str &quot;one&quot;, !!str &quot;two&quot;
  ],
  ? !!str &quot;mapping&quot;
  : !!map {
    ? !!str &quot;sky&quot; : !!str &quot;blue&quot;,
    ? !!str &quot;sea&quot; : !!str &quot;green&quot;,
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- A <span id="id869368" class="indexterm"></span>[“<span class="quote">**`,`**</span>”](#,%20end%20flow%20entry/) (**`#2C`**, comma) ends a <span id="id869393" class="indexterm"></span>[flow collection](#flow%20collection%20style/syntax) entry.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[6]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-collect-entry"></span>c-collect-entry</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">,</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id869436" class="indexterm"></span>[“<span class="quote">**`[`**</span>”](#%5B%20start%20flow%20sequence/) (**`#5B`**, left bracket) starts a <span id="id869460" class="indexterm"></span>[flow sequence](#flow%20sequence%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[7]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-sequence-start"></span>c-sequence-start</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">[</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id869501" class="indexterm"></span>[“<span class="quote">**`]`**</span>”](#%5D%20end%20flow%20sequence/) (**`#5D`**, right bracket) ends a <span id="id869528" class="indexterm"></span>[flow sequence](#flow%20sequence%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[8]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-sequence-end"></span>c-sequence-end</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">]</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id869567" class="indexterm"></span>[“<span class="quote">**`{`**</span>”](#%7B%20start%20flow%20mapping/) (**`#7B`**, left brace) starts a <span id="id869594" class="indexterm"></span>[flow mapping](#flow%20mapping%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[9]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-mapping-start"></span>c-mapping-start</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">{</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id869634" class="indexterm"></span>[“<span class="quote">**`}`**</span>”](#%7D%20end%20flow%20mapping/) (**`#7D`**, right brace) ends a <span id="id869660" class="indexterm"></span>[flow mapping](#flow%20mapping%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[10]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-mapping-end"></span>c-mapping-end</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">}</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id869695"></span>

**Example 5.4. Flow Collection Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>sequence: [ one, two, ]
mapping: { sky: blue, sea: green }</code></pre>
<pre class="synopsis"><code>Legend:
  c-sequence-start c-sequence-end
  c-mapping-start  c-mapping-end
  c-collect-entry</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;sequence&quot;
  : !!seq [
    !!str &quot;one&quot;, !!str &quot;two&quot;
  ],
  ? !!str &quot;mapping&quot;
  : !!map {
    ? !!str &quot;sky&quot; : !!str &quot;blue&quot;,
    ? !!str &quot;sea&quot; : !!str &quot;green&quot;,
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- An <span id="id869844" class="indexterm"></span>[“<span class="quote">**`#`**</span>”](##%20comment/) (**`#23`**, octothorpe, hash, sharp, number sign) denotes a <span id="id869867" class="indexterm"></span>[comment](#comment/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[11]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-comment"></span>c-comment</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">#</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id869903"></span>

**Example 5.5. Comment Indicator**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code># Comment only.</code></pre>
<pre class="synopsis"><code>Legend:
  c-comment</code></pre></td>
<td><pre class="programlisting"><code># This stream contains no
# documents, only comments.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- An <span id="id869972" class="indexterm"></span>[“<span class="quote">**`&`**</span>”](#&%20anchor/) (**`#26`**, ampersand) denotes a <span id="id869997" class="indexterm"></span>[node’s anchor property](#anchor/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[12]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-anchor"></span>c-anchor</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">&amp;</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- An <span id="id870038" class="indexterm"></span>[“<span class="quote">**`*`**</span>”](#*%20alias/) (**`#2A`**, asterisk) denotes an <span id="id870060" class="indexterm"></span>[alias node](#alias/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[13]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-alias"></span>c-alias</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">*</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- An <span id="id870101" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20tag%20indicator/) (**`#21`**, exclamation) denotes a <span id="id870124" class="indexterm"></span>[node’s tag](#tag/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[14]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-tag"></span>c-tag</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">!</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id870160"></span>

**Example 5.6. Node Property Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>anchored: !local &amp;anchor value
alias: *anchor</code></pre>
<pre class="synopsis"><code>Legend:
  c-anchor
  c-alias
  c-tag</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;anchored&quot;
  : !local &amp;A1 &quot;value&quot;,
  ? !!str &quot;alias&quot;
  : *A1,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- A <span id="id870265" class="indexterm"></span>[“<span class="quote">**`|`**</span>”](#%7C%20literal%20style/) (**`7C`**, vertical bar) denotes a <span id="id870287" class="indexterm"></span>[literal block scalar](#literal%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[15]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-literal"></span>c-literal</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">|</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id870329" class="indexterm"></span>[“<span class="quote">**`>`**</span>”](#%3E%20folded%20style/) (**`#3E`**, greater than) denotes a <span id="id870354" class="indexterm"></span>[folded block scalar](#folded%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[16]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-folded"></span>c-folded</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">&gt;</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id870388"></span>

**Example 5.7. Block Scalar Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>literal: |
  text
folded: &gt;
  text</code></pre>
<pre class="synopsis"><code>Legend:
  c-literal
  c-folded</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;literal&quot;
  : !!str &quot;text\n&quot;,
  ? !!str &quot;folded&quot;
  : !!str &quot;text\n&quot;,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- An <span id="id870475" class="indexterm"></span>[“<span class="quote">**`'`**</span>”](#'%20single-quoted%20style/) (**`#27`**, apostrophe, single quote) surrounds a <span id="id870502" class="indexterm"></span>[single-quoted flow scalar](#single-quoted%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[17]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-single-quote"></span>c-single-quote</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">'</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A <span id="id870542" class="indexterm"></span>[“<span class="quote">**`"`**</span>”](#%22%20double-quoted%20style/) (**`#22`**, double quote) surrounds a <span id="id870568" class="indexterm"></span>[double-quoted flow scalar](#double-quoted%20style/syntax).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[18]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-double-quote"></span>c-double-quote</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">"</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id870602"></span>

**Example 5.8. Quoted Scalar Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>single: &#39;text&#39;
double: &quot;text&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  c-single-quote
  c-double-quote</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;double&quot;
  : !!str &quot;text&quot;,
  ? !!str &quot;single&quot;
  : !!str &quot;text&quot;,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- A <span id="id870700" class="indexterm"></span>[“<span class="quote">**`%`**</span>”](#%%20directive/) (**`#25`**, percent) denotes a <span id="id870723" class="indexterm"></span>[directive](#directive/syntax) line.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[19]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-directive"></span>c-directive</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">%</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id870761"></span>

**Example 5.9. Directive Indicator**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>%YAML 1.1
--- text</code></pre>
<pre class="synopsis"><code>Legend:
  c-directive</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;text&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- The <span id="id870830" class="indexterm"></span><span id="@ reserved indicator/"></span>*“<span class="quote">**`@`**</span>”* (**`#40`**, at) and <span id="id870857" class="indexterm"></span><span id="' reserved indicator/"></span>*“<span class="quote">**`` ` ``**</span>”* (**`#60`**, grave accent) are <span id="id870882" class="indexterm"></span><span id="reserved indicator/"></span>*reserved* for future use.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[20]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-reserved"></span>c-reserved</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">@</span>” | “<span class="quote">`</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id870919"></span>

**Example 5.10. Invalid use of Reserved Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>commercial-at: @text
grave-accent: `text</code></pre></td>
<td><pre class="screen"><code>ERROR:
 Reserved indicators can&#39;t
 start a plain scalar.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- Any indicator character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[21]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-indicator"></span>c-indicator</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-sequence-entry">“<span class="quote">-</span>”</a> | <a href="#c-mapping-key">“<span class="quote">?</span>”</a> | <a href="#c-mapping-value">“<span class="quote">:</span>”</a> | <a href="#c-collect-entry">“<span class="quote">,</span>”</a> | <a href="#c-sequence-start">“<span class="quote">[</span>”</a> | <a href="#c-sequence-end">“<span class="quote">]</span>”</a> | <a href="#c-mapping-start">“<span class="quote">{</span>”</a> | <a href="#c-mapping-end">“<span class="quote">}</span>”</a><br />
| <a href="#c-comment">“<span class="quote">#</span>”</a> | <a href="#c-anchor">“<span class="quote">&amp;</span>”</a> | <a href="#c-alias">“<span class="quote">*</span>”</a> | <a href="#c-tag">“<span class="quote">!</span>”</a> | <a href="#c-literal">“<span class="quote">|</span>”</a> | <a href="#c-folded">“<span class="quote">&gt;</span>”</a> | <a href="#c-single-quote">“<span class="quote">'</span>”</a> | <a href="#c-double-quote">“<span class="quote">"</span>”</a><br />
| <a href="#c-directive">“<span class="quote">%</span>”</a> | <a href="#c-reserved">“<span class="quote">@</span>” | “<span class="quote">`</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id871136"></span>5.4. Line Break Characters

</div>

</div>

</div>

The Unicode standard defines the following <span id="id871145" class="indexterm"></span><span id="line break character/"></span>*line break* characters:

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[22]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-line-feed"></span>b-line-feed</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#xA /*LF*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[23]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-carriage-return"></span>b-carriage-return</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#xD /*CR*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[24]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-next-line"></span>b-next-line</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#x85 /*NEL*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[25]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-line-separator"></span>b-line-separator</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#x2028 /*LS*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[26]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-paragraph-separator"></span>b-paragraph-separator</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#x2029 /*PS*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

A YAML <span id="id871235" class="indexterm"></span>[processor](#processor/) must accept all the possible Unicode line break characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[27]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-char"></span>b-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#b-line-feed">b-line-feed</a> | <a href="#b-carriage-return">b-carriage-return</a> | <a href="#b-next-line">b-next-line</a><br />
| <a href="#b-line-separator">b-line-separator</a> | <a href="#b-paragraph-separator">b-paragraph-separator</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Line breaks can be grouped into two categories. <span id="id871293" class="indexterm"></span><span id="specific line break/"></span>*Specific line breaks* have well-defined semantics for breaking text into lines and paragraphs, and must be preserved by the YAML <span id="id871310" class="indexterm"></span>[processor](#processor/) inside <span id="id871322" class="indexterm"></span>[scalar content](#scalar/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[28]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-specific"></span>b-specific</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-line-separator">b-line-separator</a> | <a href="#b-paragraph-separator">b-paragraph-separator</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<span id="id871365" class="indexterm"></span><span id="generic line break/"></span>*Generic line breaks* do not carry a meaning beyond “<span class="quote">ending a line</span>”. Unlike specific line breaks, there are several widely used forms for generic line breaks.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[29]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-generic"></span>b-generic</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#b-carriage-return">b-carriage-return</a> <a href="#b-line-feed">b-line-feed</a> ) /* DOS, Windows */<br />
| <a href="#b-carriage-return">b-carriage-return</a>                 /* Macintosh */<br />
| <a href="#b-line-feed">b-line-feed</a>                       /* UNIX */<br />
| <a href="#b-next-line">b-next-line</a>                       /* Unicode */</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Generic line breaks inside <span id="id871447" class="indexterm"></span>[scalar content](#scalar/syntax) must be <span id="id871460" class="indexterm"></span><span id="line break normalization/"></span>*normalized* by the YAML <span id="id871475" class="indexterm"></span>[processor](#processor/). Each such line break must be <span id="id871488" class="indexterm"></span>[parsed](#parse/) into a single line feed character. The original line break form is a <span id="id871502" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not be used to convey <span id="id871516" class="indexterm"></span>[content information](#content/information%20model).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[30]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-as-line-feed"></span>b-as-line-feed</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-generic">b-generic</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[31]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-normalized"></span>b-normalized</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-as-line-feed">b-as-line-feed</a> | <a href="#b-specific">b-specific</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Normalization does not apply to ignored (<span id="id871577" class="indexterm"></span>[escaped](#escaped%20(ignored)%20line%20break/) or <span id="id871594" class="indexterm"></span>[chomped](#chomping/)) generic line breaks.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[32]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-ignored-generic"></span>b-ignored-generic</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-generic">b-generic</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Outside <span id="id871628" class="indexterm"></span>[scalar content](#scalar/syntax), YAML allows any line break to be used to terminate lines.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[33]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-ignored-any"></span>b-ignored-any</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-generic">b-generic</a> | <a href="#b-specific">b-specific</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

On output, a YAML <span id="id871673" class="indexterm"></span>[processor](#processor/) is free to <span id="id871685" class="indexterm"></span>[present](#present/) line breaks using whatever convention is most appropriate, though specific line breaks must be preserved in <span id="id871700" class="indexterm"></span>[scalar content](#scalar/syntax). These rules are compatible with <a href="http://www.unicode.org/unicode/reports/tr13/" target="_top">Unicode’s newline guidelines</a>.

In the examples, line break characters are displayed as follows: “<span class="quote">**`↓`**</span>” or no glyph for a generic line break, “<span class="quote">**`⇓`**</span>” for a line separator and “<span class="quote">**`¶`**</span>” for a paragraph separator.

<div class="example">

<span id="id871752"></span>

**Example 5.11. Line Break Characters**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>|
  Generic line break (no glyph)
  Generic line break (glyphed)↓
  Line separator⇓
  Paragraph separator¶</code></pre>
<pre class="synopsis"><code>Legend:
  b-generic b-line-separator
  b-paragraph-separator</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
--- !!str
&quot;Generic line break (no glyph)\n\
 Generic line break (glyphed)\n\
 Line separator\u2028\
 Paragraph separator\u2029&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id871856"></span>5.5. Miscellaneous Characters

</div>

</div>

</div>

The YAML syntax productions make use of the following character range definitions:

<div class="itemizedlist">

- A non-<span id="id871872" class="indexterm"></span>[break](#line%20break%20character/) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[34]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-char"></span>nb-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-printable">c-printable</a> - <a href="#b-char">b-char</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- An ignored space character outside <span id="id871918" class="indexterm"></span>[scalar content](#scalar/syntax). Such spaces are used for <span id="id871933" class="indexterm"></span>[indentation](#indentation%20space/) and <span id="id871947" class="indexterm"></span>[separation](#separation%20space/) between tokens. To maintain portability, <span id="id871963" class="indexterm"></span><span id="tab/"></span>*tab* characters must not be used in these cases, since different systems treat tabs differently. Note that most modern editors may be configured so that pressing the tab key results in the insertion of an appropriate number of spaces.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[35]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ignored-space"></span>s-ignored-space</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#x20 /*SP*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id871998"></span>

**Example 5.12. Invalid Use of Tabs**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code># Tabs do&#39;s and don&#39;ts:
# comment: →
quoted: &quot;Quoted →&quot;
block: |
  void main() {
  →printf(&quot;Hello, world!\n&quot;);
  }
elsewhere:→# separation
→indentation, in→plain scalar</code></pre></td>
<td><pre class="screen"><code>ERROR:
 Tabs may appear inside
 comments and quoted or
 block scalar content.
 Tabs must not appear
 elsewhere, such as
 in indentation and
 separation spaces.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- A <span id="id872096" class="indexterm"></span><span id="white space/"></span>*white space* character in <span id="id872110" class="indexterm"></span>[quoted](#quoted%20style/syntax) or <span id="id872127" class="indexterm"></span>[block scalar content](#block%20scalar%20style/syntax):

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[36]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-white"></span>s-white</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">#x9 /*TAB*/ | #x20 /*SP*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In the examples, tab characters are displayed as the glyph “<span class="quote">**`→`**</span>”. Space characters are sometimes displayed as the glyph “<span class="quote">**`·`**</span>” for clarity.

<div class="example">

<span id="id872184"></span>

**Example 5.13. Tabs and Spaces**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>··&quot;Text·containing···
··both·space·and→
··→tab→characters&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  #x9 (TAB) #x20 (SP)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
--- !!str
&quot;Text·containing·\
 both·space·and·\
 tab→characters&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="itemizedlist">

- An ignored white space character inside <span id="id872327" class="indexterm"></span>[scalar content](#scalar/syntax):

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[37]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ignored-white"></span>s-ignored-white</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-white">s-white</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A non space (and non-<span id="id872367" class="indexterm"></span>[break](#line%20break%20character/)) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[38]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-char"></span>ns-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-char">nb-char</a> - <a href="#s-white">s-white</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A decimal digit for numbers:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[39]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-dec-digit"></span>ns-dec-digit</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">[#x30-#x39] /*0-9*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A hexadecimal digit for <span id="id872437" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/):

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[40]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-hex-digit"></span>ns-hex-digit</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-dec-digit">ns-dec-digit</a> | [#x41-#x46] /*A-F*/ | [#x61-#x66] /*a-f*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- An ASCII letter (alphabetic) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[41]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-ascii-letter"></span>ns-ascii-letter</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">[#x41-#x5A] /*A-Z*/ | [#x61-#x7A] /*a-z*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A word (alphanumeric) character for identifiers:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[42]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-word-char"></span>ns-word-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-dec-digit">ns-dec-digit</a> | <a href="#ns-ascii-letter">ns-ascii-letter</a> | “<span class="quote">-</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- A URI character for <span id="id872540" class="indexterm"></span>[tags](#tag/syntax), as specified in <a href="http://www.ietf.org/rfc/rfc2396.txt" target="_top">RFC2396</a> with the addition of the <span id="id872560" class="indexterm"></span>[“<span class="quote">**`[`**</span>”](#%5B%20start%20flow%20sequence/) and <span id="id872579" class="indexterm"></span>[“<span class="quote">**`]`**</span>”](#%5D%20end%20flow%20sequence/) for presenting IPv6 addresses as proposed in <a href="http://www.ietf.org/rfc/rfc2732.txt" target="_top">RFC2732</a>. A limited form of 8-bit <span id="id872604" class="indexterm"></span><span id="escaping in URI/"></span>*escaping* is available using the <span id="id872620" class="indexterm"></span><span id="% escaping in URI/"></span>*“<span class="quote">**`%`**</span>”* character. By convention, URIs containing 16 and 32 bit Unicode characters are <span id="id872638" class="indexterm"></span>[encoded](#character%20encoding/) in UTF-8, and then each octet is written as a separate character.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[43]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-uri-char"></span>ns-uri-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#ns-word-char">ns-word-char</a> | “<span class="quote">%</span>” <a href="#ns-hex-digit">ns-hex-digit</a> <a href="#ns-hex-digit">ns-hex-digit</a><br />
| “<span class="quote">;</span>” | “<span class="quote">/</span>” | “<span class="quote">?</span>” | “<span class="quote">:</span>” | “<span class="quote">@</span>” | “<span class="quote">&amp;</span>” | “<span class="quote">=</span>” | “<span class="quote">+</span>” | “<span class="quote">$</span>” | “<span class="quote">,</span>”<br />
| “<span class="quote">_</span>” | “<span class="quote">.</span>” | “<span class="quote">!</span>” | “<span class="quote">~</span>” | “<span class="quote">*</span>” | “<span class="quote">'</span>” | “<span class="quote">(</span>” | “<span class="quote">)</span>” | “<span class="quote">[</span>” | “<span class="quote">]</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- The <span id="id872764" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20named%20handle/) character is used to indicate the end of a <span id="id872783" class="indexterm"></span>[named tag handle](#named%20tag%20handle/); hence its use in <span id="id872800" class="indexterm"></span>[tag shorthands](#tag%20shorthand/) is restricted.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[44]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-tag-char"></span>ns-tag-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-uri-char">ns-uri-char</a> - <a href="#c-tag">“<span class="quote">!</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id872840"></span>5.6. Escape Sequences

</div>

</div>

</div>

All non-<span id="id872849" class="indexterm"></span>[printable](#printable%20character/) characters must be <span id="id872862" class="indexterm"></span>[presented](#present/) as <span id="id872875" class="indexterm"></span><span id="escaping in double-quoted style/"></span>*escape sequences*. Each escape sequences must be <span id="id872891" class="indexterm"></span>[parsed](#parse/) into the appropriate Unicode character. The original escape sequence form is a <span id="id872905" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not be used to convey <span id="id872919" class="indexterm"></span>[content information](#content/information%20model). YAML escape sequences use the <span id="id872936" class="indexterm"></span><span id="\ escaping in double-quoted style/"></span>*“<span class="quote">**`\`**</span>”* notation common to most modern computer languages. Note that escape sequences are only interpreted in <span id="id872960" class="indexterm"></span>[double-quoted scalars](#double-quoted%20style/syntax). In all other <span id="id872974" class="indexterm"></span>[scalar styles](#scalar/syntax), the <span id="id872989" class="indexterm"></span>[“<span class="quote">**`\`**</span>”](#\%20escaping%20in%20double-quoted%20style/) character has no special meaning and non-<span id="id873012" class="indexterm"></span>[printable](#printable%20character/) characters are not available.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[45]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-escape"></span>c-escape</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">\</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

YAML escape sequences are a superset of C’s escape sequences:

<div class="itemizedlist">

- Escaped ASCII null (**`#x0`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[46]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-null"></span>ns-esc-null</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">0</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII bell (**`#x7`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[47]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-bell"></span>ns-esc-bell</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">a</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII backspace (**`#x8`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[48]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-backspace"></span>ns-esc-backspace</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">b</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII horizontal <span id="id873168" class="indexterm"></span>[tab](#tab/) (**`#x9`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[49]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-horizontal-tab"></span>ns-esc-horizontal-tab</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">t</span>” | <a href="#c-escape">“<span class="quote">\</span>”</a> #x9</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII <span id="id873225" class="indexterm"></span>[line feed](#generic%20line%20break/) (**`#xA`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[50]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-line-feed"></span>ns-esc-line-feed</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">n</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII vertical tab (**`#xB`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[51]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-vertical-tab"></span>ns-esc-vertical-tab</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">v</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII form feed (**`#xC`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[52]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-form-feed"></span>ns-esc-form-feed</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">f</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII <span id="id873355" class="indexterm"></span>[carriage return](#generic%20line%20break/) (**`#xD`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[53]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-carriage-return"></span>ns-esc-carriage-return</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">r</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII escape (**`#x1B`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[54]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-escape"></span>ns-esc-escape</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">e</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII space (**`#x20`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[55]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-space"></span>ns-esc-space</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> #x20</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII double quote (<span id="id873481" class="indexterm"></span>[“<span class="quote">**`"`**</span>”](#%22%20double-quoted%20style/)):

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[56]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-double-quote"></span>ns-esc-double-quote</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> <a href="#c-double-quote">“<span class="quote">"</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped ASCII back slash (<span id="id873536" class="indexterm"></span>[“<span class="quote">**`\`**</span>”](#\%20escaping%20in%20double-quoted%20style/)):

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[57]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-backslash"></span>ns-esc-backslash</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> <a href="#c-escape">“<span class="quote">\</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped Unicode <span id="id873590" class="indexterm"></span>[next line](#generic%20line%20break/) (**`#x85`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[58]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-next-line"></span>ns-esc-next-line</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">N</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped Unicode non-breaking space (**`#xA0`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[59]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-non-breaking-space"></span>ns-esc-non-breaking-space</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">_</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped Unicode <span id="id873679" class="indexterm"></span>[line separator](#specific%20line%20break/) (**`#x2028`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[60]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-line-separator"></span>ns-esc-line-separator</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">L</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped Unicode <span id="id873732" class="indexterm"></span>[paragraph separator](#specific%20line%20break/) (**`#x2029`**) character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[61]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-paragraph-separator"></span>ns-esc-paragraph-separator</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">P</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped 8-bit Unicode character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[62]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-8-bit"></span>ns-esc-8-bit</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">x</span>” ( <a href="#ns-hex-digit">ns-hex-digit</a> x 2 )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped 16-bit Unicode character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[63]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-16-bit"></span>ns-esc-16-bit</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">u</span>” ( <a href="#ns-hex-digit">ns-hex-digit</a> x 4 )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Escaped 32-bit Unicode character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[64]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-32-bit"></span>ns-esc-32-bit</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-escape">“<span class="quote">\</span>”</a> “<span class="quote">U</span>” ( <a href="#ns-hex-digit">ns-hex-digit</a> x 8 )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Any escaped character:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[65]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-esc-char"></span>ns-esc-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#ns-esc-null">ns-esc-null</a> | <a href="#ns-esc-bell">ns-esc-bell</a> | <a href="#ns-esc-backspace">ns-esc-backspace</a><br />
| <a href="#ns-esc-horizontal-tab">ns-esc-horizontal-tab</a> | <a href="#ns-esc-line-feed">ns-esc-line-feed</a><br />
| <a href="#ns-esc-vertical-tab">ns-esc-vertical-tab</a> | <a href="#ns-esc-form-feed">ns-esc-form-feed</a><br />
| <a href="#ns-esc-carriage-return">ns-esc-carriage-return</a> | <a href="#ns-esc-escape">ns-esc-escape</a> | <a href="#ns-esc-space">ns-esc-space</a><br />
| <a href="#ns-esc-double-quote">ns-esc-double-quote</a> | <a href="#ns-esc-backslash">ns-esc-backslash</a><br />
| <a href="#ns-esc-next-line">ns-esc-next-line</a> | <a href="#ns-esc-non-breaking-space">ns-esc-non-breaking-space</a><br />
| <a href="#ns-esc-line-separator">ns-esc-line-separator</a> | <a href="#ns-esc-paragraph-separator">ns-esc-paragraph-separator</a><br />
| <a href="#ns-esc-8-bit">ns-esc-8-bit</a> | <a href="#ns-esc-16-bit">ns-esc-16-bit</a> | <a href="#ns-esc-32-bit">ns-esc-32-bit</a><br />
</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id891492"></span>

**Example 5.14. Escaped Characters**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&quot;Fun with \\
 \&quot; \a \b \e \f \↓
 \n \r \t \v \0 \⇓
 \  \_ \N \L \P \¶
 \x41 \u0041 \U00000041&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  ns-esc-char</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
&quot;Fun with \x5C
 \x22 \x07 \x08 \x1B \0C
 \x0A \x0D \x09 \x0B \x00
 \x20 \xA0 \x85 \u2028 \u2029
 A A A&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id891672"></span>

**Example 5.15. Invalid Escaped Characters**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>Bad escapes:
  &quot;\c
  \xq-&quot;</code></pre></td>
<td><pre class="screen"><code>ERROR:
- c is an invalid escaped character.
- q and - are invalid hex digits.</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id891745"></span>Chapter 6. Syntax Primitives

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id891751"></span>6.1. Indentation Spaces

</div>

</div>

</div>

In a YAML character <span id="id891760" class="indexterm"></span>[stream](#stream/syntax), structure is often determined from <span id="id891775" class="indexterm"></span><span id="indentation space/"></span>*indentation*, where indentation is defined as a <span id="id891791" class="indexterm"></span>[line break](#line%20break%20character/) character (or the start of the <span id="id891806" class="indexterm"></span>[stream](#stream/syntax)) followed by zero or more space characters. Note that indentation must not contain any <span id="id891822" class="indexterm"></span>[tab](#tab/) characters. The amount of indentation is a <span id="id891834" class="indexterm"></span>[presentation detail](#presentation%20detail/) used exclusively to delineate structure and is otherwise ignored. In particular, indentation characters must never be considered part of a <span id="id891852" class="indexterm"></span>[node’s content information](#content/information%20model).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[66]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-indent(n)"></span>s-indent(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-space">s-ignored-space</a> x n</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id891888"></span>

**Example 6.1. Indentation Spaces**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>··# Leading comment line spaces are
···# neither content nor indentation.
····
Not indented:
·By one space: |
····By four
······spaces
·Flow style: [    # Leading spaces
···By two,        # in flow style
··Also by two,    # are neither
··→Still by two   # content nor
····]             # indentation.</code></pre>
<pre class="synopsis"><code>Legend:
  s-indent(n) Content
  Neither content nor indentation</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;Not indented&quot;
  : !!map {
      ? !!str &quot;By one space&quot;
      : !!str &quot;By four\n  spaces\n&quot;,
      ? !!str &quot;Flow style&quot;
      : !!seq [
          !!str &quot;By two&quot;,
          !!str &quot;Still by two&quot;,
          !!str &quot;Again by two&quot;,
        ]
    }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

In general, a <span id="id892050" class="indexterm"></span>[node](#node/syntax) must be indented further than its parent <span id="id892067" class="indexterm"></span>[node](#node/syntax). All sibling <span id="id892082" class="indexterm"></span>[nodes](#node/syntax) must use the exact same indentation level, however the <span id="id892098" class="indexterm"></span>[content](#content/syntax) of each sibling <span id="id892113" class="indexterm"></span>[node](#node/syntax) may be further indented independently. The <span id="id892129" class="indexterm"></span>[“<span class="quote">**`-`**</span>”](#-%20block%20sequence%20entry/), <span id="id892148" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20mapping%20key/) and <span id="id892165" class="indexterm"></span>[“<span class="quote">**`:`**</span>”](#:%20mapping%20value/) characters used to denote <span id="id892182" class="indexterm"></span>[block collection](#block%20collection%20style/syntax) entries are perceived by people to be part of the indentation. Hence the indentation rules are slightly more flexible when dealing with these <span id="id892201" class="indexterm"></span>[indicators](#indicator/). First, a <span id="id892212" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax) need not be indented relative to its parent <span id="id892229" class="indexterm"></span>[node](#node/syntax), unless that <span id="id892244" class="indexterm"></span>[node](#node/syntax) is also a <span id="id892259" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax). Second, compact <span id="id892275" class="indexterm"></span>[in-line](#in-line%20style/syntax) notations allow a nested <span id="id892292" class="indexterm"></span>[collection](#collection/syntax) to begin immediately following the <span id="id892309" class="indexterm"></span>[indicator](#indicator/) (where the <span id="id892321" class="indexterm"></span>[indicator](#indicator/) is counted as part of the indentation). This provides for an intuitive <span id="id892335" class="indexterm"></span>[collection](#collection/syntax) nesting syntax.

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id892353"></span>6.2. Comments

</div>

</div>

</div>

An explicit <span id="id892361" class="indexterm"></span><span id="comment/syntax"></span>*comment* is marked by a <span id="id892378" class="indexterm"></span><span id="# comment/"></span>*“<span class="quote">**`#`**</span>” indicator*. Comments are a <span id="id892397" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must have no effect on the <span id="id892411" class="indexterm"></span>[serialization tree](#serialization/) (and hence the <span id="id892424" class="indexterm"></span>[representation graph](#representation/)).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[67]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-nb-comment-text"></span>c-nb-comment-text</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-comment">“<span class="quote">#</span>”</a> <a href="#nb-char">nb-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Comments always span to the end of the line.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[68]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-b-comment"></span>c-b-comment</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-nb-comment-text">c-nb-comment-text</a>? <a href="#b-ignored-any">b-ignored-any</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Outside <span id="id892496" class="indexterm"></span>[scalar content](#scalar/syntax), comments may appear on a line of their own, independent of the <span id="id892513" class="indexterm"></span>[indentation](#indentation%20space/) level. Note that <span id="id892528" class="indexterm"></span>[tab](#tab/) characters must not be used and that <span id="id892540" class="indexterm"></span>[empty lines](#empty%20line/) outside <span id="id892552" class="indexterm"></span>[scalar content](#scalar/syntax) are taken to be (empty) comment lines.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[69]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-comment"></span>l-comment</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-space">s-ignored-space</a>* <a href="#c-b-comment">c-b-comment</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id892593"></span>

**Example 6.2. Comment Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>··# Comment↓
···↓
↓</code></pre></td>
<td><pre class="programlisting"><code># This stream contains no
# documents, only comments.</code></pre>
<pre class="synopsis"><code>Legend:
  c-b-comment l-comment</code></pre></td>
</tr>
</tbody>
</table>

</div>

When a comment follows another syntax element, it must be <span id="id892698" class="indexterm"></span>[separated](#separation%20space/) from it by space characters. Like the comment itself, such characters are not considered part of the <span id="id892714" class="indexterm"></span>[content information](#content/information%20model).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[70]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-b-comment"></span>s-b-comment</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#s-ignored-space">s-ignored-space</a>+ <a href="#c-nb-comment-text">c-nb-comment-text</a>? )?<br />
<a href="#b-ignored-any">b-ignored-any</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id892762"></span>

**Example 6.3. Comments Ending a Line**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>key:····# Comment↓
  value↓</code></pre>
<pre class="synopsis"><code>Legend:
  c-nb-comment-text s-b-comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;key&quot;
  : !!str &quot;value&quot;
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

In most cases, when a line may end with a comment, YAML allows it to be followed by additional comment lines.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[71]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l-comments"></span>c-l-comments</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-b-comment">c-b-comment</a> <a href="#l-comment">l-comment</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[72]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-comments"></span>s-l-comments</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-b-comment">s-b-comment</a> <a href="#l-comment">l-comment</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id892902"></span>

**Example 6.4. Multi-Line Comments**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>key:····# Comment↓
········# lines↓
  value↓
↓</code></pre>
<pre class="synopsis"><code>Legend:
  s-b-comment l-comment s-l-comments</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;key&quot;
  : !!str &quot;value&quot;
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id893014"></span>6.3. Separation Spaces

</div>

</div>

</div>

Outside <span id="id893023" class="indexterm"></span>[scalar content](#scalar/syntax), YAML uses space characters for <span id="id893039" class="indexterm"></span><span id="separation space/"></span>*separation* between tokens. Note that separation must not contain <span id="id893056" class="indexterm"></span>[tab](#tab/) characters. Separation spaces are a <span id="id893068" class="indexterm"></span>[presentation detail](#presentation%20detail/) used exclusively to delineate structure and are otherwise ignored; in particular, such characters must never be considered part of a <span id="id893085" class="indexterm"></span>[node’s content information](#content/information%20model).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[73]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-separate(n,c)"></span>s-separate(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = block-out ⇒ <a href="#s-separate-lines(n)">s-separate-lines(n)</a><br />
<code class="varname">c</code> = block-in  ⇒ <a href="#s-separate-lines(n)">s-separate-lines(n)</a><br />
<code class="varname">c</code> = flow-out  ⇒ <a href="#s-separate-lines(n)">s-separate-lines(n)</a><br />
<code class="varname">c</code> = flow-in   ⇒ <a href="#s-separate-lines(n)">s-separate-lines(n)</a><br />
<code class="varname">c</code> = flow-key  ⇒ <a href="#s-separate-spaces">s-separate-spaces</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- YAML usually allows separation spaces to include a <span id="id893173" class="indexterm"></span>[comment](#comment/syntax) ending the line and additional <span id="id893188" class="indexterm"></span>[comment](#comment/syntax) lines. Note that the token following the separation <span id="id893204" class="indexterm"></span>[comment](#comment/syntax) lines must be properly <span id="id893219" class="indexterm"></span>[indented](#indentation%20space/), even though there is no such restriction on the separation <span id="id893234" class="indexterm"></span>[comment](#comment/syntax) lines themselves.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[74]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-separate-lines(n)"></span>s-separate-lines(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#s-ignored-space">s-ignored-space</a>+<br />
| ( <a href="#s-l-comments">s-l-comments</a> <a href="#s-indent(n)">s-indent(n)</a> <a href="#s-ignored-space">s-ignored-space</a>* )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Inside <span id="id893293" class="indexterm"></span>[simple keys](#simple%20key/), however, separation spaces are confined to the current line.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[75]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-separate-spaces"></span>s-separate-spaces</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-space">s-ignored-space</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id893330"></span>

**Example 6.5. Separation Spaces**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>{·first:·Sammy,·last:·Sosa·}:↓
# Statistics:
··hr:··# Home runs
····65
··avg:·# Average
····0.278</code></pre>
<pre class="synopsis"><code>Legend:
  s-separate-spaces
  s-separate-lines(n)
  s-indent(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!map {
    ? !!str &quot;first&quot;
    : !!str &quot;Sammy&quot;,
    ? !!str &quot;last&quot;
    : !!str &quot;Sosa&quot;
  }
  : !!map {
    ? !!str &quot;hr&quot;
    : !!int &quot;65&quot;,
    ? !!str &quot;avg&quot;
    : !!float &quot;0.278&quot;
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id893482"></span>6.4. Ignored Line Prefix

</div>

</div>

</div>

YAML discards the “<span class="quote">empty</span>” <span id="id893494" class="indexterm"></span><span id="ignored line prefix/"></span>*prefix* of each <span id="id893511" class="indexterm"></span>[scalar content](#scalar/syntax) line. This prefix always includes the <span id="id893526" class="indexterm"></span>[indentation](#indentation%20space/), and depending on the scalar style may also include all leading <span id="id893541" class="indexterm"></span>[white space](#white%20space/). The ignored prefix is a <span id="id893554" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must never be considered part of a <span id="id893568" class="indexterm"></span>[node’s content information](#content/information%20model).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[76]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ignored-prefix(n,s)"></span>s-ignored-prefix(n,s)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">s</code> = plain   ⇒ <a href="#s-ignored-prefix-plain(n)">s-ignored-prefix-plain(n)</a><br />
<code class="varname">s</code> = double  ⇒ <a href="#s-ignored-prefix-quoted(n)">s-ignored-prefix-quoted(n)</a><br />
<code class="varname">s</code> = single  ⇒ <a href="#s-ignored-prefix-quoted(n)">s-ignored-prefix-quoted(n)</a><br />
<code class="varname">s</code> = literal ⇒ <a href="#s-ignored-prefix-block(n)">s-ignored-prefix-block(n)</a><br />
<code class="varname">s</code> = folded  ⇒ <a href="#s-ignored-prefix-block(n)">s-ignored-prefix-block(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Plain scalars must not contain any <span id="id893656" class="indexterm"></span>[tab](#tab/) characters, and all leading spaces are always discarded.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[77]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ignored-prefix-plain(n)"></span>s-ignored-prefix-plain(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(n)</a> <a href="#s-ignored-space">s-ignored-space</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Quoted scalars may contain <span id="id893701" class="indexterm"></span>[tab](#tab/) characters. Again, all leading <span id="id893714" class="indexterm"></span>[white space](#white%20space/) is always discarded.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[78]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ignored-prefix-quoted(n)"></span>s-ignored-prefix-quoted(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(n)</a> <a href="#s-ignored-white">s-ignored-white</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Block scalars rely on <span id="id893759" class="indexterm"></span>[indentation](#indentation%20space/); hence leading <span id="id893775" class="indexterm"></span>[white space](#white%20space/), if any, is not discarded.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[79]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ignored-prefix-block(n)"></span>s-ignored-prefix-block(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id893810"></span>

**Example 6.6. Ignored Prefix**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>plain: text
··lines
quoted: &quot;text
··→lines&quot;
block: |
··text
···→lines</code></pre>
<pre class="synopsis"><code>Legend:
  s-ignored-prefix-plain(n)
  s-ignored-prefix-quoted(n)
  s-ignored-prefix-block(n)
  s-indent(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;plain&quot;
  : !!str &quot;text lines&quot;,
  ? !!str &quot;quoted&quot;
  : !!str &quot;text lines&quot;,
  ? !!str &quot;block&quot;
  : !!str &quot;text·→lines\n&quot;
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

An <span id="id893944" class="indexterm"></span><span id="empty line/"></span>*empty line* line consists of the ignored prefix followed by a <span id="id893956" class="indexterm"></span>[line break](#line%20break%20character/). When trailing <span id="id893972" class="indexterm"></span>[block scalars](#block%20scalar%20style/syntax), such lines can also be interpreted as (empty) <span id="id893987" class="indexterm"></span>[comment](#comment/syntax) lines. YAML provides a <span id="id894002" class="indexterm"></span>[chomping](#chomping/) mechanism to resolve this ambiguity.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[80]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-empty(n,s)"></span>l-empty(n,s)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#s-indent(n)">s-indent(&lt;n)</a> | <a href="#s-ignored-prefix(n,s)">s-ignored-prefix(n,s)</a> )<br />
<a href="#b-normalized">b-normalized</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id894049"></span>

**Example 6.7. Empty Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- foo
·↓
  bar
- |-
  foo
·↓
  bar
··↓</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq {
  !!str &quot;foo\nbar&quot;,
  !!str &quot;foo\n\nbar&quot;
}</code></pre>
<pre class="synopsis"><code>Legend:
  l-empty(n,s)
  l-comment</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id894136"></span>6.5. Line Folding

</div>

</div>

</div>

<span id="id894144" class="indexterm"></span><span id="line folding/"></span>*Line folding* allows long lines to be broken for readability, while retaining the original semantics of a single long line. When folding is done, any <span id="id894160" class="indexterm"></span>[line break](#line%20break%20character/) ending an <span id="id894174" class="indexterm"></span>[empty line](#empty%20line/) is preserved. In addition, any <span id="id894187" class="indexterm"></span>[specific line breaks](#specific%20line%20break/) are also preserved, even when ending a non-<span id="id894201" class="indexterm"></span>[empty line](#empty%20line/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[81]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-l-folded-specific(n,s)"></span>b-l-folded-specific(n,s)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-specific">b-specific</a> <a href="#l-empty(n,s)">l-empty(n,s)</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Hence, folding only applies to <span id="id894242" class="indexterm"></span>[generic line breaks](#generic%20line%20break/) that end non-<span id="id894257" class="indexterm"></span>[empty lines](#empty%20line/). If the following line is also not <span id="id894269" class="indexterm"></span>[empty](#empty%20line/), the <span id="id894281" class="indexterm"></span>[generic line break](#generic%20line%20break/) is converted to a single space (**`#x20`**).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[82]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-l-folded-as-space"></span>b-l-folded-as-space</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-generic">b-generic</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

If the following line is <span id="id894325" class="indexterm"></span>[empty line](#empty%20line/), the <span id="id894338" class="indexterm"></span>[generic line break](#generic%20line%20break/) is ignored.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[83]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-l-folded-trimmed(n,s)"></span>b-l-folded-trimmed(n,s)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-ignored-generic">b-ignored-generic</a> <a href="#l-empty(n,s)">l-empty(n,s)</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Thus, a folded non-<span id="id894380" class="indexterm"></span>[empty line](#empty%20line/) may end with one of three possible folded line break forms. The original form of such a folded line break is a <span id="id894395" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not be used to convey <span id="id894409" class="indexterm"></span>[node’s content information](#content/information%20model).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[84]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-l-folded-any(n,s)"></span>b-l-folded-any(n,s)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#b-l-folded-specific(n,s)">b-l-folded-specific(n,s)</a><br />
| <a href="#b-l-folded-as-space">b-l-folded-as-space</a><br />
| <a href="#b-l-folded-trimmed(n,s)">b-l-folded-trimmed(n,s)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id894461"></span>

**Example 6.8. Line Folding**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&gt;-
  specific⇓
  trimmed↓
··↓
·↓
↓
  as↓
  space</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
--- !!str
&quot;specific\L\
 trimmed\n\n\n\
 as space&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  b-l-folded-specific(n,s)
  b-l-folded-trimmed(n,s)
  b-l-folded-as-space</code></pre></td>
</tr>
</tbody>
</table>

</div>

The above rules are common to both the <span id="id894567" class="indexterm"></span>[folded block style](#folded%20style/syntax) and the <span id="id894583" class="indexterm"></span>[scalar flow styles](#flow%20scalar%20style/syntax). Folding does distinguish between the <span id="id894601" class="indexterm"></span>[folded block style](#folded%20style/syntax) and the <span id="id894617" class="indexterm"></span>[scalar flow styles](#flow%20scalar%20style/syntax) in the following way:

<div class="variablelist">

<span class="term">Block Folding</span>  
In the <span id="id894645" class="indexterm"></span>[folded block style](#folded%20style/syntax), folding does not apply to <span id="id894661" class="indexterm"></span>[line breaks](#line%20break%20character/) or <span id="id894674" class="indexterm"></span>[empty lines](#empty%20line/) that precede or follow a text line containing leading <span id="id894688" class="indexterm"></span>[white space](#white%20space/). Note that such a line may consist of only such leading <span id="id894702" class="indexterm"></span>[white space](#white%20space/); an <span id="id894714" class="indexterm"></span>[empty](#empty%20line/)<span id="id894726" class="indexterm"></span>[block](#block%20style/syntax) line is confined to (optional) <span id="id894743" class="indexterm"></span>[indentation](#indentation%20space/) spaces only. Further, the final <span id="id894758" class="indexterm"></span>[line break](#line%20break%20character/) and <span id="id894770" class="indexterm"></span>[empty lines](#empty%20line/) are subject to <span id="id894783" class="indexterm"></span>[chomping](#chomping/), and are never folded. The combined effect of these rules is that each “<span class="quote">paragraph</span>” is interpreted as a line, <span id="id894801" class="indexterm"></span>[empty lines](#empty%20line/) are used to <span id="id894813" class="indexterm"></span>[present](#present/) a line feed, the formatting of <span id="id894826" class="indexterm"></span>[“<span class="quote">more indented</span>” lines](#more%20indented%20line/) is preserved, and final <span id="id894843" class="indexterm"></span>[line breaks](#line%20break%20character/) may be included or excluded from the <span id="id894859" class="indexterm"></span>[node’s content information](#content/information%20model) as appropriate.

<span class="term">Flow Folding</span>  
Folding in <span id="id894885" class="indexterm"></span>[flow styles](#flow%20style/syntax) provides more relaxed, less powerful semantics. <span id="id894901" class="indexterm"></span>[Flow styles](#flow%20style/syntax) typically depend on explicit <span id="id894917" class="indexterm"></span>[indicators](#indicator/) to convey structure, rather than <span id="id894930" class="indexterm"></span>[indentation](#indentation%20space/). Hence, in <span id="id894945" class="indexterm"></span>[flow styles](#flow%20style/syntax), spaces preceding or following the text in a line are a <span id="id894962" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not be considered a part of the <span id="id894976" class="indexterm"></span>[node’s content information](#content/information%20model). Once all such spaces have been discarded, folding proceeds as described above. In contrast with the <span id="id894995" class="indexterm"></span>[block folded style](#folded%20style/syntax), all <span id="id895010" class="indexterm"></span>[line breaks](#line%20break%20character/) are folded, without exception, and a line consisting only of spaces is considered to be an <span id="id895025" class="indexterm"></span>[empty line](#empty%20line/), regardless of the number of spaces. The combined effect of these processing rules is that each “<span class="quote">paragraph</span>” is interpreted as a line, <span id="id895043" class="indexterm"></span>[empty lines](#empty%20line/) are used to <span id="id895056" class="indexterm"></span>[present](#present/) a line feed, and text can be freely <span id="id895069" class="indexterm"></span>[“<span class="quote">more indented</span>”](#more%20indented%20line/) without affecting the <span id="id895084" class="indexterm"></span>[node’s content information](#content/information%20model).

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id895107"></span>Chapter 7. YAML Character Stream

</div>

</div>

</div>

A YAML character <span id="id895115" class="indexterm"></span>[stream](#stream/syntax) may contain several YAML <span id="id895130" class="indexterm"></span>[documents](#document/syntax), denoted by <span id="id895146" class="indexterm"></span>[document boundary markers](#document%20boundary%20marker/). Each <span id="id895159" class="indexterm"></span>[document](#document/syntax) <span id="id895174" class="indexterm"></span>[presents](#present/) a single independent <span id="id895187" class="indexterm"></span>[root node](#root%20node/) and may be preceded by a series of <span id="id895200" class="indexterm"></span>[directives](#directive/syntax).

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id895217"></span>7.1. Directives

</div>

</div>

</div>

<span id="id895225" class="indexterm"></span><span id="directive/syntax"></span>*Directives* are instructions to the YAML <span id="id895242" class="indexterm"></span>[processor](#processor/). Like <span id="id895254" class="indexterm"></span>[comments](#comment/syntax), directives are <span id="id895270" class="indexterm"></span>[presentation details](#presentation%20detail/) and are not reflected in the <span id="id895284" class="indexterm"></span>[serialization tree](#serialization/) (and hence the <span id="id895297" class="indexterm"></span>[representation graph](#representation/)). This specification defines two directives, <span id="id895310" class="indexterm"></span>[“<span class="quote">**`YAML`**</span>”](#YAML%20directive/) and <span id="id895329" class="indexterm"></span>[“<span class="quote">**`TAG`**</span>”](#TAG%20directive/), and <span id="id895346" class="indexterm"></span><span id="reserved directive/"></span>*reserves* all other directives for future use. There is no way to define private directives. This is intentional.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[85]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-directive"></span>l-directive</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-yaml-directive">l-yaml-directive</a> | <a href="#l-tag-directive">l-tag-directive</a> | <a href="#l-reserved-directive">l-reserved-directive</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Each directive is specified on a separate non-<span id="id895395" class="indexterm"></span>[indented](#indentation%20space/) line starting with the <span id="id895409" class="indexterm"></span><span id="% directive/"></span>*“<span class="quote">**`%`**</span>” indicator*, followed by the directive name and a space-separated list of parameters. The semantics of these tokens depend on the specific directive. A YAML <span id="id895431" class="indexterm"></span>[processor](#processor/) should ignore unknown directives with an appropriate warning.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[86]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-reserved-directive"></span>l-reserved-directive</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-directive">“<span class="quote">%</span>”</a> <a href="#ns-directive-name">ns-directive-name</a><br />
( <a href="#s-ignored-space">s-ignored-space</a>+ <a href="#ns-directive-parameter">ns-directive-parameter</a> )*<br />
<a href="#s-l-comments">s-l-comments</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[87]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-directive-name"></span>ns-directive-name</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-char">ns-char</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[88]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-directive-parameter"></span>ns-directive-parameter</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-char">ns-char</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id895525"></span>

**Example 7.1. Reserved Directives**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>%FOO  bar baz # Should be ignored
               # with a warning.
--- &quot;foo&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
--- !!str
&quot;foo&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-reserved-directive
  ns-directive-name
  ns-directive-parameter</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id895631"></span>7.1.1. “<span class="quote">**`YAML`**</span>” Directive

</div>

</div>

</div>

The <span id="id895645" class="indexterm"></span><span id="YAML directive/"></span>*“<span class="quote">**`YAML`**</span>” directive* specifies the version of YAML the <span id="id895665" class="indexterm"></span>[document](#document/syntax) adheres to. This specification defines version “<span class="quote">**`1.1`**</span>”. A version 1.1 YAML <span id="id895688" class="indexterm"></span>[processor](#processor/) should accept <span id="id895701" class="indexterm"></span>[documents](#document/syntax) with an explicit “<span class="quote">**`%YAML 1.1`**</span>” directive, as well as <span id="id895723" class="indexterm"></span>[documents](#document/syntax) lacking a “<span class="quote">**`YAML`**</span>” directive. <span id="id895745" class="indexterm"></span>[Documents](#document/syntax) with a “<span class="quote">**`YAML`**</span>” directive specifying a higher minor version (e.g. “<span class="quote">**`%YAML 1.2`**</span>”) should be processed with an appropriate warning. <span id="id895776" class="indexterm"></span>[Documents](#document/syntax) with a “<span class="quote">**`YAML`**</span>” directive specifying a higher major version (e.g. “<span class="quote">**`%YAML 2.0`**</span>”) should be rejected with an appropriate error message.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[89]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-yaml-directive"></span>l-yaml-directive</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-directive">“<span class="quote">%</span>”</a> “<span class="quote">Y</span>” “<span class="quote">A</span>” “<span class="quote">M</span>” “<span class="quote">L</span>”<br />
<a href="#s-ignored-space">s-ignored-space</a>+ <a href="#ns-yaml-version">ns-yaml-version</a><br />
<a href="#s-l-comments">s-l-comments</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[90]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-yaml-version"></span>ns-yaml-version</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-dec-digit">ns-dec-digit</a>+ “<span class="quote">.</span>” <a href="#ns-dec-digit">ns-dec-digit</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id895885"></span>

**Example 7.2. “<span class="quote">**`YAML`**</span>” directive**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>%YAML 1.2 # Attempt parsing
           # with a warning
---
&quot;foo&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;foo&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-yaml-directive ns-yaml-version</code></pre></td>
</tr>
</tbody>
</table>

</div>

It is an error to specify more than one “<span class="quote">**`YAML`**</span>” directive for the same document, even if both occurrences give the same version number.

<div class="example">

<span id="id895987"></span>

**Example 7.3. Invalid Repeated YAML directive**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>%YAML 1.1
%YAML 1.1
foo</code></pre></td>
<td><pre class="screen"><code>ERROR:
The YAML directive must only be
given at most once per document.</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id896044"></span>7.1.2. “<span class="quote">**`TAG`**</span>” Directive

</div>

</div>

</div>

The <span id="id896057" class="indexterm"></span><span id="TAG directive/"></span>*“<span class="quote">**`TAG`**</span>” directive* establishes a <span id="id896076" class="indexterm"></span>[shorthand](#tag%20shorthand/) notation for specifying <span id="id896091" class="indexterm"></span>[node tags](#tag/syntax). Each “<span class="quote">**`TAG`**</span>” directive associates a <span id="id896111" class="indexterm"></span>[handle](#tag%20handle/) with a <span id="id896126" class="indexterm"></span>[prefix](#tag%20prefix/), allowing for compact and readable <span id="id896141" class="indexterm"></span>[tag](#tag/syntax) notation.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[91]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-tag-directive"></span>l-tag-directive</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-directive">“<span class="quote">%</span>”</a> “<span class="quote">T</span>” “<span class="quote">A</span>” “<span class="quote">G</span>”<br />
<a href="#s-ignored-space">s-ignored-space</a>+ <a href="#c-tag-handle">c-tag-handle</a><br />
<a href="#s-ignored-space">s-ignored-space</a>+ <a href="#ns-tag-prefix">ns-tag-prefix</a><br />
<a href="#s-l-comments">s-l-comments</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id896213"></span>

**Example 7.4. “<span class="quote">**`TAG`**</span>” directive**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>%TAG !yaml! tag:yaml.org,2002:↓
---
!yaml!str &quot;foo&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;foo&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-tag-directive
  c-tag-handle ns-tag-prefix</code></pre></td>
</tr>
</tbody>
</table>

</div>

It is an error to specify more than one “<span class="quote">**`TAG`**</span>” directive for the same <span id="id896326" class="indexterm"></span>[handle](#tag%20handle/) in the same document, even if both occurrences give the same <span id="id896342" class="indexterm"></span>[prefix](#tag%20prefix/).

<div class="example">

<span id="id896356"></span>

**Example 7.5. Invalid Repeated TAG directive**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>%TAG ! !foo
%TAG ! !foo
bar</code></pre></td>
<td><pre class="screen"><code>ERROR:
The TAG directive must only
be given at most once per
handle in the same document.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id896411"></span>7.1.2.1. Tag Prefixes

</div>

</div>

</div>

There are two <span id="id896420" class="indexterm"></span><span id="tag prefix/"></span>*tag prefix* variants:

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[92]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-tag-prefix"></span>ns-tag-prefix</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-local-tag-prefix">ns-local-tag-prefix</a> | <a href="#ns-global-tag-prefix">ns-global-tag-prefix</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Local Tags</span>  
If the prefix begins with a <span id="id896469" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20local%20tag/) character, <span id="id896489" class="indexterm"></span>[shorthands](#tag%20shorthand/) using the <span id="id896501" class="indexterm"></span>[handle](#tag%20handle/) are expanded to a <span id="id896513" class="indexterm"></span>[local tag](#local%20tag/) beginning with <span id="id896526" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20local%20tag/). Note that such a <span id="id896545" class="indexterm"></span>[tag](#tag/syntax) is intentionally not a valid URI, since its semantics are specific to the <span id="id896561" class="indexterm"></span>[application](#application/). In particular, two <span id="id896573" class="indexterm"></span>[documents](#document/syntax) in the same <span id="id896588" class="indexterm"></span>[stream](#stream/syntax) may assign different semantics to the same <span id="id896603" class="indexterm"></span>[local tag](#local%20tag/).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[93]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-local-tag-prefix"></span>ns-local-tag-prefix</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-tag">“<span class="quote">!</span>”</a> <a href="#ns-uri-char">ns-uri-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Global Tags</span>  
If the prefix begins with a character other than <span id="id896657" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20local%20tag/), it must to be a valid URI prefix, and should contain at least the scheme and the authority. <span id="id896676" class="indexterm"></span>[Shorthands](#tag%20shorthand/) using the associated <span id="id896691" class="indexterm"></span>[handle](#tag%20handle/) are expanded to globally unique URI tags, and their semantics is consistent across <span id="id896704" class="indexterm"></span>[applications](#application/). In particular, two <span id="id896717" class="indexterm"></span>[documents](#document/syntax) in different <span id="id896732" class="indexterm"></span>[streams](#stream/syntax) must assign the same semantics to the same <span id="id896747" class="indexterm"></span>[global tag](#global%20tag/).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[94]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-global-tag-prefix"></span>ns-global-tag-prefix</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-tag-char">ns-tag-char</a> <a href="#ns-uri-char">ns-uri-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id896788"></span>

**Example 7.6. Tag Prefixes**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>%TAG !      !foo
%TAG !yaml! tag:yaml.org,2002:
---
- !bar &quot;baz&quot;
- !yaml!str &quot;string&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  ns-local-tag-prefix ns-global-tag-prefix</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !&lt;!foobar&gt; &quot;baz&quot;,
  !&lt;tag:yaml.org,2002:str&gt; &quot;string&quot;
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect3" lang="en">

<div class="titlepage">

<div>

<div>

#### <span id="id896876"></span>7.1.2.2. Tag Handles

</div>

</div>

</div>

The <span id="id896884" class="indexterm"></span><span id="tag handle/"></span>*tag handle* exactly matches the prefix of the affected <span id="id896897" class="indexterm"></span>[shorthand](#tag%20shorthand/). There are three tag handle variants:

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[95]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-tag-handle"></span>c-tag-handle</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-primary-tag-handle">c-primary-tag-handle</a><br />
| <a href="#c-secondary-tag-handle">ns-secondary-tag-handle</a><br />
| <a href="#c-named-tag-handle">c-named-tag-handle</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Primary Handle</span>  
The <span id="id896955" class="indexterm"></span><span id="primary tag handle/"></span>*primary tag handle* is a single <span id="id896971" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20tag%20indicator/) character. This allows using the most compact possible notation for a single “<span class="quote">primary</span>” name space. By default, the prefix associated with this handle is <span id="id896997" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20local%20tag/). Thus, by default, <span id="id897014" class="indexterm"></span>[shorthands](#tag%20shorthand/) using this handle are interpreted as <span id="id897026" class="indexterm"></span>[local tags](#local%20tag/). It is possible to override this behavior by providing an explicit “<span class="quote">**`TAG`**</span>” directive associating a different prefix for this handle. This provides smooth migration from using <span id="id897048" class="indexterm"></span>[local tags](#local%20tag/) to using <span id="id897063" class="indexterm"></span>[global tags](#global%20tag/) by a simple addition of a single “<span class="quote">**`TAG`**</span>” directive.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[96]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-primary-tag-handle"></span>c-primary-tag-handle</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-tag">“<span class="quote">!</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id897109"></span>

**Example 7.7. Migrating from Local to Global Tags**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code># Private application:
!foo &quot;bar&quot;</code></pre>
<pre class="programlisting"><code># Migrated to global:
%TAG ! tag:ben-kiki.org,2000:app/
---
!foo &quot;bar&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!&lt;!foo&gt; &quot;bar&quot;</code></pre>
<pre class="programlisting"><code>%YAML 1.1
---
!&lt;tag:ben-kiki.org,2000:app/foo&gt; &quot;bar&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="variablelist">

<span class="term">Secondary Handle</span>  
The <span id="id897186" class="indexterm"></span><span id="secondary tag handle/"></span>*secondary tag handle* is written as <span id="id897202" class="indexterm"></span>[“<span class="quote">**`!!`**</span>”](#!%20tag%20indicator/). This allows using a compact notation for a single “<span class="quote">secondary</span>” name space. By default, the prefix associated with this handle is “<span class="quote">**`tag:yaml.org,2002:`**</span>” used by the <a href="/type/index.html" target="_top">YAML tag repository</a> providing recommended <span id="id897240" class="indexterm"></span>[tags](#tag/information%20model) for increasing the portability of YAML <span id="id897257" class="indexterm"></span>[documents](#document/syntax) between different <span id="id897272" class="indexterm"></span>[applications](#application/). It is possible to override this behavior by providing an explicit “<span class="quote">**`TAG`**</span>” directive associating a different prefix for this handle.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[97]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-secondary-tag-handle"></span>ns-secondary-tag-handle</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-tag">“<span class="quote">!</span>”</a> <a href="#c-tag">“<span class="quote">!</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Named Handles</span>  
A <span id="id897336" class="indexterm"></span><span id="named tag handle/"></span>*named tag handle* surrounds the non-empty name with <span id="id897352" class="indexterm"></span><span id="! named handle/"></span>*“<span class="quote">**`!`**</span>”* characters. A handle name must not be used in a <span id="id897371" class="indexterm"></span>[shorthand](#tag%20shorthand/) unless an explicit “<span class="quote">**`TAG`**</span>” directive has associated some prefix with it. The name of the handle is a <span id="id897392" class="indexterm"></span>[presentation detail](#presentation%20detail/) and is not part of the <span id="id897406" class="indexterm"></span>[node’s content information](#content/information%20model). In particular, the YAML <span id="id897424" class="indexterm"></span>[processor](#processor/) need not preserve the handle name once <span id="id897437" class="indexterm"></span>[parsing](#parse/) is completed.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[98]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-named-tag-handle"></span>c-named-tag-handle</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-tag">“<span class="quote">!</span>”</a> <a href="#ns-word-char">ns-word-char</a>+ <a href="#c-tag">“<span class="quote">!</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id897487"></span>

**Example 7.8. Tag Handles**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code># Explicitly specify default settings:
%TAG !     !
%TAG !!    tag:yaml.org,2002:
# Named handles have no default:
%TAG !o! tag:ben-kiki.org,2000:
---
- !foo &quot;bar&quot;
- !!str &quot;string&quot;
- !o!type &quot;baz&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !&lt;!foo&gt; &quot;bar&quot;,
  !&lt;tag:yaml.org,2002:str&gt; &quot;string&quot;
  !&lt;tag:ben-kiki.org,2000:type&gt; &quot;baz&quot;
]</code></pre>
<pre class="synopsis"><code>Legend:
  c-primary-tag-handle
  c-secondary-tag-handle
  c-named-tag-handle</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id897596"></span>7.2. Document Boundary Markers

</div>

</div>

</div>

YAML <span id="id897604" class="indexterm"></span>[streams](#stream/syntax) use <span id="id897619" class="indexterm"></span><span id="document boundary marker/"></span>*document boundary markers* to allow more than one <span id="id897634" class="indexterm"></span>[document](#document/syntax) to be contained in the same <span id="id897650" class="indexterm"></span>[stream](#stream/syntax). Such markers are a <span id="id897665" class="indexterm"></span>[presentation detail](#presentation%20detail/) and are used exclusively to convey structure. A line beginning with “<span class="quote">**`---`**</span>” may be used to explicitly denote the beginning of a new YAML <span id="id897688" class="indexterm"></span>[document](#document/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[99]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-document-start"></span>c-document-start</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">-</span>” “<span class="quote">-</span>” “<span class="quote">-</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

When YAML is used as the format of a communication channel, it is useful to be able to indicate the end of a <span id="id897733" class="indexterm"></span>[document](#document/syntax) without closing the <span id="id897749" class="indexterm"></span>[stream](#stream/syntax), independent of starting the next <span id="id897764" class="indexterm"></span>[document](#document/syntax). Lacking such a marker, the YAML <span id="id897780" class="indexterm"></span>[processor](#processor/) reading the <span id="id897792" class="indexterm"></span>[stream](#stream/syntax) would be forced to wait for the header of the next <span id="id897808" class="indexterm"></span>[document](#document/syntax) (that may be long time in coming) in order to detect the end of the previous one. To support this scenario, a YAML <span id="id897825" class="indexterm"></span>[document](#document/syntax) may be terminated by an explicit end line denoted by “<span class="quote">**`...`**</span>”, followed by optional <span id="id897848" class="indexterm"></span>[comments](#comment/syntax). To ease the task of concatenating YAML <span id="id897864" class="indexterm"></span>[streams](#stream/syntax), the end marker may be repeated.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[100]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-document-end"></span>c-document-end</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">“<span class="quote">.</span>” “<span class="quote">.</span>” “<span class="quote">.</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[101]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-document-suffix"></span>l-document-suffix</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#c-document-end">c-document-end</a> <a href="#s-l-comments">s-l-comments</a> )+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id897927"></span>

**Example 7.9. Document Boundary Markers**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>---↓
foo
...
# Repeated end marker.
...↓
---↓
bar
# No end marker.
---↓
baz
...↓</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;foo&quot;
%YAML 1.1
---
!!str &quot;bar&quot;
%YAML 1.1
---
!!str &quot;baz&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  c-document-start l-document-suffix</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id898031"></span>7.3. Documents

</div>

</div>

</div>

A YAML <span id="id898040" class="indexterm"></span><span id="document/syntax"></span>*document* is a single native data structure <span id="id898056" class="indexterm"></span>[presented](#present/) as a single <span id="id898069" class="indexterm"></span>[root](#root%20node/) <span id="id898081" class="indexterm"></span>[node](#node/syntax). <span id="id898096" class="indexterm"></span>[Presentation details](#presentation%20detail/) such as <span id="id898111" class="indexterm"></span>[directives](#directive/syntax), <span id="id898125" class="indexterm"></span>[comments](#comment/syntax), <span id="id898139" class="indexterm"></span>[indentation](#indentation%20space/) and <span id="id898153" class="indexterm"></span>[styles](#style/) are not considered part of the <span id="id898165" class="indexterm"></span>[content information](#content/information%20model) of the document.

<div class="variablelist">

<span class="term">Explicit Documents</span>  
An <span id="id898193" class="indexterm"></span><span id="explicit document/"></span>*explicit document* begins with a <span id="id898209" class="indexterm"></span>[document start marker](#document%20boundary%20marker/) followed by the <span id="id898225" class="indexterm"></span>[presentation](#presentation/) of the <span id="id898236" class="indexterm"></span>[root node](#root%20node/). The <span id="id898248" class="indexterm"></span>[node](#node/syntax) may begin in the same line as the <span id="id898264" class="indexterm"></span>[document start marker](#document%20boundary%20marker/). If the explicit document’s <span id="id898270" class="indexterm"></span>[node](#node/syntax) is <span id="id898294" class="indexterm"></span>[completely empty](#completely%20empty%20node/), it is assumed to be an empty <span id="id898312" class="indexterm"></span>[plain scalar](#plain%20style/syntax) with no specified <span id="id898326" class="indexterm"></span>[properties](#node%20property/). Optional <span id="id898341" class="indexterm"></span>[document end marker(s)](#document%20boundary%20marker/) may follow the document.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[102]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-explicit-document"></span>l-explicit-document</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-document-start">c-document-start</a><br />
( <a href="#s-l+block-node(n,c)">s-l+block-node(-1,block-in)</a> | <a href="#s-l-empty-block">s-l-empty-block</a> )<br />
<a href="#l-document-suffix">l-document-suffix</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Implicit Documents</span>  
An <span id="id898408" class="indexterm"></span><span id="implicit document/"></span>*implicit document* does not begin with a <span id="id898423" class="indexterm"></span>[document start marker](#document%20boundary%20marker/). In this case, the <span id="id898437" class="indexterm"></span>[root node](#root%20node/) must not be <span id="id898452" class="indexterm"></span>[presented](#present/) as a <span id="id898463" class="indexterm"></span>[completely empty node](#completely%20empty%20node/). Again, optional <span id="id898476" class="indexterm"></span>[document end marker(s)](#document%20boundary%20marker/) may follow the document.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[103]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-implicit-document"></span>l-implicit-document</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-space">s-ignored-space</a>* <a href="#ns-l+block-node(n,c)">ns-l+block-node(-1,block-in)</a><br />
<a href="#l-document-suffix">l-document-suffix</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In general, the document’s <span id="id898482" class="indexterm"></span>[node](#node/syntax) is <span id="id898545" class="indexterm"></span>[indented](#indentation%20space/) as if it has a parent <span id="id898561" class="indexterm"></span>[indented](#indentation%20space/) at -1 spaces. Since a <span id="id898573" class="indexterm"></span>[node](#node/syntax) must be more <span id="id898588" class="indexterm"></span>[indented](#indentation%20space/) that its parent <span id="id898601" class="indexterm"></span>[node](#node/syntax), this allows the document’s <span id="id898618" class="indexterm"></span>[node](#node/syntax) to be <span id="id898632" class="indexterm"></span>[indented](#indentation%20space/) at zero or more spaces. Note that <span id="id898648" class="indexterm"></span>[flow scalar](#flow%20scalar%20style/syntax) continuation lines must be <span id="id898663" class="indexterm"></span>[indented](#indentation%20space/) by at least one space, even if their first line is not <span id="id898679" class="indexterm"></span>[indented](#indentation%20space/).

<div class="example">

<span id="id898692"></span>

**Example 7.10. Documents**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&quot;Root flow
 scalar&quot;
--- !!str &gt;
 Root block
 scalar
---
# Root collection:
foo : bar
... # Is optional.
---
# Explicit document may be empty.</code></pre>
<pre class="synopsis"><code>Legend:
  l-implicit-document l-explicit-document</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;Root flow scalar&quot;
%YAML 1.1
---
!!str &quot;Root block scalar&quot;
%YAML 1.1
---
!!map {
  ? !!str &quot;foo&quot;
  : !!str &quot;bar&quot;
}
---
!!str &quot;&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id898785"></span>7.4. Complete Stream

</div>

</div>

</div>

A sequence of bytes is a YAML character <span id="id898794" class="indexterm"></span><span id="stream/syntax"></span>*stream* if, taken as a whole, it complies with the [**`l-yaml-stream`**](#l-yaml-stream) production. The stream begins with a prefix containing an optional <span id="id898823" class="indexterm"></span>[byte order mark](#byte%20order%20mark/) denoting its <span id="id898838" class="indexterm"></span>[character encoding](#character%20encoding/), followed by optional <span id="id898852" class="indexterm"></span>[comments](#comment/syntax). Note that the stream may contain no <span id="id898867" class="indexterm"></span>[documents](#document/syntax), even if it contains a non-empty prefix. In particular, a stream containing no characters is valid and contains no <span id="id898884" class="indexterm"></span>[documents](#document/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[104]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-yaml-stream"></span>l-yaml-stream</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-byte-order-mark">c-byte-order-mark</a>? <a href="#l-comment">l-comment</a>*<br />
( <a href="#l-first-document">l-first-document</a> <a href="#l-next-document">l-next-document</a>* )?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id898937"></span>

**Example 7.11. Empty Stream**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>⇔# A stream may contain
# no documents.</code></pre>
<pre class="synopsis"><code>Legend:
  l-yaml-stream</code></pre></td>
<td><pre class="programlisting"><code># This stream contains no
# documents, only comments.</code></pre></td>
</tr>
</tbody>
</table>

</div>

The first <span id="id899004" class="indexterm"></span>[document](#document/syntax) may be <span id="id899019" class="indexterm"></span>[implicit](#implicit%20document/) (omit the <span id="id899034" class="indexterm"></span>[document start marker](#document%20boundary%20marker/)). In such a case it must not specify any <span id="id899048" class="indexterm"></span>[directives](#directive/syntax) and will be <span id="id899064" class="indexterm"></span>[parsed](#parse/) using the default settings. If the <span id="id899076" class="indexterm"></span>[document](#document/syntax) is <span id="id899091" class="indexterm"></span>[explicit](#explicit%20document/) (begins with an <span id="id899106" class="indexterm"></span>[document start marker](#document%20boundary%20marker/)), it may specify <span id="id899120" class="indexterm"></span>[directives](#directive/syntax) to control its <span id="id899136" class="indexterm"></span>[parsing](#parse/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[105]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-first-document"></span>l-first-document</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#l-implicit-document">l-implicit-document</a><br />
| ( <a href="#l-directive">l-directive</a>* <a href="#l-explicit-document">l-explicit-document</a> ) )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id899182"></span>

**Example 7.12. First Document**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code># Implicit document. Root
# collection (mapping) node.
foo : bar</code></pre>
<pre class="programlisting"><code># Explicit document. Root
# scalar (literal) node.
--- |
 Text content</code></pre>
<pre class="synopsis"><code>Legend:
  l-first-document</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;foo&quot;
  : !!str &quot;bar&quot;
}</code></pre>
<pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;Text content\n&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

To ease the task of concatenating character streams, following <span id="id899282" class="indexterm"></span>[documents](#document/syntax) may begin with a <span id="id899297" class="indexterm"></span>[byte order mark](#byte%20order%20mark/) and <span id="id899310" class="indexterm"></span>[comments](#comment/syntax), though the same <span id="id899325" class="indexterm"></span>[character encoding](#character%20encoding/) must be used through the stream. Each following <span id="id899340" class="indexterm"></span>[document](#document/syntax) must be <span id="id899355" class="indexterm"></span>[explicit](#explicit%20document/) (begin with a <span id="id899369" class="indexterm"></span>[document start marker](#document%20boundary%20marker/)). If the <span id="id899382" class="indexterm"></span>[document](#document/syntax) specifies no <span id="id899397" class="indexterm"></span>[directives](#directive/syntax), it is <span id="id899413" class="indexterm"></span>[parsed](#parse/) using the same settings as the previous <span id="id899426" class="indexterm"></span>[document](#document/syntax). If the <span id="id899441" class="indexterm"></span>[document](#document/syntax) does specify any <span id="id899456" class="indexterm"></span>[directives](#directive/syntax), all <span id="id899472" class="indexterm"></span>[directives](#directive/syntax) of previous <span id="id899487" class="indexterm"></span>[documents](#document/syntax), if any, are ignored.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[106]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-next-document"></span>l-next-document</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-byte-order-mark">c-byte-order-mark</a>? <a href="#l-comment">l-comment</a>*<br />
<a href="#l-directive">l-directive</a>* <a href="#l-explicit-document">l-explicit-document</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id899540"></span>

**Example 7.13. Next Documents**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>! &quot;First document&quot;
---
!foo &quot;No directives&quot;
%TAG ! !foo
---
!bar &quot;With directives&quot;
%YAML 1.1
---
!baz &quot;Reset settings&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;First document&quot;
---
!&lt;!foo&gt; &quot;No directives&quot;
---
!&lt;!foobar&gt; &quot;With directives&quot;
---
!&lt;!baz&gt; &quot;Reset settings&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-next-document</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id899622"></span>Chapter 8. Nodes

</div>

</div>

</div>

Each <span id="id899630" class="indexterm"></span><span id="node/syntax"></span>*presentation node* may have two optional <span id="id899646" class="indexterm"></span><span id="node property/"></span>*properties*, <span id="id899662" class="indexterm"></span>[anchor](#anchor/syntax) and <span id="id899675" class="indexterm"></span>[tag](#tag/syntax), in addition to its <span id="id899690" class="indexterm"></span>[content](#content/syntax). Node properties may be specified in any order before the <span id="id899705" class="indexterm"></span>[node’s content](#content/syntax), and either or both may be omitted from the character <span id="id899722" class="indexterm"></span>[stream](#stream/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[107]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-ns-properties(n,c)"></span>c-ns-properties(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#c-ns-tag-property">c-ns-tag-property</a><br />
    ( <a href="#s-separate(n,c)">s-separate(n,c)</a> <a href="#c-ns-anchor-property">c-ns-anchor-property</a> )? )<br />
| ( <a href="#c-ns-anchor-property">c-ns-anchor-property</a><br />
    ( <a href="#s-separate(n,c)">s-separate(n,c)</a> <a href="#c-ns-tag-property">c-ns-tag-property</a> )? )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id899790"></span>

**Example 8.1. Node Properties**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>!!str
 &amp;a1↓
  &quot;foo&quot; : !!str bar
&amp;a2 baz : *a1</code></pre>
<pre class="synopsis"><code>Legend:
  c-ns-anchor-property c-ns-tag-property
  c-ns-properties(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? &amp;A1 !!str &quot;foo&quot;
  : !!str &quot;bar&quot;,
  ? !!str &amp;A2 &quot;baz&quot;
  : *a1
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id899912"></span>8.1. Node Anchors

</div>

</div>

</div>

The <span id="id899920" class="indexterm"></span><span id="anchor/syntax"></span>*anchor property* marks a <span id="id899935" class="indexterm"></span>[node](#node/syntax) for future reference. An anchor is denoted by the <span id="id899951" class="indexterm"></span><span id="& anchor/"></span>*“<span class="quote">**`&`**</span>” indicator*. An <span id="id899973" class="indexterm"></span>[alias node](#alias/syntax) can then be used to indicate additional inclusions of the anchored node by specifying its anchor. An anchored node need not be referenced by any <span id="id899989" class="indexterm"></span>[alias node](#alias/syntax); in particular, it is valid for all <span id="id900004" class="indexterm"></span>[nodes](#node/syntax) to be anchored.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[108]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-ns-anchor-property"></span>c-ns-anchor-property</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-anchor">“<span class="quote">&amp;</span>”</a> <a href="#ns-anchor-name">ns-anchor-name</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Note that as a <span id="id900049" class="indexterm"></span>[serialization detail](#serialization%20detail/), the anchor name is preserved in the <span id="id900064" class="indexterm"></span>[serialization tree](#serialization/). However, it is not reflected in the <span id="id900077" class="indexterm"></span>[representation](#representation/) graph and must not be used to convey <span id="id900090" class="indexterm"></span>[content information](#content/information%20model). In particular, the YAML <span id="id900109" class="indexterm"></span>[processor](#processor/) need not preserve the anchor name once the <span id="id900121" class="indexterm"></span>[representation](#representation/) is <span id="id900133" class="indexterm"></span>[composed](#compose/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[109]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-anchor-name"></span>ns-anchor-name</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-char">ns-char</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id900166"></span>

**Example 8.2. Node Anchors**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>First occurrence: &amp;anchor Value
Second occurrence: *anchor</code></pre>
<pre class="synopsis"><code>Legend:
  c-ns-anchor-property
  ns-anchor-name</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;First occurrence&quot;
  : &amp;A !!str &quot;Value&quot;,
  ? !!str &quot;Second occurrence&quot;
  : *A
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id900262"></span>8.2. Node Tags

</div>

</div>

</div>

The <span id="id900269" class="indexterm"></span><span id="tag/syntax"></span>*tag property* identifies the type of the native data structure <span id="id900285" class="indexterm"></span>[presented](#present/) by the <span id="id900297" class="indexterm"></span>[node](#node/syntax). A tag is denoted by the <span id="id900312" class="indexterm"></span><span id="! tag indicator/"></span>*“<span class="quote">**`!`**</span>” indicator*. In contrast with <span id="id900333" class="indexterm"></span>[anchors](#anchor/syntax), tags are an inherent part of the <span id="id900348" class="indexterm"></span>[representation](#representation/) graph.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[110]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-ns-tag-property"></span>c-ns-tag-property</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-verbatim-tag">c-verbatim-tag</a> | <a href="#c-ns-shorthand-tag">c-ns-shorthand-tag</a><br />
| <a href="#c-ns-non-specific-tag">c-ns-non-specific-tag</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Verbatim Tags</span>  
A tag may be written <span id="id900405" class="indexterm"></span><span id="verbatim tag/"></span>*verbatim* by surrounding it with the <span id="id900421" class="indexterm"></span><span id="< … > verbatim tag/"></span>*“<span class="quote">**`<`**</span>” and “<span class="quote">**`>`**</span>”* characters. In this case, the YAML <span id="id900449" class="indexterm"></span>[processor](#processor/) must deliver the verbatim tag as-is to the <span id="id900461" class="indexterm"></span>[application](#application/). In particular, verbatim tags are not subject to <span id="id900474" class="indexterm"></span>[tag resolution](#tag%20resolution/). A verbatim tag must either begin with a <span id="id900489" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20local%20tag/) (a <span id="id900506" class="indexterm"></span>[local tag](#local%20tag/)) or be a valid URI (a <span id="id900519" class="indexterm"></span>[global tag](#global%20tag/)).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[111]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-verbatim-tag"></span>c-verbatim-tag</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-tag">“<span class="quote">!</span>”</a> “<span class="quote">&lt;</span>” <a href="#ns-uri-char">ns-uri-char</a>+ “<span class="quote">&gt;</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id900566"></span>

**Example 8.3. Verbatim Tags**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>!&lt;tag:yaml.org,2002:str&gt; foo :
  !&lt;!bar&gt; baz</code></pre>
<pre class="synopsis"><code>Legend:
  c-verbatim-tag</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !&lt;tag:yaml.org,2002:str&gt; &quot;foo&quot;
  : !&lt;!bar&gt; &quot;baz&quot;
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id900640"></span>

**Example 8.4. Invalid Verbatim Tags**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>- !&lt;!&gt; foo
- !&lt;$:?&gt; bar</code></pre></td>
<td><pre class="screen"><code>ERROR:
- Verbatim tags aren&#39;t resolved,
  so ! is invalid.
- The $:? tag is neither a global
  URI tag nor a local tag starting
  with “!”.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="variablelist">

<span class="term">Tag Shorthands</span>  
A <span id="id900720" class="indexterm"></span><span id="tag shorthand/"></span>*tag shorthand* consists of a valid <span id="id900734" class="indexterm"></span>[tag handle](#tag%20handle/) followed by a non-empty suffix. The <span id="id900748" class="indexterm"></span>[tag handle](#tag%20handle/) must be associated with a <span id="id900761" class="indexterm"></span>[prefix](#tag%20prefix/), either by default or by using a <span id="id900774" class="indexterm"></span>[“<span class="quote">**`TAG`**</span>” directive](#TAG%20directive/). The resulting <span id="id900794" class="indexterm"></span>[parsed](#parse/) tag is the concatenation of the prefix and the suffix, and must either begin with <span id="id900807" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20local%20tag/) (a <span id="id900823" class="indexterm"></span>[local tag](#local%20tag/)) or be a valid URI (a <span id="id900836" class="indexterm"></span>[global tag](#global%20tag/)). When the <span id="id900849" class="indexterm"></span>[primary tag handle](#primary%20tag%20handle/) is used, the suffix must not contain any <span id="id900866" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20named%20handle/) character, as this would cause the tag shorthand to be interpreted as having a <span id="id900884" class="indexterm"></span>[named tag handle](#named%20tag%20handle/). If the <span id="id900897" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20named%20handle/) character exists in the suffix of a tag using the <span id="id900917" class="indexterm"></span>[primary tag handle](#primary%20tag%20handle/), it must be <span id="id900931" class="indexterm"></span>[escaped](#escaping%20in%20URI/) as <span id="id900943" class="indexterm"></span>[“<span class="quote">**`%21`**</span>”](#%%20escaping%20in%20URI/), and the parser should expand this particular escape sequence before passing the tag to the application. This behavior is consistent with the URI character quoting rules (specifically, section 1.3 of <a href="http://www.ietf.org/rfc/rfc2396.txt" target="_top">RFC2396</a>), and ensures the choice of <span id="id900973" class="indexterm"></span>[tag handle](#tag%20handle/) remains a <span id="id900985" class="indexterm"></span>[presentation detail](#presentation/) and is not reflected in the <span id="id900998" class="indexterm"></span>[serialization tree](#serialization/) (and hence the <span id="id901012" class="indexterm"></span>[representation](#representation/) graph). In particular, the <span id="id901025" class="indexterm"></span>[tag handle](#tag%20handle/) may be discarded once <span id="id901038" class="indexterm"></span>[parsing](#parse/) is completed.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[112]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-ns-shorthand-tag"></span>c-ns-shorthand-tag</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#c-primary-tag-handle">c-primary-tag-handle</a> <a href="#ns-tag-char">ns-tag-char</a>+ )<br />
| ( <a href="#c-secondary-tag-handle">ns-secondary-tag-handle</a> <a href="#ns-uri-char">ns-uri-char</a>+ )<br />
| ( <a href="#c-named-tag-handle">c-named-tag-handle</a> <a href="#ns-uri-char">ns-uri-char</a>+ )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id901102"></span>

**Example 8.5. Tag Shorthands**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>%TAG !o! tag:ben-kiki.org,2000:
---
- !local foo
- !!str bar
- !o!type baz</code></pre>
<pre class="synopsis"><code>Legend:
  c-ns-shorthand-tag</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !&lt;!local&gt; &quot;foo&quot;,
  !&lt;tag:yaml.org,2002:str&gt; &quot;bar&quot;,
  !&lt;tag:ben-kiki.org,2000:type&gt; &quot;baz&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id901185"></span>

**Example 8.6. Invalid Shorthand Tags**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>%TAG !o! tag:ben-kiki.org,2000:
---
- !$a!b foo
- !o! bar
- !h!type baz</code></pre></td>
<td><pre class="screen"><code>ERROR:
- The !$a! looks like a handle.
- The !o! handle has no suffix.
- The !h! handle wasn&#39;t declared.</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="variablelist">

<span class="term">Non-Specific Tags</span>  
If a <span id="id901274" class="indexterm"></span>[node](#node/syntax) has no tag property, it is assigned a <span id="id901288" class="indexterm"></span>[non-specific tag](#non-specific%20tag/): <span id="id901301" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20non-specific%20tag/) for <span id="id901321" class="indexterm"></span>[plain scalars](#plain%20style/syntax) and <span id="id901336" class="indexterm"></span>[“<span class="quote">**`!`**</span>”](#!%20non-specific%20tag/) for all other <span id="id901357" class="indexterm"></span>[nodes](#node/syntax). <span id="id901371" class="indexterm"></span>[Non-specific tags](#non-specific%20tag/) must be <span id="id901384" class="indexterm"></span>[resolved](#tag%20resolution/) to a <span id="id901397" class="indexterm"></span>[specific tag](#specific%20tag/) for a <span id="id901411" class="indexterm"></span>[complete representation](#complete%20representation/) graph to be <span id="id901425" class="indexterm"></span>[composed](#compose/). It is also possible for the tag property to explicitly specify the <span id="id901439" class="indexterm"></span>[node](#node/syntax) has the <span id="id901454" class="indexterm"></span>[“<span class="quote">**`!`**</span>” non-specific tag](#!%20non-specific%20tag/). This is only useful for <span id="id901476" class="indexterm"></span>[plain scalars](#plain%20style/syntax), causing them to be <span id="id901490" class="indexterm"></span>[resolved](#tag%20resolution/) as if they were non-<span id="id901502" class="indexterm"></span>[plain](#plain%20style/syntax) (hence, by the common <span id="id901518" class="indexterm"></span>[tag resolution](#tag%20resolution/) convention, as “<span class="quote">**`tag:yaml.org,2002:str`**</span>”). There is no way to explicitly set the tag to the <span id="id901539" class="indexterm"></span>[“<span class="quote">**`?`**</span>” non-specific](#?%20non-specific%20tag/) tag. This is intentional.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[113]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-ns-non-specific-tag"></span>c-ns-non-specific-tag</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-tag">“<span class="quote">!</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id901586"></span>

**Example 8.7. Non-Specific Tags**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code># Assuming conventional resolution:
- &quot;12&quot;
- 12
- ! 12</code></pre>
<pre class="synopsis"><code>Legend:
  c-ns-non-specific-tag</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !&lt;tag:yaml.org,2002:str&gt; &quot;12&quot;,
  !&lt;tag:yaml.org,2002:int&gt; &quot;12&quot;,
  !&lt;tag:yaml.org,2002:str&gt; &quot;12&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id901659"></span>8.3. Node Content

</div>

</div>

</div>

<span id="id901666" class="indexterm"></span><span id="content/syntax"></span>*Node content* may be <span id="id901680" class="indexterm"></span>[presented](#present/) in either a <span id="id901693" class="indexterm"></span>[flow style](#flow%20style/syntax) or a <span id="id901709" class="indexterm"></span>[block style](#block%20style/syntax). <span id="id901724" class="indexterm"></span>[Block content](#block%20style/syntax) always extends to the end of a line and uses <span id="id901741" class="indexterm"></span>[indentation](#indentation%20space/) to denote structure, while <span id="id901756" class="indexterm"></span>[flow content](#flow%20style/syntax) starts and ends at some non-space character within a line and uses <span id="id901772" class="indexterm"></span>[indicators](#indicator/) to denote structure. Each collection <span id="id901784" class="indexterm"></span>[kind](#kind/) can be presented in a single <span id="id901797" class="indexterm"></span>[flow collection style](#flow%20collection%20style/syntax) or a single <span id="id901813" class="indexterm"></span>[block collection style](#block%20collection%20style/syntax). However, each collection <span id="id901830" class="indexterm"></span>[kind](#kind/) also provides compact <span id="id901843" class="indexterm"></span>[in-line](#in-line%20style/syntax) forms for common cases. <span id="id901859" class="indexterm"></span>[Scalar content](#scalar/syntax) may be <span id="id901874" class="indexterm"></span>[presented](#present/) in the <span id="id901886" class="indexterm"></span>[plain style](#plain%20style/syntax) or one of the two <span id="id901905" class="indexterm"></span>[quoted styles](#quoted%20style/syntax) (the <span id="id901919" class="indexterm"></span>[single-quoted style](#single-quoted%20style/syntax) and the <span id="id901934" class="indexterm"></span>[double-quoted style](#double-quoted%20style/syntax)). Regardless of style, <span id="id901950" class="indexterm"></span>[scalar content](#scalar/syntax) must always be <span id="id901966" class="indexterm"></span>[indented](#indentation%20space/) by at least one space. In contrast, <span id="id901980" class="indexterm"></span>[collection content](#collection/syntax) need not be <span id="id901995" class="indexterm"></span>[indented](#indentation%20space/) (note that the <span id="id902009" class="indexterm"></span>[indentation](#indentation%20space/) of the first <span id="id902022" class="indexterm"></span>[flow scalar](#flow%20scalar%20style/syntax) line is determined by the <span id="id902039" class="indexterm"></span>[block collection](#block%20collection%20style/syntax) it is nested in, if any).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[114]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-flow-scalar(n,c)"></span>ns-flow-scalar(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#ns-plain(n,c)">c-plain(max(n,1),c)</a><br />
| <a href="#c-single-quoted(n,c)">c-single-quoted(max(n,1),c)</a><br />
| <a href="#c-double-quoted(n,c)">c-double-quoted(max(n,1),c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[115]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-flow-collection(n,c)"></span>c-flow-collection(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-flow-sequence(n,c)">c-flow-sequence(n,c)</a> | <a href="#c-flow-mapping(n,c)">c-flow-mapping(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[116]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-flow-content(n,c)"></span>ns-flow-content(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-flow-scalar(n,c)">ns-flow-scalar(n,c)</a> | <a href="#c-flow-collection(n,c)">c-flow-collection(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[117]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l+block-scalar(n)"></span>c-l+block-scalar(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-l+folded(n)">c-l+folded(max(n,0))</a> | <a href="#c-l+literal(n)">c-l+literal(max(n,0))</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[118]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l-block-collection(n,c)"></span>c-l-block-collection(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-l-block-sequence(n,c)">c-l-block-sequence(n,c)</a> | <a href="#c-l-block-mapping(n)">c-l-block-mapping(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[119]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l+block-content(n,c)"></span>c-l+block-content(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-l+block-scalar(n)">c-l+block-scalar(n)</a><br />
| <a href="#c-l-block-collection(n,c)">c-l-block-collection(&gt;n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id902212"></span>

**Example 8.8. Mandatory Scalar Indentation**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>---
foo:
·&quot;bar
·baz&quot;
---
&quot;foo
·bar&quot;
---
foo
·bar
--- |
·foo
...</code></pre>
<pre class="synopsis"><code>Legend:
  Normal &quot;more-indented&quot; indentation
  Mandatory for &quot;non-indented&quot; scalar</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;foo&quot;
  : !!str &quot;bar baz&quot;
}
%YAML 1.1
---
!!str &quot;foo bar&quot;
%YAML 1.1
---
!!str &quot;foo bar&quot;
%YAML 1.1
---
!!str &quot;foo bar\n&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id902304"></span>

**Example 8.9. Flow Content**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>---
scalars:
  plain: !!str some text↓
  quoted:
    single: &#39;some text&#39;↓
    double: &quot;some text&quot;↓
collections:
  sequence: !!seq [ !str entry,
    # Mapping entry:↓
      key: value ]↓
  mapping: { key: value }↓</code></pre>
<pre class="synopsis"><code>Legend:
  ns-flow-scalar
  c-flow-collection
  not content</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
--- !!map {
  ? !!str &quot;scalars&quot; : !!map {
      ? !!str &quot;plain&quot;
      : !!str &quot;some text&quot;,
      ? !!str &quot;quoted&quot;
      : !!map {
        ? !!str &quot;single&quot;
        : !!str &quot;some text&quot;,
        ? !!str &quot;double&quot;
        : !!str &quot;some text&quot;
  } },
  ? !!str &quot;collections&quot;: : !!map {
    ? !!str &quot;sequence&quot; : !!seq [
      ? !!str &quot;entry&quot;,
      : !!map {
        ? !!str &quot;key&quot; : !!str &quot;value&quot;
    } ],
    ? !!str &quot;mapping&quot;: : !!map {
      ? !!str &quot;key&quot; : !!str &quot;value&quot;
} } }</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id902431"></span>

**Example 8.10. Block Content**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>block styles:
  scalars:
    literal: !!str |
      #!/usr/bin/perl
      print &quot;Hello, world!\n&quot;;↓
    folded: &gt;
      This sentence
      is false.↓
  collections: !!seq
    sequence: !!seq # Entry:↓
      - entry # Plain
      # Mapping entry:↓
      - key: value↓
    mapping: ↓
      key: value↓</code></pre>
<pre class="synopsis"><code>Legend:
  c-l+block-scalar
  c-l-block-collection
  not content</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;block styles&quot; : !!map {
    ? !!str &quot;scalars&quot; : !!map {
      ? !!str &quot;literal&quot;
      : !!str &quot;#!!/usr/bin/perl\n\
          print \&quot;Hello,
          world!!\\n\&quot;;\n&quot;,
      ? !!str &quot;folded&quot;
      : !!str &quot;This sentence
          is false.\n&quot;
    },
    ? !!str &quot;collections&quot; : !!map {
      ? !!str &quot;sequence&quot; : !!seq [
        !!str &quot;entry&quot;,
        !!map {
          ? !!str &quot;key&quot; : !!str &quot;value&quot;
        }
      ],
      ? !!str &quot;mapping&quot; : !!map {
        ? !!str &quot;key&quot; : !!str &quot;value&quot;
} } } }</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id902561"></span>8.4. Alias Nodes

</div>

</div>

</div>

Subsequent occurrences of a previously <span id="id902569" class="indexterm"></span>[serialized](#serialize/) node are <span id="id902582" class="indexterm"></span>[presented](#present/) as <span id="id902594" class="indexterm"></span><span id="alias/syntax"></span>*alias nodes*, denoted by the <span id="id902611" class="indexterm"></span><span id="* alias/"></span>*“<span class="quote">**`*`**</span>” indicator*. The first occurrence of the <span id="id902630" class="indexterm"></span>[node](#node/syntax) must be marked by an <span id="id902646" class="indexterm"></span>[anchor](#anchor/syntax) to allow subsequent occurrences to be <span id="id902661" class="indexterm"></span>[presented](#present/) as alias nodes. An alias node refers to the most recent preceding <span id="id902675" class="indexterm"></span>[node](#node/syntax) having the same <span id="id902690" class="indexterm"></span>[anchor](#anchor/syntax). It is an error to have an alias node use an <span id="id902706" class="indexterm"></span>[anchor](#anchor/syntax) that does not previously occur in the <span id="id902722" class="indexterm"></span>[document](#document/syntax). It is not an error to specify an <span id="id902737" class="indexterm"></span>[anchor](#anchor/syntax) that is not used by any alias node. Note that an alias node must not specify any <span id="id902754" class="indexterm"></span>[properties](#node%20property/) or <span id="id902766" class="indexterm"></span>[content](#content/syntax), as these were already specified at the first occurrence of the <span id="id902782" class="indexterm"></span>[node](#node/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[120]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-ns-alias-node"></span>c-ns-alias-node</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-alias">“<span class="quote">*</span>”</a> <a href="#ns-anchor-name">ns-anchor-name</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id902824"></span>

**Example 8.11. Alias Nodes**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>First occurrence: &amp;anchor Value
Second occurrence: *anchor</code></pre>
<pre class="synopsis"><code>Legend:
  c-ns-alias-node
  ns-anchor-name</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;First occurrence&quot;
  : &amp;A !!str &quot;Value&quot;,
  ? !!str &quot;Second occurrence&quot;
  : *A
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id902919"></span>8.5. Complete Nodes

</div>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id902924"></span>8.5.1. Flow Nodes

</div>

</div>

</div>

A complete <span id="id902932" class="indexterm"></span><span id="flow style/syntax"></span>*flow node* is either an <span id="id902948" class="indexterm"></span>[alias node](#alias/syntax) <span id="id902962" class="indexterm"></span>[presenting](#present/) a second occurrence of a previous <span id="id902976" class="indexterm"></span>[node](#node/syntax), or consists of the <span id="id902991" class="indexterm"></span>[node properties](#node%20property/) followed by the <span id="id903004" class="indexterm"></span>[node’s content](#content/syntax). A <span id="id903020" class="indexterm"></span>[node](#node/syntax) with empty <span id="id903035" class="indexterm"></span>[content](#content/syntax) is considered to be an empty <span id="id903050" class="indexterm"></span>[plain scalar](#plain%20style/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[121]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-flow-node(n,c)"></span>ns-flow-node(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-ns-alias-node">c-ns-alias-node</a> | <a href="#ns-flow-content(n,c)">ns-flow-content(n,c)</a><br />
| ( <a href="#c-ns-properties(n,c)">c-ns-properties(n,c)</a><br />
    ( /* empty plain scalar content */<br />
    | ( <a href="#s-separate(n,c)">s-separate(n,c)</a> <a href="#ns-flow-content(n,c)">ns-flow-content(n,c)</a> ) ) )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id903117"></span>

**Example 8.12. Flow Nodes in Flow Context**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>[
  Without properties,
  &amp;anchor &quot;Anchored&quot;,
  !!str &#39;Tagged&#39;,
  *anchor, # Alias node
  !!str,   # Empty plain scalar
]</code></pre>
<pre class="synopsis"><code>Legend:
  ns-flow-node(n,c) ns-flow-content(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;Without properties&quot;,
  &amp;A !!str &quot;Anchored&quot;,
  !!str &quot;Tagged&quot;,
  *A,
  !!str &quot;&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

Since both the <span id="id903245" class="indexterm"></span>[node’s properties](#node%20property/) and <span id="id903257" class="indexterm"></span>[node content](#content/syntax) are optional, this allows for a <span id="id903272" class="indexterm"></span><span id="completely empty node/"></span>*completely empty node*. Completely empty nodes are only valid when following some explicit <span id="id903289" class="indexterm"></span>[indicator](#indicator/) for their existence.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[122]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="e-empty-flow"></span>e-empty-flow</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">/* empty plain scalar node */</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In the examples, completely empty nodes are displayed as the glyph “<span class="quote">**`°`**</span>”. Note that this glyph corresponds to a position in the characters <span id="id903331" class="indexterm"></span>[stream](#stream/syntax) rather than to an actual character.

<div class="example">

<span id="id903348"></span>

**Example 8.13. Completely Empty Flow Nodes**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>{
  ? foo :°,
  ?° : bar,
}</code></pre>
<pre class="synopsis"><code>Legend:
  e-empty-flow</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;foo&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;&quot;,
  : !!str &quot;bar&quot;,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id903421"></span>8.5.2. Block Nodes

</div>

</div>

</div>

A complete <span id="id903430" class="indexterm"></span><span id="block style/syntax"></span>*block node* consists of the <span id="id903448" class="indexterm"></span>[node’s properties](#node%20property/) followed by the <span id="id903462" class="indexterm"></span>[node’s content](#content/syntax). In addition, a block node may consist of a (possibly <span id="id903478" class="indexterm"></span>[completely empty](#completely%20empty%20node/)) <span id="id903492" class="indexterm"></span>[flow node](#flow%20style/syntax) followed by a <span id="id903508" class="indexterm"></span>[line break](#line%20break%20character/) (with optional <span id="id903524" class="indexterm"></span>[comments](#comment/syntax)).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[123]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l+flow-in-block(n,c)"></span>ns-l+flow-in-block(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-flow-node(n,c)">ns-flow-node(n+1,flow-out)</a> <a href="#s-l-comments">s-l-comments</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[124]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l+block-in-block(n,c)"></span>ns-l+block-in-block(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#c-ns-properties(n,c)">c-ns-properties(n+1,c)</a> <a href="#s-separate(n,c)">s-separate(n+1,c)</a> )?<br />
<a href="#c-l+block-content(n,c)">c-l+block-content(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[125]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l+block-node(n,c)"></span>ns-l+block-node(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#ns-l+block-in-block(n,c)">ns-l+block-in-block(n,c)</a><br />
| <a href="#ns-l+flow-in-block(n,c)">ns-l+flow-in-block(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[126]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l+block-node(n,c)"></span>s-l+block-node(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-separate(n,c)">s-separate(n+1,c)</a> <a href="#ns-l+block-node(n,c)">ns-l+block-node(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id903643"></span>

**Example 8.14. Block Nodes**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>-·&quot;flow in block&quot;↓
-·&gt;
 Block scalar↓
-·!!map # Block collection
  foo : bar↓</code></pre>
<pre class="synopsis"><code>Legend:
  ns-l+flow-in-block(n,c)
  ns-l+block-in-block(n,c)
  s-l+block-node(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;flow in block&quot;,
  !!str &quot;Block scalar\n&quot;,
  !!map {
    ? !!str &quot;foo&quot;
    : !!str &quot;bar&quot;
  }
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

A block node always spans to the end of the line, even when <span id="id903767" class="indexterm"></span>[completely empty](#completely%20empty%20node/). <span id="id903780" class="indexterm"></span>[Completely empty](#completely%20empty%20node/) block nodes may only appear when there is some explicit <span id="id903797" class="indexterm"></span>[indicator](#indicator/) for their existence.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[127]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-empty-block"></span>s-l-empty-block</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#e-empty-flow">e-empty-flow</a> <a href="#s-l-comments">s-l-comments</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id903833"></span>

**Example 8.15. Completely Empty Block Nodes**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>seq:
-° # Empty plain scalar↓
- ? foo
  :°↓
  ?°↓
  : bar,</code></pre>
<pre class="synopsis"><code>Legend:
  s-l-empty-block</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;&quot;,
  !!map {
    ? !!str &quot;foo&quot;
    : !!str &quot;&quot;,
    ? !!str &quot;&quot;,
    : !!str &quot;bar&quot;,
  }
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id903915"></span>Chapter 9. Scalar Styles

</div>

</div>

</div>

YAML provides a rich set of <span id="id903924" class="indexterm"></span><span id="scalar/syntax"></span>*scalar styles* to choose from, depending upon the readability requirements: three <span id="id903941" class="indexterm"></span>[scalar flow styles](#flow%20scalar%20style/syntax) (the <span id="id903957" class="indexterm"></span>[plain style](#plain%20style/syntax) and the two <span id="id903973" class="indexterm"></span><span id="quoted style/syntax"></span>*quoted styles*: <span id="id903990" class="indexterm"></span>[single-quoted](#single-quoted%20style/syntax) and <span id="id904006" class="indexterm"></span>[double-quoted](#double-quoted%20style/syntax)), and two <span id="id904022" class="indexterm"></span>[scalar block styles](#block%20scalar%20style/syntax) (the <span id="id904037" class="indexterm"></span>[literal style](#literal%20style/syntax) and the <span id="id904053" class="indexterm"></span>[folded style](#folded%20style/syntax)). <span id="id904069" class="indexterm"></span>[Comments](#comment/syntax) may precede or follow scalar content, but must not appear inside it. Scalar node style is a <span id="id904085" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not be used to convey <span id="id904099" class="indexterm"></span>[content information](#content/information%20model), with the exception that <span id="id904115" class="indexterm"></span>[untagged](#non-specific%20tag/) <span id="id904129" class="indexterm"></span>[plain scalars](#plain%20style/syntax) are <span id="id904146" class="indexterm"></span>[resolved](#tag%20resolution/) in a distinct way.

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id904158"></span>9.1. Flow Scalar Styles

</div>

</div>

</div>

All <span id="id904166" class="indexterm"></span><span id="flow scalar style/syntax"></span>*flow scalar styles* may span multiple lines, except when used in <span id="id904184" class="indexterm"></span>[simple keys](#simple%20key/). Flow scalars are subject to (flow) <span id="id904197" class="indexterm"></span>[line folding](#line%20folding/). This allows flow scalar content to be broken anywhere a single space character (**`#x20`**) separates non-space characters, at the cost of requiring an <span id="id904218" class="indexterm"></span>[empty line](#empty%20line/) to <span id="id904231" class="indexterm"></span>[present](#present/) each line feed character.

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id904245"></span>9.1.1. Double Quoted

</div>

</div>

</div>

The <span id="id904253" class="indexterm"></span><span id="double-quoted style/syntax"></span>*double-quoted style* is specified by surrounding <span id="id904271" class="indexterm"></span><span id="" double-quoted style/"></span>*“<span class="quote">**`"`**</span>” indicators*. This is the only <span id="id904294" class="indexterm"></span>[scalar style](#scalar/syntax) capable of expressing arbitrary strings, by using <span id="id904308" class="indexterm"></span>[“<span class="quote">**`\`**</span>”](#\%20escaping%20in%20double-quoted%20style/) <span id="id904327" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/). Therefore, the <span id="id904342" class="indexterm"></span>[“<span class="quote">**`\`**</span>”](#\%20escaping%20in%20double-quoted%20style/) and “<span class="quote">**`"`**</span>” characters must also be <span id="id904367" class="indexterm"></span>[escaped](#\%20escaping%20in%20double-quoted%20style/) when present in double-quoted content. Note it is an error for double-quoted content to contain invalid <span id="id904384" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[128]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-double-char"></span>nb-double-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#nb-char">nb-char</a> - <a href="#c-escape">“<span class="quote">\</span>”</a> - <a href="#c-double-quote">“<span class="quote">"</span>”</a> ) | <a href="#ns-esc-char">ns-esc-char</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[129]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-double-char"></span>ns-double-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-double-char">nb-double-char</a> - <a href="#s-white">s-white</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Double-quoted scalars are restricted to a single line when contained inside a <span id="id904461" class="indexterm"></span>[simple key](#simple%20key/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[130]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-double-quoted(n,c)"></span>c-double-quoted(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-double-quote">“<span class="quote">"</span>”</a> <a href="#nb-double-text(n,c)">nb-double-text(n,c)</a> <a href="#c-double-quote">“<span class="quote">"</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[131]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-double-text(n,c)"></span>nb-double-text(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = flow-out ⇒ <a href="#nb-double-any(n)">nb-double-any(n)</a><br />
<code class="varname">c</code> = flow-in  ⇒ <a href="#nb-double-any(n)">nb-double-any(n)</a><br />
<code class="varname">c</code> = flow-key ⇒ <a href="#nb-double-single">nb-double-single</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[132]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-double-any(n)"></span>nb-double-any(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-double-single">nb-double-single</a> | <a href="#nb-double-multi(n)">nb-double-multi(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id904573"></span>

**Example 9.1. Double Quoted Scalars**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&quot;simple key&quot; : {
  &quot;also simple&quot; : value,
  ? &quot;not a
  simple key&quot; : &quot;any
  value&quot;
}</code></pre>
<pre class="synopsis"><code>Legend:
  nb-double-single nb-double-multi(n)
  c-double-quoted(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;simple key&quot;
  : !!map {
    ? !!str &quot;also simple&quot;
    : !!str &quot;value&quot;,
    ? !!str &quot;not a simple key&quot;
    : !!str &quot;any value&quot;
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

A single line double-quoted scalar is a sequence of (possibly <span id="id904712" class="indexterm"></span>[escaped](#escaping%20in%20double-quoted%20style/)) non-<span id="id904728" class="indexterm"></span>[break](#line%20break%20character/) Unicode characters. All characters are considered <span id="id904742" class="indexterm"></span>[content](#content/syntax), including any leading or trailing <span id="id904757" class="indexterm"></span>[white space](#white%20space/) characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[133]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-double-single"></span>nb-double-single</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-double-char">nb-double-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In a multi-line double-quoted scalar, <span id="id904794" class="indexterm"></span>[line breaks](#line%20break%20character/) are subject to flow line <span id="id904810" class="indexterm"></span>[folding](#line%20folding/), and any trailing <span id="id904821" class="indexterm"></span>[white space](#white%20space/) is excluded from the <span id="id904834" class="indexterm"></span>[content](#content/syntax). However, an <span id="id904849" class="indexterm"></span><span id="escaped (ignored) line break/"></span>*escaped line break* (using a <span id="id904864" class="indexterm"></span>[“<span class="quote">**`\`**</span>”](#\%20escaping%20in%20double-quoted%20style/)) is excluded from the <span id="id904885" class="indexterm"></span>[content](#content/syntax), while <span id="id904898" class="indexterm"></span>[white space](#white%20space/) preceding it is preserved. This allows double-quoted content to be broken at arbitrary positions.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[134]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-double-folded(n)"></span>s-l-double-folded(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-white">s-ignored-white</a>* <a href="#b-l-folded-any(n,s)">b-l-folded-any(n,double)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[135]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-double-escaped(n)"></span>s-l-double-escaped(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-white">s-white</a>* <a href="#c-escape">“<span class="quote">\</span>”</a> <a href="#b-ignored-any">b-ignored-any</a><br />
<a href="#l-empty(n,s)">l-empty(n,double)</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[136]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-double-break(n)"></span>s-l-double-break(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-l-double-folded(n)">s-l-double-folded(n)</a> | <a href="#s-l-double-escaped(n)">s-l-double-escaped(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id905000"></span>

**Example 9.2. Double Quoted Line Breaks**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> &quot;as space→↓
 trimmed·↓
↓
 specific⇓
↓
 escaped→\¶
·↓
 none&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;as space \
  trimmed\n\
  specific\L\n\
  escaped\t\
  none&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  s-l-double-folded(n) s-l-double-escaped(n)
  s-ignored-white      s-white (Content)</code></pre></td>
</tr>
</tbody>
</table>

</div>

A multi-line double-quoted scalar consists of a (possibly empty) first line, any number of inner lines, and a final (possibly empty) last line.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[137]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-double-multi(n)"></span>nb-double-multi(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-l-double-first(n)">nb-l-double-first(n)</a><br />
<a href="#l-double-inner(n)">l-double-inner(n)</a>*<br />
<a href="#s-nb-double-last(n)">s-nb-double-last(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Leading <span id="id905171" class="indexterm"></span>[white space](#white%20space/) in the first line is considered <span id="id905184" class="indexterm"></span>[content](#content/syntax) only if followed by a non-space character or an escaped (ignored) line break.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[138]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-l-double-first(n)"></span>nb-l-double-first(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#nb-double-char">nb-double-char</a>* <a href="#ns-double-char">ns-double-char</a> )?<br />
<a href="#s-l-double-break(n)">s-l-double-break(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id905233"></span>

**Example 9.3. First Double Quoted Line**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- &quot;↓
  last&quot;
- &quot;·→↓
  last&quot;
- &quot;·→first↓
  last&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  nb-l-double-first(n) s-ignored-white</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot; last&quot;,
  !!str &quot; last&quot;,
  !!str &quot; \tfirst last&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

All leading and trailing <span id="id905329" class="indexterm"></span>[white space](#white%20space/) of an inner lines are excluded from the <span id="id905343" class="indexterm"></span>[content](#content/syntax). Note that while <span id="id905358" class="indexterm"></span>[prefix white space](#ignored%20line%20prefix/) may contain <span id="id905373" class="indexterm"></span>[tab](#tab/) characters, line <span id="id905385" class="indexterm"></span>[indentation](#indentation%20space/) is restricted to space characters only. It is possible to force considering leading <span id="id905400" class="indexterm"></span>[white space](#white%20space/) as <span id="id905412" class="indexterm"></span>[content](#content/syntax) by <span id="id905427" class="indexterm"></span>[escaping](#escaping%20in%20double-quoted%20style/) the first character (<span id="id905442" class="indexterm"></span>[“<span class="quote">**`\·`**</span>”](#\%20escaping%20in%20double-quoted%20style/), <span id="id905462" class="indexterm"></span>[“<span class="quote">**`\→`**</span>”](#\%20escaping%20in%20double-quoted%20style/) or <span id="id905480" class="indexterm"></span>[“<span class="quote">**`\t`**</span>”](#\%20escaping%20in%20double-quoted%20style/)).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[139]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-double-inner(n)"></span>l-double-inner(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-prefix(n,s)">s-ignored-prefix(n,double)</a> <a href="#ns-double-char">ns-double-char</a><br />
( <a href="#nb-double-char">nb-double-char</a>* <a href="#ns-double-char">ns-double-char</a> )?<br />
<a href="#s-l-double-break(n)">s-l-double-break(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id905543"></span>

**Example 9.4. Inner Double Quoted Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> &quot;first
·→inner 1→↓
·\·inner 2·\↓
 last&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-double-inner(n)
  s-ignored-prefix(n,s) s-l-double-break(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;first·\
  inner 1··\
  inner 2·\
  last&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

The leading <span id="id905662" class="indexterm"></span>[prefix](#ignored%20line%20prefix/) <span id="id905675" class="indexterm"></span>[white space](#white%20space/) of the last line is stripped in the same way as for inner lines. Trailing <span id="id905689" class="indexterm"></span>[white space](#white%20space/) is considered <span id="id905701" class="indexterm"></span>[content](#content/syntax) only if preceded by a non-space character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[140]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-nb-double-last(n)"></span>s-nb-double-last(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-prefix(n,s)">s-ignored-prefix(n,double)</a><br />
( <a href="#ns-double-char">ns-double-char</a> <a href="#nb-double-char">nb-double-char</a>* )?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id905751"></span>

**Example 9.5. Last Double Quoted Line**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- &quot;first
··→&quot;
- &quot;first
&#10;··→last&quot;
- &quot;first
 inner
·\·→last&quot;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;first &quot;,
  !!str &quot;first\nlast&quot;,
  !!str &quot;first inner··\tlast&quot;,
]</code></pre>
<pre class="synopsis"><code>Legend:
  s-nb-double-last(n)
  s-ignored-prefix(n,s)</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id905860"></span>9.1.2. Single Quoted

</div>

</div>

</div>

The <span id="id905868" class="indexterm"></span><span id="single-quoted style/syntax"></span>*single-quoted style* is specified by surrounding <span id="id905887" class="indexterm"></span><span id="' single-quoted style/"></span>*“<span class="quote">**`'`**</span>” indicators*. Therefore, within a single-quoted scalar such characters need to be repeated. This is the only form of <span id="id905912" class="indexterm"></span><span id="escaping in single-quoted style/"></span>*escaping* performed in single-quoted scalars. In particular, the <span id="id905929" class="indexterm"></span>[“<span class="quote">**`\`**</span>”](#\%20escaping%20in%20double-quoted%20style/) and <span id="id905946" class="indexterm"></span>[“<span class="quote">**`"`**</span>”](#%22%20double-quoted%20style/) characters may be freely used. This restricts single-quoted scalars to <span id="id905966" class="indexterm"></span>[printable](#printable%20character/) characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[141]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-quoted-quote"></span>c-quoted-quote</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-single-quote">“<span class="quote">'</span>”</a> <a href="#c-single-quote">“<span class="quote">'</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[142]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-single-char"></span>nb-single-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#nb-char">nb-char</a> - <a href="#c-single-quote">“<span class="quote">"</span>”</a> ) | <a href="#c-quoted-quote">c-quoted-quote</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[143]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-single-char"></span>ns-single-char</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-single-char">nb-single-char</a> - <a href="#s-white">s-white</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id906056"></span>

**Example 9.6. Single Quoted Quotes**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> &#39;here&#39;&#39;s to &quot;quotes&quot;&#39;</code></pre>
<pre class="synopsis"><code>Legend:
  single-quoted-quote</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;here&#39;s to \&quot;quotes\&quot;&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

Single-quoted scalars are restricted to a single line when contained inside a <span id="id906126" class="indexterm"></span>[simple key](#simple%20key/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[144]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-single-quoted(n,c)"></span>c-single-quoted(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-single-quote">“<span class="quote">'</span>”</a> <a href="#nb-single-text(n,c)">nb-single-text(n,c)</a> <a href="#c-single-quote">“<span class="quote">'</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[145]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-single-text(n,c)"></span>nb-single-text(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = flow-out ⇒ <a href="#nb-single-any(n)">nb-single-any(n)</a><br />
<code class="varname">c</code> = flow-in  ⇒ <a href="#nb-single-any(n)">nb-single-any(n)</a><br />
<code class="varname">c</code> = flow-key ⇒ <a href="#nb-single-single">nb-single-single(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[146]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-single-any(n)"></span>nb-single-any(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-single-single">nb-single-single(n)</a> | <a href="#nb-single-multi(n)">nb-single-multi(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id906238"></span>

**Example 9.7. Single Quoted Scalars**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&#39;simple key&#39; : {
  &#39;also simple&#39; : value,
  ? &#39;not a
  simple key&#39; : &#39;any
  value&#39;
}</code></pre>
<pre class="synopsis"><code>Legend:
  nb-single-single nb-single-multi(n)
  c-single-quoted(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;simple key&quot;
  : !!map {
    ? !!str &quot;also simple&quot;
    : !!str &quot;value&quot;,
    ? !!str &quot;not a simple key&quot;
    : !!str &quot;any value&quot;
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

A single line single-quoted scalar is a sequence of non-<span id="id906376" class="indexterm"></span>[break](#line%20break%20character/) <span id="id906390" class="indexterm"></span>[printable](#printable%20character/) characters. All characters are considered <span id="id906404" class="indexterm"></span>[content](#content/syntax), including any leading or trailing <span id="id906419" class="indexterm"></span>[white space](#white%20space/) characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[147]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-single-single"></span>nb-single-single(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-single-char">nb-single-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In a multi-line single-quoted scalar, <span id="id906457" class="indexterm"></span>[line breaks](#line%20break%20character/) are subject to (flow) <span id="id906473" class="indexterm"></span>[line folding](#line%20folding/), and any trailing <span id="id906484" class="indexterm"></span>[white space](#white%20space/) is excluded from the <span id="id906497" class="indexterm"></span>[content](#content/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[148]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-single-break(n)"></span>s-l-single-break(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-white">s-ignored-white</a>* <a href="#b-l-folded-any(n,s)">b-l-folded-any(n,single)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id906540"></span>

**Example 9.8. Single Quoted Line Breaks**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> &#39;as space→↓
 trimmed·↓
↓
 specific⇓
↓
 none&#39;</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;as space \
  trimmed\n\
  specific\L\n\
  none&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  s-l-single-break(n)
  s-ignored-white s-white (Content)</code></pre></td>
</tr>
</tbody>
</table>

</div>

A multi-line single-quoted scalar consists of a (possibly empty) first line, any number of inner lines, and a final (possibly empty) last line.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[149]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-single-multi(n)"></span>nb-single-multi(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-l-single-first(n)">nb-l-single-first(n)</a><br />
<a href="#l-single-inner(n)">l-single-inner(n)</a>*<br />
<a href="#s-nb-single-last(n)">s-nb-single-last(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Leading <span id="id906689" class="indexterm"></span>[white space](#white%20space/) in the first line is considered <span id="id906701" class="indexterm"></span>[content](#content/syntax) only if followed by a non-space character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[150]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-l-single-first(n)"></span>nb-l-single-first(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#nb-single-char">nb-single-char</a>* <a href="#ns-single-char">ns-single-char</a> )?<br />
<a href="#s-l-single-break(n)">s-l-single-break(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id906750"></span>

**Example 9.9. First Single Quoted Line**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- &#39;↓
  last&#39;
- &#39;·→↓
  last&#39;
- &#39;·→first↓
  last&#39;</code></pre>
<pre class="synopsis"><code>Legend:
  nb-l-single-first(n) s-ignored-white</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot; last&quot;,
  !!str &quot; last&quot;,
  !!str &quot; \tfirst last&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

All leading and trailing <span id="id906846" class="indexterm"></span>[white space](#white%20space/) of inner lines is excluded from the <span id="id906860" class="indexterm"></span>[content](#content/syntax). Note that while <span id="id906875" class="indexterm"></span>[prefix white space](#ignored%20line%20prefix/) may contain <span id="id906889" class="indexterm"></span>[tab](#tab/) characters, line <span id="id906901" class="indexterm"></span>[indentation](#indentation%20space/) is restricted to space characters only. Unlike <span id="id906917" class="indexterm"></span>[double-quoted scalars](#double-quoted%20style/syntax), it is impossible to force the inclusion of the leading or trailing spaces in the <span id="id906933" class="indexterm"></span>[content](#content/syntax). Therefore, single-quoted scalars lines can only be broken where a single space character separates two non-space characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[151]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-single-inner(n)"></span>l-single-inner(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-prefix(n,s)">s-ignored-prefix(n,single)</a> <a href="#ns-single-char">ns-single-char</a><br />
( <a href="#nb-single-char">nb-single-char</a>* <a href="#ns-single-char">ns-single-char</a> )?<br />
<a href="#s-l-single-break(n)">s-l-single-break(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id906995"></span>

**Example 9.10. Inner Single Quoted Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> &#39;first
·→inner→↓
 last&#39;</code></pre>
<pre class="synopsis"><code>Legend:
  l-single-inner(n)
  s-ignored-prefix(n,s) s-l-single-break(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;first \
  inner \
  last&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

The leading <span id="id907096" class="indexterm"></span>[prefix](#ignored%20line%20prefix/) <span id="id907109" class="indexterm"></span>[white space](#white%20space/) of the last line is stripped in the same way as for inner lines. Trailing <span id="id907123" class="indexterm"></span>[white space](#white%20space/) is considered <span id="id907136" class="indexterm"></span>[content](#content/syntax) only if preceded by a non-space character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[152]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-nb-single-last(n)"></span>s-nb-single-last(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-prefix(n,s)">s-ignored-prefix(n,single)</a><br />
( <a href="#ns-single-char">ns-single-char</a> <a href="#nb-single-char">nb-single-char</a>* )?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id907185"></span>

**Example 9.11. Last Single Quoted Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- &#39;first
··→&#39;
- &#39;first
&#10;··→last&#39;</code></pre>
<pre class="synopsis"><code>Legend:
  s-nb-double-last(n) s-ignored-prefix(n,s)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;first &quot;,
  !!str &quot;first\nlast&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id907281"></span>9.1.3. Plain

</div>

</div>

</div>

The <span id="id907289" class="indexterm"></span><span id="plain style/syntax"></span>*plain style* uses no identifying <span id="id907307" class="indexterm"></span>[indicators](#indicator/), and is therefore the most limited and most <span id="id907320" class="indexterm"></span>[context](#context/) sensitive <span id="id907334" class="indexterm"></span>[scalar style](#scalar/syntax). Plain scalars can never contain any <span id="id907350" class="indexterm"></span>[tab](#tab/) characters. They also must not contain the <span id="id907363" class="indexterm"></span>[“<span class="quote">**`: `**</span>”](#:%20mapping%20value/) and <span id="id907382" class="indexterm"></span>[“<span class="quote">**` #`**</span>”](##%20comment/) character sequences as these combinations cause ambiguity with <span id="id907400" class="indexterm"></span>[key:](#key/syntax) <span id="id907413" class="indexterm"></span>[value](#value/syntax) pairs and <span id="id907428" class="indexterm"></span>[comments](#comment/syntax). Inside <span id="id907443" class="indexterm"></span>[flow collections](#flow%20collection%20style/syntax), plain scalars are further restricted to avoid containing the <span id="id907460" class="indexterm"></span>[“<span class="quote">**`[`**</span>”](#%5B%20start%20flow%20sequence/), <span id="id907480" class="indexterm"></span>[“<span class="quote">**`]`**</span>”](#%5D%20end%20flow%20sequence/), <span id="id907497" class="indexterm"></span>[“<span class="quote">**`{`**</span>”](#%7B%20start%20flow%20mapping/), <span id="id907514" class="indexterm"></span>[“<span class="quote">**`}`**</span>”](#%7D%20end%20flow%20mapping/) and <span id="id907529" class="indexterm"></span>[“<span class="quote">**`,`**</span>”](#,%20end%20flow%20entry/) characters as these would cause ambiguity with the <span id="id907548" class="indexterm"></span>[flow collection](#flow%20collection%20style/syntax) structure (hence the need for the <span id="id907564" class="indexterm"></span><span id="flow-in context/"></span>*flow-in context* and the <span id="id907580" class="indexterm"></span><span id="flow-out context/"></span>*flow-out context*).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[153]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-plain-char(c)"></span>nb-plain-char(c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = flow-out ⇒ <a href="#nb-plain-char-out">nb-plain-char-out</a><br />
<code class="varname">c</code> = flow-in  ⇒ <a href="#nb-plain-char-in">nb-plain-char-in</a><br />
<code class="varname">c</code> = flow-key ⇒ <a href="#nb-plain-char-in">nb-plain-char-in</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[154]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-plain-char-out"></span>nb-plain-char-out</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#nb-char">nb-char</a> - <a href="#c-mapping-value">“<span class="quote">:</span>”</a> - <a href="#c-comment">“<span class="quote">#</span>”</a> - #x9 /*TAB*/ )<br />
| ( <a href="#ns-plain-char(c)">ns-plain-char(flow-out)</a> <a href="#c-comment">“<span class="quote">#</span>”</a> )<br />
| ( <a href="#c-mapping-value">“<span class="quote">:</span>”</a> <a href="#ns-plain-char(c)">ns-plain-char(flow-out)</a> )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[155]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="nb-plain-char-in"></span>nb-plain-char-in</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-plain-char-out">nb-plain-char-out</a> - <a href="#c-collect-entry">“<span class="quote">,</span>”</a> - <a href="#c-sequence-start">“<span class="quote">[</span>”</a> - <a href="#c-sequence-end">“<span class="quote">]</span>”</a> - <a href="#c-mapping-start">“<span class="quote">{</span>”</a> - <a href="#c-mapping-end">“<span class="quote">}</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[156]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-plain-char(c)"></span>ns-plain-char(c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#nb-plain-char(c)">nb-plain-char(c)</a> - #x20 /*SP*/</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

The first plain character is further restricted to avoid most <span id="id907776" class="indexterm"></span>[indicators](#indicator/) as these would cause ambiguity with various YAML structures. However, the first character may be <span id="id907790" class="indexterm"></span>[“<span class="quote">**`-`**</span>”](#-%20block%20sequence%20entry/), <span id="id907810" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20mapping%20key/) or <span id="id907826" class="indexterm"></span>[“<span class="quote">**`:`**</span>”](#:%20mapping%20value/) provided it is followed by a non-space character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[157]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-plain-first-char(c)"></span>ns-plain-first-char(c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#ns-plain-char(c)">ns-plain-char(c)</a> - <a href="#c-indicator">c-indicator</a> )<br />
| ( ( <a href="#c-sequence-entry">“<span class="quote">-</span>”</a> | <a href="#c-mapping-key">“<span class="quote">?</span>”</a> | <a href="#c-mapping-value">“<span class="quote">:</span>”</a> ) <a href="#ns-plain-char(c)">ns-plain-char(c)</a> )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id907899"></span>

**Example 9.12. Plain Characters**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code># Outside flow collection:
- ::std::vector
- Up, up and away!
- -123
# Inside flow collection:
- [ ::std::vector,
  &quot;Up, up and away!&quot;,
  -123 ]</code></pre>
<pre class="synopsis"><code>Legend:
  ns-plain-first-char(c)
  ns-plain-char(c) Not ns-plain-char(c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;::std::vector&quot;,
  !!str &quot;Up, up, and away!&quot;,
  !!int &quot;-123&quot;,
  !!seq [
    !!str &quot;::std::vector&quot;,
    !!str &quot;Up, up, and away!&quot;,
    !!int &quot;-123&quot;,
  ]
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

Plain scalars are restricted to a single line when contained inside a <span id="id908025" class="indexterm"></span>[simple key](#simple%20key/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[158]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-plain(n,c)"></span>ns-plain(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = flow-out ⇒ <a href="#ns-plain-multi(n,c)">ns-plain-multi(n,c)</a>?<br />
<code class="varname">c</code> = flow-in  ⇒ <a href="#ns-plain-multi(n,c)">ns-plain-multi(n,c)</a>?<br />
<code class="varname">c</code> = flow-key ⇒ <a href="#ns-plain-single(c)">ns-plain-single(c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id908084"></span>

**Example 9.13. Plain Scalars**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>simple key : {
  also simple : value,
  ? not a
  simple key : any
  value
}</code></pre>
<pre class="synopsis"><code>Legend:
  ns-plain-single(c) ns-plain-multi(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;simple key&quot;
  : !!map {
    ? !!str &quot;also simple&quot;
    : !!str &quot;value&quot;,
    ? !!str &quot;not a simple key&quot;
    : !!str &quot;any value&quot;
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

The first line of any <span id="id908184" class="indexterm"></span>[flow scalar](#flow%20scalar%20style/syntax) is <span id="id908200" class="indexterm"></span>[indented](#indentation%20space/) according to the <span id="id908213" class="indexterm"></span>[collection](#collection/syntax) it is contained in. Therefore, there are two cases where a plain scalar begins on the first column of a line, without any preceding <span id="id908232" class="indexterm"></span>[indentation](#indentation%20space/) spaces: a plain scalar used as a <span id="id908245" class="indexterm"></span>[simple key](#simple%20key/) of a non-indented <span id="id908258" class="indexterm"></span>[block mapping](#block%20mapping%20style/syntax), and any plain scalar nested in a non-indented <span id="id908276" class="indexterm"></span>[flow collection](#flow%20collection%20style/syntax). In these cases, the first line of the plain scalar must not conflict with a <span id="id908293" class="indexterm"></span>[document boundary marker](#document%20boundary%20marker/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[159]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-forbidden-content"></span>l-forbidden-content</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">/* start of line */<br />
( <a href="#c-document-start">c-document-start</a> | <a href="#c-document-end">c-document-end</a> )<br />
/* space or end of line */</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id908340"></span>

**Example 9.14. Forbidden Non-Indented Plain Scalar Content**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>---
---·||| : foo
...·&gt;&gt;&gt;: bar
---
[
---↓
,
...·,
{
---·:
...·# Nested
}
]
...</code></pre></td>
<td><pre class="screen"><code>ERROR:
 The --- and ... document
 start and end markers must
 not be specified as the
 first content line of a
 non-indented plain scalar.</code></pre></td>
</tr>
</tbody>
</table>

</div>

YAML provides several easy ways to <span id="id908434" class="indexterm"></span>[present](#present/) such <span id="id908446" class="indexterm"></span>[content](#content/syntax) without conflicting with the <span id="id908462" class="indexterm"></span>[document boundary markers](#document%20boundary%20marker/). For example:

<div class="example">

<span id="id908477"></span>

**Example 9.15. Document Marker Scalar Content**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>---
&quot;---&quot; : foo
...: bar
---
[
---,
...,
{
? ---
: ...
}
]
...</code></pre>
<pre class="synopsis"><code>Legend:
  Content --- and ...
  Document marker --- and ...</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;---&quot;
  : !!str &quot;foo&quot;,
  ? !!str &quot;...&quot;,
  : !!str &quot;bar&quot;
}
%YAML 1.1
---
!!seq [
  !!str &quot;---&quot;,
  !!str &quot;...&quot;,
  !!map {
    ? !!str &quot;---&quot;
    : !!str &quot;...&quot;
  }
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

Thus, a single line plain scalar is a sequence of valid plain non-<span id="id908596" class="indexterm"></span>[break](#line%20break%20character/) <span id="id908610" class="indexterm"></span>[printable](#printable%20character/) characters, beginning and ending with non-space character and not conflicting with a <span id="id908625" class="indexterm"></span>[document boundary markers](#document%20boundary%20marker/). All characters are considered <span id="id908640" class="indexterm"></span>[content](#content/syntax), including any inner space characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[160]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-plain-single(c)"></span>ns-plain-single(c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#ns-plain-first-char(c)">ns-plain-first-char(c)</a><br />
    ( <a href="#nb-plain-char(c)">nb-plain-char(c)</a>* <a href="#ns-plain-char(c)">ns-plain-char(c)</a> )? )<br />
- <a href="#l-forbidden-content">l-forbidden-content</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

In a multi-line plain scalar, <span id="id908698" class="indexterm"></span>[line breaks](#line%20break%20character/) are subject to (flow) <span id="id908714" class="indexterm"></span>[line folding](#line%20folding/). Any <span id="id908725" class="indexterm"></span>[prefix](#ignored%20line%20prefix/) and trailing spaces are excluded from the <span id="id908739" class="indexterm"></span>[content](#content/syntax). Like <span id="id908754" class="indexterm"></span>[single-quoted scalars](#single-quoted%20style/syntax), in plain scalars it is impossible to force the inclusion of the leading or trailing spaces in the <span id="id926249" class="indexterm"></span>[content](#content/syntax). Therefore, plain scalars lines can only be broken where a single space character separates two non-space characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[161]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l-plain-break(n)"></span>s-l-plain-break(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-ignored-white">s-ignored-white</a>* <a href="#b-l-folded-any(n,s)">b-l-folded-any(n,plain)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id926293"></span>

**Example 9.16. Plain Line Breaks**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> as space→↓
 trimmed·↓
↓
 specific⇓
↓
 none</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;as space \
  trimmed\n\
  specific\L\n\
  none&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  s-l-plain-break(n)
  s-ignored-white</code></pre></td>
</tr>
</tbody>
</table>

</div>

A multi-line plain scalar contains additional continuation lines following the first line.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[162]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-plain-multi(n,c)"></span>ns-plain-multi(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-plain-single(c)">ns-plain-single(c)</a> <a href="#s-ns-plain-more(n,c)">s-ns-plain-more(n,c)</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Each continuation line must contain at least one non-space character. Note that it may be preceded by any number of <span id="id926427" class="indexterm"></span>[empty lines](#empty%20line/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[163]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-ns-plain-more(n,c)"></span>s-ns-plain-more(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-l-plain-break(n)">s-l-plain-break(n)</a><br />
<a href="#s-ignored-prefix(n,s)">s-ignored-prefix(n,plain)</a> <a href="#ns-plain-char(c)">ns-plain-char(c)</a><br />
( <a href="#nb-plain-char(c)">nb-plain-char(c)</a>* <a href="#ns-plain-char(c)">ns-plain-char(c)</a> )?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id926484"></span>

**Example 9.17. Plain Scalars**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> first line·↓
···↓
··more line</code></pre>
<pre class="synopsis"><code>Legend:
  ns-plain-single(c) s-l-plain-break(n)
  s-ignored-prefix(n,s) s-ns-plain-more(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;first line\n\
      more line&quot;</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id926597"></span>9.2. Block Scalar Header

</div>

</div>

</div>

<span id="id926605" class="indexterm"></span>[Block scalars](#block%20scalar%20style/syntax) are specified by several <span id="id926621" class="indexterm"></span>[indicators](#indicator/) given in a <span id="id926633" class="indexterm"></span><span id="block scalar header/"></span>*header* preceding the <span id="id926649" class="indexterm"></span>[content](#content/syntax) itself. The header is followed by an ignored <span id="id926664" class="indexterm"></span>[line break](#line%20break%20character/) (with an optional <span id="id926680" class="indexterm"></span>[comment](#comment/syntax)).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[164]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-b-block-header(s,m,t)"></span>c-b-block-header(s,m,t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-style-indicator(s)">c-style-indicator(s)</a><br />
( ( <a href="#c-indentation-indicator(m)">c-indentation-indicator(m)</a><br />
    <a href="#c-chomping-indicator(t)">c-chomping-indicator(t)</a> )<br />
| ( <a href="#c-chomping-indicator(t)">c-chomping-indicator(t)</a><br />
    <a href="#c-indentation-indicator(m)">c-indentation-indicator(m)</a> ) )<br />
<a href="#s-b-comment">s-b-comment</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id926746"></span>

**Example 9.18. Block Scalar Header**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- | # Just the style↓
 literal
- &gt;1 # Indentation indicator↓
 ·folded
- |+ # Chomping indicator↓
 keep
&#10;- &gt;-1 # Both indicators↓
 ·strip
</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;literal\n&quot;,
  !!str &quot;·folded\n&quot;,
  !!str &quot;keep\n\n&quot;,
  !!str &quot;·strip&quot;,
]</code></pre>
<pre class="synopsis"><code>Legend:
  c-b-block-header(s,m,t)</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id926836"></span>9.2.1. Block Style Indicator

</div>

</div>

</div>

The first character of the <span id="id926845" class="indexterm"></span>[block scalar header](#block%20scalar%20header/) is either <span id="id926861" class="indexterm"></span><span id="| literal style/"></span>*“<span class="quote">**`|`**</span>”* for a <span id="id926880" class="indexterm"></span>[literal scalar](#literal%20style/syntax) or <span id="id926895" class="indexterm"></span><span id="> folded style/"></span>*“<span class="quote">**`>`**</span>”* for a <span id="id926915" class="indexterm"></span>[folded scalar](#folded%20style/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[165]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-style-indicator(s)"></span>c-style-indicator(s)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">s = literal ⇒ <a href="#c-literal">“<span class="quote">|</span>”</a><br />
s = folded  ⇒ <a href="#c-folded">“<span class="quote">&gt;</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id926963"></span>

**Example 9.19. Block Style Indicator**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- |
 literal
- &gt;
 folded</code></pre>
<pre class="synopsis"><code>Legend:
  c-style-indicator(s)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;literal\n&quot;,
  !!str &quot;folded\n&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id927035"></span>9.2.2. Block Indentation Indicator

</div>

</div>

</div>

Typically, the <span id="id927043" class="indexterm"></span>[indentation](#indentation%20space/) level of a <span id="id927058" class="indexterm"></span>[block scalar](#block%20scalar%20style/syntax) is detected from its first non-<span id="id927075" class="indexterm"></span>[empty](#empty%20line/) line. This detection fails when this line contains leading space characters (note it may safely start with a <span id="id927088" class="indexterm"></span>[tab](#tab/) or a <span id="id927100" class="indexterm"></span>[“<span class="quote">**`#`**</span>”](##%20comment/) character). When detection fails, YAML requires that the <span id="id927119" class="indexterm"></span>[indentation](#indentation%20space/) level for the <span id="id927133" class="indexterm"></span>[content](#content/syntax) be given using an explicit <span id="id927146" class="indexterm"></span><span id="indentation indicator/"></span>*indentation indicator*. This level is specified as the integer number of the additional <span id="id927164" class="indexterm"></span>[indentation](#indentation%20space/) spaces used for the <span id="id927177" class="indexterm"></span>[content](#content/syntax). If the <span id="id927191" class="indexterm"></span>[block scalar](#block%20scalar%20style/syntax) begins with leading <span id="id927208" class="indexterm"></span>[empty lines](#empty%20line/) followed by a non-<span id="id927222" class="indexterm"></span>[empty line](#empty%20line/), the <span id="id927234" class="indexterm"></span>[indentation](#indentation%20space/) level is deduced from the non-<span id="id927248" class="indexterm"></span>[empty line](#empty%20line/). In this case, it is an error for any such leading <span id="id927261" class="indexterm"></span>[empty line](#empty%20line/) to contain more spaces than the <span id="id927274" class="indexterm"></span>[indentation](#indentation%20space/) level deduced from the non-<span id="id927288" class="indexterm"></span>[empty](#empty%20line/) line. It is always valid to specify an indentation indicator for a <span id="id927304" class="indexterm"></span>[block scalar](#block%20scalar%20style/syntax) node, though a YAML <span id="id927319" class="indexterm"></span>[processor](#processor/) should only do so in cases where detection will fail.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[166]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-indentation-indicator(m)"></span>c-indentation-indicator(m)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">explicit(m) ⇒ <a href="#ns-dec-digit">ns-dec-digit</a> - “<span class="quote">0</span>”<br />
detect(m)   ⇒ /* empty */</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id927362"></span>

**Example 9.20. Block Indentation Indicator**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- |
·detected
- &gt;
·
··
··# detected
- |1
··explicit
- &gt;
·→
·detected</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;detected\n&quot;,
  !!str &quot;\n\n# detected\n&quot;,
  !!str &quot;·explicit\n&quot;,
  !!str &quot;\t·detected\n&quot;,
]</code></pre>
<pre class="synopsis"><code>Legend:
  c-indentation-indicator(m)
  s-indent(n)</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id927476"></span>

**Example 9.21. Invalid Block Scalar Indentation Indicators**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>- |
··
·text
- &gt;
··text
·text
- |1
·text</code></pre></td>
<td><pre class="screen"><code>ERROR:
- A leading all-space line must
  not have too many spaces.
- A following text line must
  not be less indented.
- The text is less indented
  than the indicated level.</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id927557"></span>9.2.3. Block Chomping Indicator

</div>

</div>

</div>

YAML supports three possible block <span id="id927566" class="indexterm"></span><span id="chomping/"></span>*chomping* methods:

<div class="variablelist">

<span class="term">Strip</span>  
<span id="id927591" class="indexterm"></span><span id="strip chomping/"></span>*Stripping* is specified using the <span id="id927605" class="indexterm"></span><span id="- strip chomping/"></span>*“<span class="quote">**`-`**</span>” chomping indicator*. In this case, the <span id="id927628" class="indexterm"></span>[line break](#line%20break%20character/) character of the last non-<span id="id927640" class="indexterm"></span>[empty line](#empty%20line/) (if any) is excluded from the <span id="id927653" class="indexterm"></span>[scalar’s content](#scalar/syntax). Any trailing <span id="id927669" class="indexterm"></span>[empty lines](#empty%20line/) are considered to be (empty) <span id="id927681" class="indexterm"></span>[comment](#comment/syntax) lines and are also discarded.

<span class="term">Clip</span>  
<span id="id927707" class="indexterm"></span><span id="clip chomping/"></span>*Clipping* is the default behavior used if no explicit chomping indicator is specified. In this case, The <span id="id927723" class="indexterm"></span>[line break](#line%20break%20character/) character of the last non-<span id="id927739" class="indexterm"></span>[empty line](#empty%20line/) (if any) is preserved in the <span id="id927750" class="indexterm"></span>[scalar’s content](#scalar/syntax). However, any trailing <span id="id927766" class="indexterm"></span>[empty lines](#empty%20line/) are considered to be (empty) <span id="id927779" class="indexterm"></span>[comment](#comment/syntax) lines and are discarded.

<span class="term">Keep</span>  
<span id="id927804" class="indexterm"></span><span id="keep chomping/"></span>*Keeping* is specified using the <span id="id927819" class="indexterm"></span><span id="+ keep chomping/"></span>*“<span class="quote">**`+`**</span>” chomping indicator*. In this case, the <span id="id927840" class="indexterm"></span>[line break](#line%20break%20character/) character of the last non-<span id="id927854" class="indexterm"></span>[empty line](#empty%20line/) (if any) is preserved in the <span id="id927867" class="indexterm"></span>[scalar’s content](#scalar/syntax). In addition, any trailing <span id="id927884" class="indexterm"></span>[empty lines](#empty%20line/) are each considered to <span id="id927896" class="indexterm"></span>[present](#present/) a single trailing <span id="id927909" class="indexterm"></span>[content](#content/syntax)<span id="id927924" class="indexterm"></span>[line break](#line%20break%20character/). Note that these <span id="id927940" class="indexterm"></span>[line breaks](#line%20break%20character/) are not subject to <span id="id927953" class="indexterm"></span>[folding](#line%20folding/).

</div>

The chomping method used is a <span id="id927971" class="indexterm"></span>[presentation detail](#presentation%20detail/) and is not reflected in the <span id="id927987" class="indexterm"></span>[serialization tree](#serialization/) (and hence the <span id="id927998" class="indexterm"></span>[representation](#representation/) graph).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[167]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-chomping-indicator(t)"></span>c-chomping-indicator(t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">t</code> = strip ⇒ “<span class="quote">-</span>”<br />
<code class="varname">t</code> = clip  ⇒ /* empty */<br />
<code class="varname">t</code> = keep  ⇒ “<span class="quote">+</span>”</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Thus, the final <span id="id928054" class="indexterm"></span>[line break](#line%20break%20character/) of a <span id="id928068" class="indexterm"></span>[block scalar](#block%20scalar%20style/syntax) may be included or excluded from the <span id="id928085" class="indexterm"></span>[content](#content/syntax), depending on the specified chomping indicator.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[168]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-chomped-last(t)"></span>b-chomped-last(t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">t</code> = strip ⇒ <a href="#b-strip-last">b-strip-last</a><br />
<code class="varname">t</code> = clip  ⇒ <a href="#b-keep-last">b-keep-last</a><br />
<code class="varname">t</code> = keep  ⇒ <a href="#b-keep-last">b-keep-last</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[169]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-strip-last"></span>b-strip-last</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-ignored-any">b-ignored-any</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[170]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-keep-last"></span>b-keep-last</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-normalized">b-normalized</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id928179"></span>

**Example 9.22. Chomping Final Line Break**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>strip: |-
  text¶
clip: |
  text↓
keep: |+
  text⇓</code></pre>
<pre class="synopsis"><code>Legend:
  b-strip-last
  b-keep-last</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;strip&quot;
  : !!str &quot;text&quot;,
  ? !!str &quot;clip&quot;
  : !!str &quot;text\n&quot;,
  ? !!str &quot;keep&quot;
  : !!str &quot;text\L&quot;,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

Similarly, <span id="id928272" class="indexterm"></span>[empty lines](#empty%20line/) immediately following the <span id="id928285" class="indexterm"></span>[block scalar](#block%20scalar%20style/syntax) may be interpreted either as <span id="id928301" class="indexterm"></span>[presenting](#present/) trailing <span id="id928314" class="indexterm"></span>[line breaks](#line%20break%20character/) or as (empty) <span id="id928327" class="indexterm"></span>[comment](#comment/syntax) lines, depending on the specified chomping indicator.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[171]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-chomped-empty(n,t)"></span>l-chomped-empty(n,t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">t</code> = strip ⇒ <a href="#l-strip-empty(n)">l-strip-empty(n)</a><br />
<code class="varname">t</code> = clip  ⇒ <a href="#l-strip-empty(n)">l-strip-empty(n)</a><br />
<code class="varname">t</code> = keep  ⇒ <a href="#l-keep-empty(n)">l-keep-empty(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[172]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-strip-empty(n)"></span>l-strip-empty(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#s-indent(n)">s-indent(≤n)</a> <a href="#b-ignored-any">b-ignored-any</a> )* <a href="#l-trail-comments(n)">l-trail-comments(n)</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[173]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-keep-empty(n)"></span>l-keep-empty(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-empty(n,s)">l-empty(n,literal)</a>* <a href="#l-trail-comments(n)">l-trail-comments(n)</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Explicit <span id="id928445" class="indexterm"></span>[comment](#comment/syntax) lines may then follow. To prevent ambiguity, the first such <span id="id928461" class="indexterm"></span>[comment](#comment/syntax) line must be less <span id="id928476" class="indexterm"></span>[indented](#indentation%20space/) than the <span id="id928490" class="indexterm"></span>[block scalar content](#block%20scalar%20style/syntax). Additional <span id="id928506" class="indexterm"></span>[comment](#comment/syntax) lines, if any, are not so restricted.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[174]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-trail-comments(n)"></span>l-trail-comments(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(&lt;n)</a> <a href="#c-nb-comment-text">c-nb-comment-text</a> <a href="#b-ignored-any">b-ignored-any</a><br />
<a href="#l-comment">l-comment</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id928558"></span>

**Example 9.23. Block Scalar Chomping**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code> # Strip
  # Comments:
strip: |-
  # text¶
··⇓
·# Clip
··# comments:
↓
clip: |
  # text↓
·¶
·# Keep
··# comments:
↓
keep: |+
  # text⇓
↓
·# Trail
··# comments.</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  ? !!str &quot;strip&quot;
  : !!str &quot;# text&quot;,
  ? !!str &quot;clip&quot;
  : !!str &quot;# text\n&quot;,
  ? !!str &quot;keep&quot;
  : !!str &quot;# text\L\n&quot;,
]</code></pre>
<pre class="synopsis"><code>Legend:
  l-strip-empty(n)
  l-keep-empty(n)
  l-trail-comments(n)</code></pre></td>
</tr>
</tbody>
</table>

</div>

Note that if a <span id="id928685" class="indexterm"></span>[block scalar](#block%20scalar%20style/syntax) consists of only <span id="id928701" class="indexterm"></span>[empty lines](#empty%20line/), then these lines are considered trailing lines and hence are affected by chomping.

<div class="example">

<span id="id928716"></span>

**Example 9.24. Empty Scalar Chomping**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>strip: &gt;-
↓
clip: &gt;
↓
keep: |+
↓</code></pre>
<pre class="synopsis"><code>Legend:
  l-strip-empty(n)
  l-keep-empty(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  ? !!str &quot;strip&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;clip&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;keep&quot;
  : !!str &quot;\n&quot;,
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id928806"></span>9.3. Block Scalar Styles

</div>

</div>

</div>

YAML provides two <span id="id928815" class="indexterm"></span><span id="block scalar style/syntax"></span>*Block scalar styles*, <span id="id928833" class="indexterm"></span>[literal](#literal%20style/syntax) and <span id="id928849" class="indexterm"></span>[folded](#folded%20style/syntax). The block scalar <span id="id928864" class="indexterm"></span>[content](#content/syntax) is ended by a less-<span id="id928879" class="indexterm"></span>[indented](#indentation%20space/) line or the end of the characters <span id="id928893" class="indexterm"></span>[stream](#stream/syntax).

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id928909"></span>9.3.1. Literal

</div>

</div>

</div>

The <span id="id928917" class="indexterm"></span><span id="literal style/syntax"></span>*literal style* is the simplest, most restricted and most readable <span id="id928935" class="indexterm"></span>[scalar style](#scalar/syntax). It is especially suitable for source code or other text containing significant use of <span id="id928952" class="indexterm"></span>[indicators](#indicator/), <span id="id928964" class="indexterm"></span>[escape sequences](#escaping%20in%20double-quoted%20style/) and <span id="id928979" class="indexterm"></span>[line breaks](#line%20break%20character/). In particular, literal content lines may begin with a <span id="id928995" class="indexterm"></span>[tab](#tab/) or a <span id="id929006" class="indexterm"></span>[“<span class="quote">**`#`**</span>”](##%20comment/) character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[175]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l+literal(n)"></span>c-l+literal(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-b-block-header(s,m,t)">c-b-block-header(literal,m,t)</a><br />
<a href="#l-literal-content(n,t)">l-literal-content(n+m,t)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id929052"></span>

**Example 9.25. Literal Scalar**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>| # Simple block scalar↓
 literal↓
 →text↓</code></pre>
<pre class="synopsis"><code>Legend:
  c-b-block-header(s,m,t)
  l-literal-content(n,t)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;literal\n\
        \ttext\n&quot;
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

Inside literal scalars, each non-<span id="id929137" class="indexterm"></span>[empty line](#empty%20line/) may be preceded by any number of <span id="id929152" class="indexterm"></span>[empty lines](#empty%20line/). No processing is performed on these lines except for stripping the <span id="id929165" class="indexterm"></span>[indentation](#indentation%20space/). In particular, such lines are never <span id="id929178" class="indexterm"></span>[folded](#line%20folding/). Literal non-<span id="id929193" class="indexterm"></span>[empty lines](#empty%20line/) may include only spaces, <span id="id929206" class="indexterm"></span>[tabs](#tab/), and other <span id="id929217" class="indexterm"></span>[printable](#printable%20character/) characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[176]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-literal-text(n)"></span>l-nb-literal-text(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-empty(n,s)">l-empty(n,block)</a>* <a href="#s-indent(n)">s-indent(n)</a> <a href="#nb-char">nb-char</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id929264"></span>

**Example 9.26. Literal Text**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>|
·
··
··literal↓
·
··text↓
↓
·# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;\nliteral\n\ntext\n&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-nb-literal-text(n)</code></pre></td>
</tr>
</tbody>
</table>

</div>

The <span id="id929338" class="indexterm"></span>[line break](#line%20break%20character/) following a non-<span id="id929352" class="indexterm"></span>[empty](#empty%20line/) inner literal line is <span id="id929365" class="indexterm"></span>[normalized](#line%20break%20normalization/). Again, such <span id="id929381" class="indexterm"></span>[line breaks](#line%20break%20character/) are never <span id="id929394" class="indexterm"></span>[folded](#line%20folding/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[177]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-literal-inner(n)"></span>l-literal-inner(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-nb-literal-text(n)">l-nb-literal-text(n)</a> <a href="#b-normalized">b-normalized</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id929431"></span>

**Example 9.27. Inner Literal Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>|
·
··
··literal↓
·
··text↓
↓
·# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;\nliteral\n\ntext\n&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-nb-literal-inner(n)
  b-normalized</code></pre></td>
</tr>
</tbody>
</table>

</div>

The <span id="id929519" class="indexterm"></span>[line break](#line%20break%20character/) following the final non-<span id="id929533" class="indexterm"></span>[empty](#empty%20line/) literal line is subject to <span id="id929548" class="indexterm"></span>[chomping](#chomping/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[178]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-literal-last(n,t)"></span>l-literal-last(n,t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-nb-literal-text(n)">l-nb-literal-text(n)</a> <a href="#b-chomped-last(t)">b-chomped-last(t)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Trailing <span id="id929587" class="indexterm"></span>[empty lines](#empty%20line/) following the last literal non-<span id="id929600" class="indexterm"></span>[empty line](#empty%20line/), if any, are also subject to <span id="id929613" class="indexterm"></span>[chomping](#chomping/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[179]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-literal-content(n,t)"></span>l-literal-content(n,t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#l-nb-literal-inner(n)">l-literal-inner(n)</a>* <a href="#l-nb-literal-last(n,t)">l-literal-last(n,t)</a> )?<br />
<a href="#l-chomped-empty(n,t)">l-chomped-empty(n,t)</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id929660"></span>

**Example 9.28. Last Literal Line**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>|
·
··
··literal↓
·
··text↓
↓
·# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!str &quot;\nliteral\n\ntext\n&quot;</code></pre>
<pre class="synopsis"><code>Legend:
  l-nb-literal-last(n,t)
  b-chomped-last(t)
  l-chomped-empty(n,t)</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id929764"></span>9.3.2. Folded

</div>

</div>

</div>

The <span id="id929773" class="indexterm"></span><span id="folded style/syntax"></span>*folded style* is similar to the <span id="id929790" class="indexterm"></span>[literal style](#literal%20style/syntax). However, unlike <span id="id929806" class="indexterm"></span>[literal content](#literal%20style/syntax), folded content is subject to (block) <span id="id929822" class="indexterm"></span>[line folding](#line%20folding/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[180]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l+folded(n)"></span>c-l+folded(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-b-block-header(s,m,t)">c-b-block-header(folded,m,t)</a><br />
<a href="#l-folded-content(n,t)">l-folded-content(n+m,t)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id929864"></span>

**Example 9.29. Folded Scalar**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&gt; # Simple folded scalar↓
 folded↓
 text↓
 →lines↓</code></pre>
<pre class="synopsis"><code>Legend:
  c-b-block-header(s,m,t)
  l-folded-content(n,t)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;folded text\n\
        \tlines\n&quot;
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

<span id="id929948" class="indexterm"></span>[Line folding](#line%20folding/) allows long <span id="id929961" class="indexterm"></span>[content](#content/syntax) lines to be broken anywhere a single space character separates two non-space characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[181]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-nb-folded-text(n)"></span>s-nb-folded-line(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(n)</a> <a href="#ns-char">ns-char</a> <a href="#nb-char">nb-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[182]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-folded-lines(n)"></span>l-nb-folded-lines(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#s-nb-folded-text(n)">s-nb-folded-line(n)</a><br />
  <a href="#b-l-folded-any(n,s)">b-l-folded-any(n,folded)</a> )*<br />
<a href="#s-nb-folded-text(n)">s-nb-folded-line(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id930040"></span>

**Example 9.30. Folded Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&gt;
·folded↓
·line↓
↓
·next
·line↓
&#10;   * bullet
   * list
&#10;·last↓
·line↓
&#10;# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;folded line\n\
        next line\n\
        \  * bullet\n\
        \  * list\n\
        last line\n&quot;
]</code></pre>
<pre class="synopsis"><code>Legend:
  l-nb-folded-lines(n)</code></pre></td>
</tr>
</tbody>
</table>

</div>

Lines starting with <span id="id930118" class="indexterm"></span>[white space](#white%20space/) characters (<span id="id930131" class="indexterm"></span><span id="more indented line/"></span>*“<span class="quote">more indented</span>” lines*) are not <span id="id930150" class="indexterm"></span>[folded](#line%20folding/). Note that folded scalars, like <span id="id930161" class="indexterm"></span>[literal scalars](#literal%20style/syntax), may contain <span id="id930177" class="indexterm"></span>[tab](#tab/) characters. However, any such characters must be properly <span id="id930190" class="indexterm"></span>[indented](#indentation%20space/) using only space characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[183]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="b-l-spaced(n)"></span>b-l-spaced(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#b-normalized">b-normalized</a> <a href="#l-empty(n,s)">l-empty(n,folded)</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[184]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-nb-spaced-text(n)"></span>s-nb-spaced-text(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(n)</a> <a href="#s-white">s-white</a> <a href="#nb-char">nb-char</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[185]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-spaced-lines(n)"></span>l-nb-spaced-lines(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#s-nb-spaced-text(n)">s-nb-spaced-text(n)</a> <a href="#b-l-spaced(n)">b-l-spaced(n)</a> )*<br />
<a href="#s-nb-spaced-text(n)">s-nb-spaced-text(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id930287"></span>

**Example 9.31. Spaced Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&gt;
 folded
 line
&#10; next
 line
&#10;···* bullet↓
···* list↓
&#10; last
 line
&#10;# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;folded line\n\
        next line\n\
        \  * bullet\n\
        \  * list\n\
        last line\n&quot;
]</code></pre>
<pre class="synopsis"><code>Legend:
  l-nb-spaced-lines(n)</code></pre></td>
</tr>
</tbody>
</table>

</div>

Folded content may start with either line type. If the <span id="id930358" class="indexterm"></span>[content](#content/syntax) begins with a “<span class="quote">more indented</span>” line (starting with spaces), an <span id="id930378" class="indexterm"></span>[indentation indicator](#indentation%20indicator/) must be specified in the block header. Note that leading <span id="id930394" class="indexterm"></span>[empty lines](#empty%20line/) and <span id="id930407" class="indexterm"></span>[empty lines](#empty%20line/) separating lines of a different type are never <span id="id930420" class="indexterm"></span>[folded](#line%20folding/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[186]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-start-with-folded(n)"></span>l-nb-start-with-folded(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-empty(n,s)">l-empty(n,block)</a>* <a href="#l-nb-folded-lines(n)">l-nb-folded-lines(n)</a><br />
( <a href="#b-normalized">b-normalized</a> <a href="#l-nb-start-with-spaced(n)">l-nb-start-with-spaced(n)</a> )?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[187]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-start-with-spaced(n)"></span>l-nb-start-with-spaced(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#l-empty(n,s)">l-empty(n,block)</a>* <a href="#l-nb-spaced-lines(n)">l-nb-spaced-lines(n)</a><br />
( <a href="#b-normalized">b-normalized</a> <a href="#l-nb-start-with-folded(n)">l-nb-start-with-folded(n)</a> )?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[188]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-nb-start-with-any(n)"></span>l-nb-start-with-any(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#l-nb-start-with-folded(n)">l-nb-start-with-folded(n)</a><br />
| <a href="#l-nb-start-with-spaced(n)">l-nb-start-with-spaced(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id930530"></span>

**Example 9.32. Empty Separation Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&gt;
 folded
 line
&#10; next
 line↓
↓
   * bullet
   * list↓
↓
 last
 line
&#10;# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;folded line\n\
        next line\n\
        \  * bullet\n\
        \  * list\n\
        last line\n&quot;
]</code></pre>
<pre class="synopsis"><code>Legend:
  b-normalized l-empty(n,s)</code></pre></td>
</tr>
</tbody>
</table>

</div>

The final <span id="id930626" class="indexterm"></span>[line break](#line%20break%20character/), and trailing <span id="id930640" class="indexterm"></span>[empty lines](#empty%20line/), if any, are subject to <span id="id930653" class="indexterm"></span>[chomping](#chomping/) and are never <span id="id930666" class="indexterm"></span>[folded](#line%20folding/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[189]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-folded-content(n,t)"></span>l-folded-content(n,t)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">( <a href="#l-nb-start-with-any(n)">l-nb-start-with-any(n)</a> <a href="#b-chomped-last(t)">b-chomped-last(t)</a> )?<br />
<a href="#l-chomped-empty(n,t)">l-chomped-empty(n,t)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id930711"></span>

**Example 9.33. Final Empty Lines**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>&gt;
 folded
 line
&#10; next
 line
&#10;   * bullet
   * list
&#10; last
 line↓
↓
# Comment</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;folded line\n\
        next line\n\
        \  * bullet\n\
        \  * list\n\
        last line\n&quot;
]</code></pre>
<pre class="synopsis"><code>Legend:
  b-chomped-last(t) l-chomped-empty(n,t)</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

</div>

<div class="chapter" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id930798"></span>Chapter 10. Collection Styles

</div>

</div>

</div>

<span id="id930806" class="indexterm"></span><span id="collection/syntax"></span>*Collection content* can be presented in a single <span id="id930824" class="indexterm"></span><span id="flow collection style/syntax"></span>*flow style* and a single <span id="id930844" class="indexterm"></span><span id="block collection style/syntax"></span>*block style* for each of the two <span id="id930860" class="indexterm"></span>[collection kinds](#kind/) (<span id="id930872" class="indexterm"></span>[sequence](#sequence/syntax) and <span id="id930888" class="indexterm"></span>[mapping](#mapping/syntax)). In addition, YAML provides several <span id="id930905" class="indexterm"></span>[in-line](#in-line%20style/syntax) compact syntax forms for improved readability of common special cases. In all cases, the collection style is a <span id="id930923" class="indexterm"></span>[presentation detail](#presentation%20detail/) and must not be used to convey <span id="id930938" class="indexterm"></span>[content information](#content/information%20model).

A flow collection may be nested within a block collection (<span id="id930957" class="indexterm"></span>[flow-out context](#flow-out%20context/)), nested within another flow collection (<span id="id930971" class="indexterm"></span>[flow-in context](#flow-in%20context/)), or be a part of a <span id="id930985" class="indexterm"></span>[simple key](#simple%20key/) (<span id="id930999" class="indexterm"></span>[flow-key context](#flow-key%20context/)). Flow collection entries are separated by the <span id="id931013" class="indexterm"></span><span id=", end flow entry/"></span>*“<span class="quote">**`,`**</span>” indicator*. The final “<span class="quote">**`,`**</span>” may be omitted. This does not cause ambiguity because flow collection entries can never be <span id="id931040" class="indexterm"></span>[completely empty](#completely%20empty%20node/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[190]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="in-flow(c)"></span>in-flow(c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = flow-out ⇒ flow-in<br />
<code class="varname">c</code> = flow-in  ⇒ flow-in<br />
<code class="varname">c</code> = flow-key ⇒ flow-key</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id931088"></span>10.1. Sequence Styles

</div>

</div>

</div>

<span id="id931095" class="indexterm"></span><span id="sequence/syntax"></span>*Sequence content* is an ordered collection of sub-<span id="id931112" class="indexterm"></span>[nodes](#node/syntax). <span id="id931127" class="indexterm"></span>[Comments](#comment/syntax) may be interleaved between the sub-<span id="id931142" class="indexterm"></span>[nodes](#node/syntax). Sequences may be <span id="id931157" class="indexterm"></span>[presented](#present/) in a <span id="id931170" class="indexterm"></span>[flow style](#flow%20sequence%20style/syntax) or a <span id="id931188" class="indexterm"></span>[block style](#block%20sequence%20style/syntax). YAML provides compact notations for <span id="id931202" class="indexterm"></span>[in-line](#in-line%20style/syntax) nesting of a <span id="id931218" class="indexterm"></span>[collection](#collection/syntax) in a <span id="id931234" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax) and for nesting a <span id="id931250" class="indexterm"></span>[single pair mapping](#single%20pair%20style/syntax) in a <span id="id931269" class="indexterm"></span>[flow sequence](#flow%20sequence%20style/syntax).

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id931285"></span>10.1.1. Flow Sequences

</div>

</div>

</div>

<span id="id931292" class="indexterm"></span><span id="flow sequence style/syntax"></span>*Flow sequence content* is denoted by surrounding <span id="id931310" class="indexterm"></span><span id="[ start flow sequence/"></span>*“<span class="quote">**`[`**</span>”* and <span id="id931329" class="indexterm"></span><span id="] end flow sequence/"></span>*“<span class="quote">**`]`**</span>”* characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[191]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-flow-sequence(n,c)"></span>c-flow-sequence(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-sequence-start">“<span class="quote">[</span>”</a> <a href="#s-separate(n,c)">s-separate(n,c)</a>?<br />
<a href="#ns-s-flow-seq-inner(n,c)">ns-s-flow-seq-inner(n,c)</a>*<br />
<a href="#ns-s-flow-seq-last(n,c)">ns-s-flow-seq-last(n,c)</a>?<br />
<a href="#c-sequence-end">“<span class="quote">]</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Sequence entries are separated by a <span id="id931402" class="indexterm"></span>[“<span class="quote">**`,`**</span>”](#,%20end%20flow%20entry/) character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[192]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-seq-inner(n,c)"></span>ns-s-flow-seq-inner(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-s-flow-seq-entry(n,c)">ns-s-flow-seq-entry(n,c)</a> <a href="#c-collect-entry">“<span class="quote">,</span>”</a> <a href="#s-separate(n,c)">s-separate(n,c)</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

The final entry may omit the <span id="id931458" class="indexterm"></span>[“<span class="quote">**`,`**</span>”](#,%20end%20flow%20entry/) character. This does not cause ambiguity since sequence entries must not be <span id="id931477" class="indexterm"></span>[completely empty](#completely%20empty%20node/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[193]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-seq-last(n,c)"></span>ns-s-flow-seq-last(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-s-flow-seq-entry(n,c)">ns-s-flow-seq-entry(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id931509"></span>

**Example 10.1. Flow Sequence**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- [ inner, inner, ]
- [inner,last]</code></pre>
<pre class="synopsis"><code>Legend:
  c-sequence-start c-sequence-end
  ns-s-flow-seq-inner(n,c)
  ns-s-flow-seq-last(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!seq [
    !!str &quot;inner&quot;,
    !!str &quot;inner&quot;,
  ],
  !!seq [
    !!str &quot;inner&quot;,
    !!str &quot;last&quot;,
  ],
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

Any <span id="id931644" class="indexterm"></span>[flow node](#flow%20style/syntax) may be used as a flow sequence entry. In addition, YAML provides a compact form for the case where a flow sequence entry is a <span id="id931662" class="indexterm"></span>[mapping](#mapping/syntax) with a <span id="id931677" class="indexterm"></span>[single key: value pair](#single%20pair%20style/syntax), and neither the <span id="id931694" class="indexterm"></span>[mapping node](#mapping/syntax) nor its single <span id="id931709" class="indexterm"></span>[key node](#key/syntax) have any <span id="id931724" class="indexterm"></span>[properties](#node%20property/) specified.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[194]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-seq-entry(n,c)"></span>ns-s-flow-seq-entry(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#ns-flow-node(n,c)">ns-flow-node(n,</a><a href="#in-flow(c)">in-flow(c)</a>)<br />
    <a href="#s-separate(n,c)">s-separate(n,</a><a href="#in-flow(c)">in-flow(c)</a>)? )<br />
| <a href="#ns-s-flow-single-pair(n,c)">ns-s-flow-single-pair(n,</a><a href="#in-flow(c)">in-flow(c)</a>)</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id931790"></span>

**Example 10.2. Flow Sequence Entries**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>[
&quot;double
 quoted&quot;, &#39;single
           quoted&#39;,
plain
 text, [ nested ],
single: pair ,
]</code></pre>
<pre class="synopsis"><code>Legend:
  ns-flow-node(n,c)
  ns-s-flow-single-pair(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;double quoted&quot;,
  !!str &quot;single quoted&quot;,
  !!str &quot;plain text&quot;,
  !!seq [
    !!str &quot;nested&quot;,
  ],
  !!map {
    ? !!str &quot;single&quot;
    : !!str &quot;pair&quot;
  }
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id931893"></span>10.1.2. Block Sequences

</div>

</div>

</div>

A <span id="id931901" class="indexterm"></span><span id="block sequence style/syntax"></span>*block sequence* is simply a series of entries, each <span id="id931920" class="indexterm"></span>[presenting](#present/) a single <span id="id931932" class="indexterm"></span>[node](#node/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[195]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l-block-sequence(n,c)"></span>c-l-block-sequence(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-l-comments">c-l-comments</a> <a href="#l-block-seq-entry(n,c)">l-block-seq-entry(n,c)</a>+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id931973"></span>

**Example 10.3. Block Sequence**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>block: # Block
       # sequence↓
- one↓
- two : three↓</code></pre>
<pre class="synopsis"><code>Legend:
  c-l-comments
  l-block-seq-entry(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;block&quot;
  : !!seq [
    !!str &quot;one&quot;,
    !!map {
      ? !!str &quot;two&quot;
      : !!str &quot;three&quot;
    }
  ]
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

Each block sequence entry is denoted by a leading <span id="id932068" class="indexterm"></span><span id="- block sequence entry/"></span>*“<span class="quote">**`-`**</span>” indicator*, <span id="id932088" class="indexterm"></span>[separated](#separation%20space/) by spaces from the entry <span id="id932104" class="indexterm"></span>[node](#node/syntax).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[196]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-block-seq-entry(n,c)"></span>l-block-seq-entry(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(seq-spaces(n,c))</a> <a href="#c-sequence-entry">“<span class="quote">-</span>”</a><br />
<a href="#s-l+block-indented(n,c)">s-l+block-indented(seq-spaces(n,c),c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

People read the “<span class="quote">**`-`**</span>” character as part of the <span id="id932163" class="indexterm"></span>[indentation](#indentation%20space/). Hence, block sequence entries require one less space of <span id="id932178" class="indexterm"></span>[indentation](#indentation%20space/), unless the block sequence is nested within another block sequence (hence the need for the <span id="id932193" class="indexterm"></span><span id="block-in context/"></span>*block-in context* and <span id="id932208" class="indexterm"></span><span id="block-out context/"></span>*block-out context*).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[197]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="seq-spaces(n,c)"></span>seq-spaces(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><code class="varname">c</code> = block-out ⇒ n-1<br />
<code class="varname">c</code> = block-in  ⇒ n</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id932251"></span>

**Example 10.4. Block Sequence Entry Indentation**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>block:
- one
-
·- two</code></pre>
<pre class="synopsis"><code>Legend:
  s-indent(n)
  s-l+block-indented(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;block&quot;
  : !!seq [
    !!str &quot;one&quot;,
    !!seq [
      !!str &quot;two&quot;
    ]
  ]
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

The entry <span id="id932346" class="indexterm"></span>[node](#node/syntax) may be either <span id="id932361" class="indexterm"></span>[completely empty](#completely%20empty%20node/), a normal <span id="id932374" class="indexterm"></span>[block node](#block%20style/syntax), or use a compact in-line form.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[198]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l+block-indented(n,c)"></span>s-l+block-indented(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#s-l-empty-block">s-l-empty-block</a><br />
| <a href="#s-l+block-node(n,c)">s-l+block-node(n,c)</a><br />
| <a href="#s-l+block-in-line(n)">s-l+block-in-line(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

The compact <span id="id932428" class="indexterm"></span><span id="in-line style/syntax"></span>*in-line* form may be used in the common case when the block sequence entry is itself a <span id="id932446" class="indexterm"></span>[block collection](#block%20collection%20style/syntax), and neither the <span id="id932462" class="indexterm"></span>[collection](#collection/syntax) entry nor its first nested <span id="id932479" class="indexterm"></span>[node](#node/syntax) have any <span id="id932493" class="indexterm"></span>[properties](#node%20property/) specified. In this case, the nested <span id="id932507" class="indexterm"></span>[collection](#collection/syntax) may be specified in the same line as the “<span class="quote">**`-`**</span>” character, and any following spaces are considered part of the in-line nested <span id="id932532" class="indexterm"></span>[collection’s](#collection/syntax) <span id="id932547" class="indexterm"></span>[indentation](#indentation%20space/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[199]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l+block-in-line(n)"></span>s-l+block-in-line(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(m&gt;0)</a><br />
( <a href="#ns-l-in-line-sequence(n)">ns-l-in-line-sequence(n+1+m)</a><br />
| <a href="#ns-l-in-line-mapping(n)">ns-l-in-line-mapping(n+1+m)</a> )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

An <span id="id932601" class="indexterm"></span><span id="in-line sequence style/"></span>*in-line block sequence* begins with an <span id="id932617" class="indexterm"></span>[indented](#indentation%20space/) same-line sequence entry, followed by optional additional normal block sequence entries, properly <span id="id932634" class="indexterm"></span>[indented](#indentation%20space/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[200]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l-in-line-sequence(n)"></span>ns-l-in-line-sequence(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-sequence-entry">“<span class="quote">-</span>”</a> <a href="#s-l+block-indented(n,c)">s-l+block-indented(n,block-out)</a><br />
<a href="#l-block-seq-entry(n,c)">l-block-seq-entry(n,block-out)</a>*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id932683"></span>

**Example 10.5. Block Sequence Entry Types**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- # Empty
- |
 block node
-·- one # in-line
··- two # sequence
- one: two # in-line
           # mapping</code></pre>
<pre class="synopsis"><code>Legend:
  s-l-empty-block
  s-l+block-node(n,c)
  s-l+block-in-line(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!str &quot;&quot;,
  !!str &quot;block node\n&quot;,
  !!seq [
    !!str &quot;one&quot;,
    !!str &quot;two&quot;,
  ]
  !!map {
    ? !!str &quot;one&quot;
    : !!str &quot;two&quot;,
  }
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

<div class="sect1" lang="en">

<div class="titlepage">

<div>

<div>

## <span id="id932806"></span>10.2. Mapping Styles

</div>

</div>

</div>

A <span id="id932814" class="indexterm"></span><span id="mapping/syntax"></span>*mapping node* is an unordered collection of <span id="id932831" class="indexterm"></span><span id="key/syntax"></span>*key:* <span id="id932847" class="indexterm"></span><span id="value/syntax"></span>*value* pairs. Of necessity, these pairs are <span id="id932863" class="indexterm"></span>[presented](#present/) in some <span id="id932876" class="indexterm"></span>[order](#key%20order/) in the characters <span id="id932889" class="indexterm"></span>[stream](#stream/syntax). As a <span id="id932904" class="indexterm"></span>[serialization detail](#serialization%20detail/), this <span id="id932917" class="indexterm"></span>[key order](#key%20order/) is preserved in the <span id="id932930" class="indexterm"></span>[serialization tree](#serialization/). However it is not reflected in the <span id="id932944" class="indexterm"></span>[representation graph](#representation/) and hence must not be used when <span id="id932957" class="indexterm"></span>[constructing](#construct/) native data structures. It is an error for two <span id="id932970" class="indexterm"></span>[equal](#equality/) keys to appear in the same mapping node. In such a case the YAML <span id="id932984" class="indexterm"></span>[processor](#processor/) may continue, ignoring the second key: value pair and issuing an appropriate warning. This strategy preserves a consistent information model for one-pass and random access <span id="id932998" class="indexterm"></span>[applications](#application/).

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id933010"></span>10.2.1. Flow Mappings

</div>

</div>

</div>

<span id="id933017" class="indexterm"></span><span id="flow mapping style/syntax"></span>*Flow mapping content* is denoted by surrounding <span id="id933036" class="indexterm"></span><span id="{ start flow mapping/"></span>*“<span class="quote">**`{`**</span>”* and <span id="id933054" class="indexterm"></span><span id="} end flow mapping/"></span>*“<span class="quote">**`}`**</span>”* characters.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[201]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-flow-mapping(n,c)"></span>c-flow-mapping(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-mapping-start">“<span class="quote">{</span>”</a> <a href="#s-separate(n,c)">s-separate(n,c)</a>?<br />
<a href="#ns-s-flow-map-inner(n,c)">ns-s-flow-map-inner(n,c)</a>*<br />
<a href="#ns-s-flow-map-last(n,c)">ns-s-flow-map-last(n,c)</a>?<br />
<a href="#c-mapping-end">“<span class="quote">}</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Mapping entries are separated by a <span id="id933127" class="indexterm"></span>[“<span class="quote">**`,`**</span>”](#,%20end%20flow%20entry/) character.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[202]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-map-inner(n,c)"></span>ns-s-flow-map-inner(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-s-flow-map-entry(n,c)">ns-s-flow-map-entry(n,c)</a> <a href="#c-collect-entry">“<span class="quote">,</span>”</a> <a href="#s-separate(n,c)">s-separate(n,c)</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

The final entry may omit the <span id="id933183" class="indexterm"></span>[“<span class="quote">**`,`**</span>”](#,%20end%20flow%20entry/) character. This does not cause ambiguity since mapping entries must not be <span id="id933202" class="indexterm"></span>[completely empty](#completely%20empty%20node/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[203]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-map-last(n,c)"></span>ns-s-flow-map-last(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-s-flow-map-entry(n,c)">ns-s-flow-map-entry(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id933234"></span>

**Example 10.6. Flow Mappings**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- { inner : entry , also: inner , }
- {inner: entry,last : entry}</code></pre>
<pre class="synopsis"><code>Legend:
  c-mapping-start c-mapping-end
  ns-s-flow-map-inner(n,c)
  ns-s-flow-map-last(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!map {
    ? !!str &quot;inner&quot;
    : !!str &quot;entry&quot;,
    ? !!str &quot;also&quot;
    : !!str &quot;inner&quot;
  },
  !!map {
    ? !!str &quot;inner&quot;
    : !!str &quot;entry&quot;,
    ? !!str &quot;last&quot;
    : !!str &quot;entry&quot;
  }
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

Flow mappings allow two forms of keys: explicit and simple.

<div class="variablelist">

<span class="term">Explicit Keys</span>  
An <span id="id933382" class="indexterm"></span><span id="explicit key/"></span>*explicit key* is denoted by the <span id="id933396" class="indexterm"></span><span id="? mapping key/"></span>*“<span class="quote">**`?`**</span>” indicator*, followed by <span id="id933417" class="indexterm"></span>[separation](#separation%20space/) spaces.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[204]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-flow-separated(n,c)"></span>s-flow-separated(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  ( <a href="#s-separate(n,c)">s-separate(n,c)</a> <a href="#ns-flow-node(n,c)">ns-flow-node(n,</a><a href="#in-flow(c)">in-flow(c)</a>)<br />
    <a href="#s-separate(n,c)">s-separate(n,c)</a>? )<br />
| ( <a href="#e-empty-flow">e-empty-flow</a> <a href="#s-separate(n,c)">s-separate(n,c)</a> )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[205]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-s-flow-explicit-key(n,c)"></span>c-s-flow-explicit-key(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-mapping-key">“<span class="quote">?</span>”</a> <a href="#s-flow-separated(n,c)">s-flow-separated(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Simple Keys</span>  
A <span id="id933516" class="indexterm"></span><span id="simple key/"></span>*simple key* has no identifying mark. It is recognized as being a key either due to being inside a flow mapping, or by being followed by an explicit value. Hence, to avoid unbound lookahead in YAML <span id="id933534" class="indexterm"></span>[processors](#processor/), simple keys are restricted to a single line and must not span more than 1024 <span id="id933548" class="indexterm"></span>[stream](#stream/syntax) characters (hence the need for the <span id="id933564" class="indexterm"></span><span id="flow-key context/"></span>*flow-key context*). Note the 1024 character limit is in terms of Unicode characters rather than stream octets, and that it includes the <span id="id933583" class="indexterm"></span>[separation](#separation%20space/) following the key itself.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[206]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-simple-key(n,c)"></span>ns-s-flow-simple-key(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-flow-node(n,c)">ns-flow-node(n,flow-key)</a> <a href="#s-flow-separated(n,c)">s-flow-separated(n,c)</a>?</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id933629"></span>

**Example 10.7. Flow Mapping Keys**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>{
?° : value # Empty key
? explicit
 key: value,
simple key : value
[ collection, simple, key ]: value
}</code></pre>
<pre class="synopsis"><code>Legend:
  c-s-flow-explicit-key(n,c)
  ns-s-flow-simple-key(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;&quot;
  : !!str &quot;value&quot;,
  ? !!str &quot;explicit key&quot;
  : !!str &quot;value&quot;,
  ? !!str &quot;simple key&quot;
  : !!str &quot;value&quot;,
  ? !!seq [
    !!str &quot;collection&quot;,
    !!str &quot;simple&quot;,
    !!str &quot;key&quot;
  ]
  : !!str &quot;value&quot;
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="example">

<span id="id933726"></span>

**Example 10.8. Invalid Flow Mapping Keys**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="screen"><code>{
multi-line
 simple key : value,
very long ...(&gt;1KB)... key: value
}</code></pre></td>
<td><pre class="screen"><code>ERROR:
- A simple key is restricted
  to only one line.
- A simple key must not be
  longer than 1024 characters.</code></pre></td>
</tr>
</tbody>
</table>

</div>

Flow mappings also allow two forms of values, explicit and <span id="id933797" class="indexterm"></span>[completely empty](#completely%20empty%20node/).

<div class="variablelist">

<span class="term">Explicit Values</span>  
An <span id="id933822" class="indexterm"></span><span id="explicit value/"></span>*explicit value* is denoted by the <span id="id933837" class="indexterm"></span><span id=": mapping value/"></span>*“<span class="quote">**`:`**</span>” indicator*, followed by <span id="id933859" class="indexterm"></span>[separation](#separation%20space/) spaces.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[207]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-s-flow-explicit-value(n,c)"></span>c-s-flow-explicit-value(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-mapping-value">“<span class="quote">:</span>”</a> <a href="#s-flow-separated(n,c)">s-flow-separated(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id933901"></span>

**Example 10.9. Flow Mapping Values**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>{
key : value,
empty:° # empty value↓
}</code></pre>
<pre class="synopsis"><code>Legend:
  c-s-flow-explicit-value(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;key&quot;
  : !!str &quot;value&quot;,
  ? !!str &quot;empty&quot;
  : !!str &quot;&quot;,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

Thus, there are four possible combinations for a flow mapping entry:

<div class="itemizedlist">

- Explicit key and explicit value:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[208]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-s-flow-explicit-explicit(n,c)"></span>c-s-flow-explicit-explicit(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-s-flow-explicit-key(n,c)">c-s-flow-explicit-key(n,c)</a><br />
<a href="#c-s-flow-explicit-value(n,c)">c-s-flow-explicit-value(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Explicit key and <span id="id934018" class="indexterm"></span>[completely empty](#completely%20empty%20node/) value:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[209]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-s-flow-explicit-empty(n,c)"></span>c-s-flow-explicit-empty(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-s-flow-explicit-key(n,c)">c-s-flow-explicit-key(n,c)</a> <a href="#e-empty-flow">e-empty-flow</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Simple key and explicit value:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[210]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-simple-explicit(n,c)"></span>ns-s-flow-simple-explicit(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-s-flow-simple-key(n,c)">ns-s-flow-simple-key(n,c)</a><br />
<a href="#c-s-flow-explicit-value(n,c)">c-s-flow-explicit-value(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- Simple key and <span id="id934099" class="indexterm"></span>[completely empty](#completely%20empty%20node/) value:

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[211]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-simple-empty(n,c)"></span>ns-s-flow-simple-empty(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-s-flow-simple-key(n,c)">ns-s-flow-simple-key(n,c)</a> <a href="#e-empty-flow">e-empty-flow</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

Inside flow mappings, all four combinations may be used.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[212]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-map-entry(n,c)"></span>ns-s-flow-map-entry(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-s-flow-explicit-explicit(n,c)">c-s-flow-explicit-explicit(n,c)</a><br />
| <a href="#c-s-flow-explicit-empty(n,c)">c-s-flow-explicit-empty(n,c)</a><br />
| <a href="#ns-s-flow-simple-explicit(n,c)">ns-s-flow-simple-explicit(n,c)</a><br />
| <a href="#ns-s-flow-simple-empty(n,c)">ns-s-flow-simple-empty(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id934182"></span>

**Example 10.10. Flow Mapping Key: Value Pairs**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>{
? explicit key1 : Explicit value,
? explicit key2 :° , # Explicit empty
? explicit key3,     # Empty value
simple key1 : explicit value,
simple key2 :° ,     # Explicit empty
simple key3,         # Empty value
}</code></pre>
<pre class="synopsis"><code>Legend:
  c-s-flow-explicit-explicit(n,c)
  c-s-flow-explicit-empty(n,c)
  ns-s-flow-simple-explicit(n,c)
  ns-s-flow-simple-empty(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;explicit key1&quot;
  : !!str &quot;explicit value&quot;,
  ? !!str &quot;explicit key2&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;explicit key3&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;simple key1&quot;
  : !!str &quot;explicit value&quot;,
  ? !!str &quot;simple key2&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;simple key3&quot;
  : !!str &quot;&quot;,
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

YAML also allows omitting the surrounding “<span class="quote">**`{`**</span>” and “<span class="quote">**`}`**</span>” characters when nesting a flow mapping in a <span id="id934334" class="indexterm"></span>[flow sequence](#flow%20sequence%20style/syntax) if the mapping consists of a <span id="id934350" class="indexterm"></span><span id="single pair style/syntax"></span>*single key: value pair* and neither the mapping nor the key have any <span id="id934370" class="indexterm"></span>[properties](#node%20property/) specified. In this case, only three of the combinations may be used, to prevent ambiguity.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[213]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-s-flow-single-pair(n,c)"></span>ns-s-flow-single-pair(n,c)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#c-s-flow-explicit-explicit(n,c)">c-s-flow-explicit-explicit(n,c)</a><br />
| <a href="#c-s-flow-explicit-empty(n,c)">c-s-flow-explicit-empty(n,c)</a><br />
| <a href="#ns-s-flow-simple-explicit(n,c)">ns-s-flow-simple-explicit(n,c)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id934419"></span>

**Example 10.11. Single Pair Mappings**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>[
? explicit key1 : explicit value,
? explicit key2 :° , # Explicit empty
? explicit key3,     # Implicit empty
simple key1 : explicit value,
simple key2 :° ,     # Explicit empty
]</code></pre>
<pre class="synopsis"><code>Legend:
  c-s-flow-explicit-explicit(n,c)
  c-s-flow-explicit-empty(n,c)
  ns-s-flow-simple-explicit(n,c)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq [
  !!map {
    ? !!str &quot;explicit key1&quot;
    : !!str &quot;explicit value&quot;,
  },
  !!map {
    ? !!str &quot;explicit key2&quot;
    : !!str &quot;&quot;,
  },
  !!map {
    ? !!str &quot;explicit key3&quot;
    : !!str &quot;&quot;,
  },
  !!map {
    ? !!str &quot;simple key1&quot;
    : !!str &quot;explicit value&quot;,
  },
  !!map {
    ? !!str &quot;simple key2&quot;
    : !!str &quot;&quot;,
  },
]</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

<div class="sect2" lang="en">

<div class="titlepage">

<div>

<div>

### <span id="id934537"></span>10.2.2. Block Mappings

</div>

</div>

</div>

A <span id="id934545" class="indexterm"></span><span id="block mapping style/syntax"></span>*Block mapping* is simply a series of entries, each <span id="id934563" class="indexterm"></span>[presenting](#present/) a key: value pair.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[214]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="c-l-block-mapping(n)"></span>c-l-block-mapping(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-l-comments">c-l-comments</a><br />
( <a href="#s-indent(n)">s-indent(n)</a> <a href="#ns-l-block-map-entry(n)">ns-l-block-map-entry(n)</a> )+</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id934609"></span>

**Example 10.12. Block Mappings**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>block: # Block
    # mapping↓
·key: value↓</code></pre>
<pre class="synopsis"><code>Legend:
  c-l-comments
  s-indent(n)
  ns-l-block-map-entry(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;block&quot;
  : !!map {
    ? !!str &quot;key&quot;,
    : !!str &quot;value&quot;
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

A block mapping entry may be <span id="id934709" class="indexterm"></span>[presented](#present/) using either an explicit or a simple key.

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[215]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l-block-map-entry(n)"></span>ns-l-block-map-entry(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#ns-l-block-explicit-entry(n)">ns-l-block-explicit-entry(n)</a><br />
| <a href="#ns-l-block-simple-entry(n)">ns-l-block-simple-entry(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="variablelist">

<span class="term">Explicit Key Entries</span>  
<span id="id934761" class="indexterm"></span>[Explicit key nodes](#explicit%20key/) are denoted by the <span id="id934776" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20mapping%20key/) character. YAML allows here the same <span id="id934795" class="indexterm"></span>[inline](#in-line%20style/syntax) compact notation described above for <span id="id934810" class="indexterm"></span>[block sequence](#block%20sequence%20style/syntax) entries, in which case the <span id="id934826" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20mapping%20key/) character is considered part of the key’s <span id="id934844" class="indexterm"></span>[indentation](#indentation%20space/).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[216]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l-block-explicit-key(n)"></span>ns-l-block-explicit-key(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#c-mapping-key">“<span class="quote">?</span>”</a> <a href="#s-l+block-indented(n,c)">s-l+block-indented(n,block-out)</a><br />
</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- In an explicit key entry, value nodes begin on a separate line and are denoted by by the <span id="id934901" class="indexterm"></span>[“<span class="quote">**`:`**</span>”](#:%20mapping%20value/) character. Here again YAML allows the use of the <span id="id934920" class="indexterm"></span>[inline](#in-line%20style/syntax) compact notation which case the <span id="id934935" class="indexterm"></span>[“<span class="quote">**`:`**</span>”](#:%20mapping%20value/) character is considered part of the values’s <span id="id934953" class="indexterm"></span>[indentation](#indentation%20space/).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[217]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="l-block-explicit-value(n)"></span>l-block-explicit-value(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#s-indent(n)">s-indent(n)</a> <a href="#c-mapping-value">“<span class="quote">:</span>”</a><br />
<a href="#s-l+block-indented(n,c)">s-l+block-indented(n,block-out)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- An explicit key entry may also use a <span id="id935012" class="indexterm"></span>[completely empty](#completely%20empty%20node/) value.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[218]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l-block-explicit-entry(n)"></span>ns-l-block-explicit-entry(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-l-block-explicit-key(n)">ns-l-block-explicit-key(n)</a><br />
( <a href="#l-block-explicit-value(n)">l-block-explicit-value(n)</a><br />
| <a href="#e-empty-flow">e-empty-flow</a> )</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id935061"></span>

**Example 10.13. Explicit Block Mapping Entries**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>? explicit key # implicit value↓°
? |
  block key↓
:·- one # explicit in-line
··- two # block value↓</code></pre>
<pre class="synopsis"><code>Legend:
  ns-l-block-explicit-key(n)
  l-block-explicit-value(n)
  e-empty-flow</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;explicit key&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;block key\n&quot;
  : !!seq [
    !!str &quot;one&quot;,
    !!str &quot;two&quot;,
  ]
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

<div class="variablelist">

<span class="term">Simple Key Entries</span>  
YAML allows the <span id="id935176" class="indexterm"></span>[“<span class="quote">**`?`**</span>”](#?%20mapping%20key/) character to be omitted for <span id="id935195" class="indexterm"></span>[simple keys](#simple%20key/). Similarly to flow mapping, such a key is recognized by a following <span id="id935208" class="indexterm"></span>[“<span class="quote">**`:`**</span>”](#:%20mapping%20value/) character. Again, to avoid unbound lookahead in YAML <span id="id935226" class="indexterm"></span>[processors](#processor/), simple keys are restricted to a single line and must not span more than 1024 <span id="id935239" class="indexterm"></span>[stream](#stream/syntax) characters. Again, this limit is in terms of Unicode characters rather than stream octets, and includes the <span id="id935256" class="indexterm"></span>[separation](#separation%20space/) following the key, if any.

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[219]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-block-simple-key(n)"></span>ns-block-simple-key(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-flow-node(n,c)">ns-flow-node(n,flow-key)</a><br />
<a href="#s-separate(n,c)">s-separate(n,block-out)</a>? <a href="#c-mapping-value">“<span class="quote">:</span>”</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="itemizedlist">

- In a simple key entry, an <span id="id935317" class="indexterm"></span>[explicit value](#explicit%20value/) node may be <span id="id935333" class="indexterm"></span>[presented](#present/) in the same line. Note however that in this case, the key is not considered to be a form of <span id="id935346" class="indexterm"></span>[indentation](#indentation%20space/), hence the compact <span id="id935360" class="indexterm"></span>[in-line](#in-line%20style/syntax) notation must not be used. The value following the simple key may also be <span id="id935377" class="indexterm"></span>[completely empty](#completely%20empty%20node/).

</div>

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[220]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="s-l+block-simple-value(n)"></span>s-l+block-simple-value(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top">  <a href="#s-l+block-node(n,c)">s-l+block-node(n,block-out)</a><br />
| <a href="#s-l-empty-block">s-l-empty-block</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[221]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l-block-simple-entry(n)"></span>ns-l-block-simple-entry(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-block-simple-key(n)">ns-block-simple-key(n)</a><br />
<a href="#s-l+block-simple-value(n)">s-l+block-simple-value(n)</a></td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id935444"></span>

**Example 10.14. Simple Block Mapping Entries**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>plain key:° # empty value↓
&quot;quoted key&quot;:↓
- one # explicit next-line
- two # block value↓</code></pre>
<pre class="synopsis"><code>Legend:
  ns-block-simple-key(n)
  s-l+block-simple-value(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!map {
  ? !!str &quot;plain key&quot;
  : !!str &quot;&quot;,
  ? !!str &quot;quoted key\n&quot;
  : !!seq [
    !!str &quot;one&quot;,
    !!str &quot;two&quot;,
  ]
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

An <span id="id935541" class="indexterm"></span><span id="in-line mapping style/"></span>*in-line block mapping* begins with a same-line mapping entry, followed by optional additional normal block mapping entries, properly <span id="id935559" class="indexterm"></span>[indented](#indentation%20space/).

<table class="productionset" width="100%" data-cellpadding="5" data-summary="EBNF">
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><table class="productionset" data-border="0" width="99%" data-cellpadding="0" data-summary="EBNF productions">
<colgroup>
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
<col style="width: 20%" />
</colgroup>
<tbody>
<tr>
<td class="productioncounter" style="text-align: left;" data-valign="top">[222]</td>
<td class="productionlhs" style="text-align: right;" data-valign="top"><span id="ns-l-in-line-mapping(n)"></span>ns-l-in-line-mapping(n)</td>
<td class="productionseperator" style="text-align: center;" data-valign="top"><code>::=</code></td>
<td class="productionrhs" data-valign="top"><a href="#ns-l-block-map-entry(n)">ns-l-block-map-entry(n)</a><br />
( <a href="#s-indent(n)">s-indent(n)</a> <a href="#ns-l-block-map-entry(n)">ns-l-block-map-entry(n)</a> )*</td>
<td class="productioncomment" style="text-align: left;" data-valign="top"> </td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<div class="example">

<span id="id935604"></span>

**Example 10.15. In-Line Block Mappings**

<table class="simplelist" data-border="0" data-summary="Simple list">
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><pre class="programlisting"><code>- sun: yellow↓
- ? earth: blue↓
  : moon: white↓</code></pre>
<pre class="synopsis"><code>Legend:
  ns-l-in-line-mapping(n)</code></pre></td>
<td><pre class="programlisting"><code>%YAML 1.1
---
!!seq {
  !!map {
    ? !!str &quot;sun&quot;
    : !!str &quot;yellow&quot;,
  },
  !!map {
    ? !!map {
      ? !!str &quot;earth&quot;
      : !!str &quot;blue&quot;
    }
    : !!map {
      ? !!str &quot;moon&quot;
      : !!str &quot;white&quot;
    }
  }
}</code></pre></td>
</tr>
</tbody>
</table>

</div>

</div>

</div>

</div>

<div class="index">

<div class="titlepage">

<div>

<div>

## <span id="id935693"></span>Index

</div>

</div>

</div>

<div class="index">

<div class="indexdiv">

### Indicators

<span id="index-entry-! local  tag"></span>! local tag, <a href="#id861700" class="preferred">Tags</a>, [Tag Prefixes](#id896411), [Tag Handles](#id896876), [Node Tags](#id900262)

<span id="index-entry-! named handle"></span>! named handle, [Miscellaneous Characters](#id871856), <a href="#id896876" class="preferred">Tag Handles</a>, [Node Tags](#id900262)

<span id="index-entry-! non-specific  tag"></span>! non-specific tag, <a href="#id865585" class="preferred">Resolved</a>, [Node Tags](#id900262)

<span id="index-entry-! tag  indicator"></span>! tag indicator, [Tags](#id858600), [Indicator Characters](#id868988), [Tag Handles](#id896876), <a href="#id900262" class="preferred">Node Tags</a>

<span id="index-entry-" double-quoted  style"></span>" double-quoted style, [Indicator Characters](#id868988), [Escape Sequences](#id872840), <a href="#id904245" class="preferred">Double Quoted</a>, [Single Quoted](#id905860)

<span id="index-entry-#  comment"></span>\# comment, [Structures](#id857577), [Tags](#id861700), [Indicator Characters](#id868988), <a href="#id892353" class="preferred">Comments</a>, [Plain](#id907281), [Block Indentation Indicator](#id927035), [Literal](#id928909)

<span id="index-entry-% directive"></span>% directive, [Indicator Characters](#id868988), <a href="#id895217" class="preferred">Directives</a>

<span id="index-entry-%  escaping in URI"></span>% escaping in URI, <a href="#id871856" class="preferred">Miscellaneous Characters</a>, [Node Tags](#id900262)

<span id="index-entry-&  anchor"></span>& anchor, [Structures](#id857577), [Indicator Characters](#id868988), <a href="#id899912" class="preferred">Node Anchors</a>

<span id="index-entry-' reserved  indicator"></span>' reserved indicator, <a href="#id868988" class="preferred">Indicator Characters</a>

<span id="index-entry-' single-quoted  style"></span>' single-quoted style, [Indicator Characters](#id868988), <a href="#id905860" class="preferred">Single Quoted</a>

<span id="index-entry-*  alias"></span>\* alias, [Structures](#id857577), [Indicator Characters](#id868988), <a href="#id902561" class="preferred">Alias Nodes</a>

<span id="index-entry-+ keep chomping"></span>+ keep chomping, <a href="#id927557" class="preferred">Block Chomping Indicator</a>

<span id="index-entry-, end flow entry"></span>, end flow entry, [Indicator Characters](#id868988), [Plain](#id907281), <a href="#id930798" class="preferred">Collection Styles</a>, [Flow Sequences](#id931285), [Flow Mappings](#id933010)

<span id="index-entry-- block sequence  entry"></span>- block sequence entry, [Collections](#id857181), [Production Parameters](#id867808), [Indicator Characters](#id868988), [Indentation Spaces](#id891751), [Plain](#id907281), <a href="#id931893" class="preferred">Block Sequences</a>

<span id="index-entry-- strip  chomping"></span>- strip chomping, <a href="#id927557" class="preferred">Block Chomping Indicator</a>

<span id="index-entry-: mapping  value"></span>: mapping value, [Collections](#id857181), [Indicator Characters](#id868988), [Indentation Spaces](#id891751), [Plain](#id907281), <a href="#id933010" class="preferred">Flow Mappings</a>, [Block Mappings](#id934537)

<span id="index-entry-< … > verbatim tag"></span>\< … \> verbatim tag, <a href="#id900262" class="preferred">Node Tags</a>

<span id="index-entry-> folded  style"></span>\> folded style, [Scalars](#id858081), [Indicator Characters](#id868988), <a href="#id926836" class="preferred">Block Style Indicator</a>

<span id="index-entry-? mapping  key"></span>? mapping key, [Structures](#id857577), [Indicator Characters](#id868988), [Indentation Spaces](#id891751), [Plain](#id907281), <a href="#id933010" class="preferred">Flow Mappings</a>, [Block Mappings](#id934537)

<span id="index-entry-? non-specific tag"></span>? non-specific tag, <a href="#id865585" class="preferred">Resolved</a>, [Node Tags](#id900262)

<span id="index-entry-@ reserved  indicator"></span>@ reserved indicator, <a href="#id868988" class="preferred">Indicator Characters</a>

<span id="index-entry-[ start flow  sequence"></span>\[ start flow sequence, [Indicator Characters](#id868988), [Miscellaneous Characters](#id871856), [Plain](#id907281), <a href="#id931285" class="preferred">Flow Sequences</a>

<span id="index-entry-\ escaping in  double-quoted style"></span>\\ escaping in double-quoted style, <a href="#id872840" class="preferred">Escape Sequences</a>, [Double Quoted](#id904245), [Single Quoted](#id905860)

<span id="index-entry-] end flow  sequence"></span>\] end flow sequence, [Indicator Characters](#id868988), [Miscellaneous Characters](#id871856), [Plain](#id907281), <a href="#id931285" class="preferred">Flow Sequences</a>

<span id="index-entry-{ start flow  mapping"></span>{ start flow mapping, [Indicator Characters](#id868988), [Plain](#id907281), <a href="#id933010" class="preferred">Flow Mappings</a>

<span id="index-entry-| literal  style"></span>\| literal style, [Scalars](#id858081), [Indicator Characters](#id868988), <a href="#id926836" class="preferred">Block Style Indicator</a>

<span id="index-entry-} end flow  mapping"></span>} end flow mapping, [Indicator Characters](#id868988), [Plain](#id907281), <a href="#id933010" class="preferred">Flow Mappings</a>

</div>

<div class="indexdiv">

### A

<span id="index-entry-alias"></span>alias

information model, <a href="#id838426" data-xmlns="">Introduction</a>, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id859873" data-xmlns="">Serialize</a>, <a href="#id862929" data-xmlns="">Serialization Tree</a>, <a href="#id863390" class="preferred" data-xmlns="">Anchors and Aliases</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865423" data-xmlns="">Well-Formed and Identified</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id899912" data-xmlns="">Node Anchors</a>, <a href="#id902561" class="preferred" data-xmlns="">Alias Nodes</a>, <a href="#id902924" data-xmlns="">Flow Nodes</a>

<span id="index-entry-anchor"></span>anchor

information model, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id859873" data-xmlns="">Serialize</a>, <a href="#id862929" data-xmlns="">Serialization Tree</a>, <a href="#id863390" class="preferred" data-xmlns="">Anchors and Aliases</a>, <a href="#id865423" data-xmlns="">Well-Formed and Identified</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id899622" data-xmlns="">Nodes</a>, <a href="#id899912" class="preferred" data-xmlns="">Node Anchors</a>, <a href="#id900262" data-xmlns="">Node Tags</a>, <a href="#id902561" data-xmlns="">Alias Nodes</a>

<span id="index-entry-application"></span>application, [Introduction](#id838426), [Prior Art](#id838686), [Tags](#id858600), <a href="#id859109" class="preferred">Processing YAML Information</a>, [Processes](#id859458), [Represent](#id859497), [Serialize](#id859873), [Present](#id860109), [Information Models](#id860735), [Tags](#id861700), [Resolved](#id865585), [Available](#id867229), [Tag Prefixes](#id896411), [Tag Handles](#id896876), [Node Tags](#id900262), [Mapping Styles](#id932806)

<span id="index-entry-available tag"></span>available tag, <a href="#id867229" class="preferred">Available</a>

</div>

<div class="indexdiv">

### B

<span id="index-entry-block collection style"></span>block collection style

information model, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id930798" class="preferred" data-xmlns="">Collection Styles</a>, <a href="#id931893" data-xmlns="">Block Sequences</a>

<span id="index-entry-block mapping style"></span>block mapping style

information model, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id934537" class="preferred" data-xmlns="">Block Mappings</a>

<span id="index-entry-block scalar header"></span>block scalar header, <a href="#id926597" class="preferred">Block Scalar Header</a>, [Block Style Indicator](#id926836)

<span id="index-entry-block scalar style"></span>block scalar style

information model, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id871856" data-xmlns="">Miscellaneous Characters</a>, <a href="#id893482" data-xmlns="">Ignored Line Prefix</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id926597" data-xmlns="">Block Scalar Header</a>, <a href="#id927035" data-xmlns="">Block Indentation Indicator</a>, <a href="#id927557" data-xmlns="">Block Chomping Indicator</a>, <a href="#id928806" class="preferred" data-xmlns="">Block Scalar Styles</a>

<span id="index-entry-block  sequence style"></span>block sequence style

information model, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id931088" data-xmlns="">Sequence Styles</a>, <a href="#id931893" class="preferred" data-xmlns="">Block Sequences</a>, <a href="#id934537" data-xmlns="">Block Mappings</a>

<span id="index-entry-block style"></span>block style

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id894136" data-xmlns="">Line Folding</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id903421" class="preferred" data-xmlns="">Block Nodes</a>, <a href="#id931893" data-xmlns="">Block Sequences</a>

<span id="index-entry-block-in context"></span>block-in context, [Production Parameters](#id867808), <a href="#id931893" class="preferred">Block Sequences</a>

<span id="index-entry-block-out  context"></span>block-out context, [Production Parameters](#id867808), <a href="#id931893" class="preferred">Block Sequences</a>

<span id="index-entry-byte order mark"></span>byte order mark, <a href="#id868742" class="preferred">Character Encoding</a>, [Complete Stream](#id898785)

</div>

<div class="indexdiv">

### C

<span id="index-entry-canonical form"></span>canonical form, [Prior Art](#id838686), [Tags](#id861700), <a href="#id862121" class="preferred">Nodes Comparison</a>, [Scalar Formats](#id864510), [Loading Failure Points](#id864977)

<span id="index-entry-character encoding"></span>character encoding, <a href="#id868742" class="preferred">Character Encoding</a>, [Miscellaneous Characters](#id871856), [Complete Stream](#id898785)

<span id="index-entry-chomping"></span>chomping, [Production Parameters](#id867808), [Line Break Characters](#id871136), [Ignored Line Prefix](#id893482), [Line Folding](#id894136), <a href="#id927557" class="preferred">Block Chomping Indicator</a>, [Literal](#id928909), [Folded](#id929764)

<span id="index-entry-clip  chomping"></span>clip chomping, [Production Parameters](#id867808), <a href="#id927557" class="preferred">Block Chomping Indicator</a>

<span id="index-entry-collection"></span>collection

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id863390" data-xmlns="">Anchors and Aliases</a>, <a href="#id864687" data-xmlns="">Comments</a>, <a href="#id865585" data-xmlns="">Resolved</a>, <a href="#id866900" data-xmlns="">Recognized and Valid</a>

syntax, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id930798" class="preferred" data-xmlns="">Collection Styles</a>, <a href="#id931088" data-xmlns="">Sequence Styles</a>, <a href="#id931893" data-xmlns="">Block Sequences</a>

<span id="index-entry-comment"></span>comment

information model, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id860109" data-xmlns="">Present</a>, <a href="#id860557" data-xmlns="">Construct</a>, <a href="#id863676" data-xmlns="">Presentation Stream</a>, <a href="#id864687" class="preferred" data-xmlns="">Comments</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id892353" class="preferred" data-xmlns="">Comments</a>, <a href="#id893014" data-xmlns="">Separation Spaces</a>, <a href="#id893482" data-xmlns="">Ignored Line Prefix</a>, <a href="#id895217" data-xmlns="">Directives</a>, <a href="#id897596" data-xmlns="">Document Boundary Markers</a>, <a href="#id898031" data-xmlns="">Documents</a>, <a href="#id898785" data-xmlns="">Complete Stream</a>, <a href="#id903421" data-xmlns="">Block Nodes</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id926597" data-xmlns="">Block Scalar Header</a>, <a href="#id927557" data-xmlns="">Block Chomping Indicator</a>, <a href="#id931088" data-xmlns="">Sequence Styles</a>

<span id="index-entry-complete representation"></span>complete representation, <a href="#id864977" class="preferred">Loading Failure Points</a>, [Resolved](#id865585), [Recognized and Valid](#id866900), [Available](#id867229), [Node Tags](#id900262)

<span id="index-entry-completely  empty node"></span>completely empty node, [Documents](#id898031), <a href="#id902924" class="preferred">Flow Nodes</a>, [Block Nodes](#id903421), [Collection Styles](#id930798), [Flow Sequences](#id931285), [Block Sequences](#id931893), [Flow Mappings](#id933010), [Block Mappings](#id934537)

<span id="index-entry-compose"></span>compose, <a href="#id860452" class="preferred">Compose</a>, [Keys Order](#id863110), [Anchors and Aliases](#id863390), [Resolved](#id865585), [Recognized and Valid](#id866900), [Available](#id867229), [Node Anchors](#id899912), [Node Tags](#id900262)

<span id="index-entry-construct"></span>construct, [Processing YAML Information](#id859109), <a href="#id860557" class="preferred">Construct</a>, [Serialization Tree](#id862929), [Loading Failure Points](#id864977), [Recognized and Valid](#id866900), [Available](#id867229), [Mapping Styles](#id932806)

<span id="index-entry-content"></span>content

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id861700" data-xmlns="">Tags</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id863975" data-xmlns="">Node Styles</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865585" data-xmlns="">Resolved</a>, <a href="#id866900" data-xmlns="">Recognized and Valid</a>, <a href="#id871136" data-xmlns="">Line Break Characters</a>, <a href="#id872840" data-xmlns="">Escape Sequences</a>, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id892353" data-xmlns="">Comments</a>, <a href="#id893014" data-xmlns="">Separation Spaces</a>, <a href="#id893482" data-xmlns="">Ignored Line Prefix</a>, <a href="#id894136" data-xmlns="">Line Folding</a>, <a href="#id896876" data-xmlns="">Tag Handles</a>, <a href="#id898031" data-xmlns="">Documents</a>, <a href="#id899912" data-xmlns="">Node Anchors</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id930798" data-xmlns="">Collection Styles</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id899622" data-xmlns="">Nodes</a>, <a href="#id901659" class="preferred" data-xmlns="">Node Content</a>, <a href="#id902561" data-xmlns="">Alias Nodes</a>, <a href="#id902924" data-xmlns="">Flow Nodes</a>, <a href="#id903421" data-xmlns="">Block Nodes</a>, <a href="#id904245" data-xmlns="">Double Quoted</a>, <a href="#id905860" data-xmlns="">Single Quoted</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id926597" data-xmlns="">Block Scalar Header</a>, <a href="#id927035" data-xmlns="">Block Indentation Indicator</a>, <a href="#id927557" data-xmlns="">Block Chomping Indicator</a>, <a href="#id928806" data-xmlns="">Block Scalar Styles</a>, <a href="#id929764" data-xmlns="">Folded</a>

<span id="index-entry-context"></span>context, <a href="#id867808" class="preferred">Production Parameters</a>, [Plain](#id907281)

</div>

<div class="indexdiv">

### D

<span id="index-entry-directive"></span>directive

information model, <a href="#id860109" data-xmlns="">Present</a>, <a href="#id860557" data-xmlns="">Construct</a>, <a href="#id863676" data-xmlns="">Presentation Stream</a>, <a href="#id864824" class="preferred" data-xmlns="">Directives</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id895107" data-xmlns="">YAML Character Stream</a>, <a href="#id895217" class="preferred" data-xmlns="">Directives</a>, <a href="#id898031" data-xmlns="">Documents</a>, <a href="#id898785" data-xmlns="">Complete Stream</a>

<span id="index-entry-document"></span>document

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id863676" class="preferred" data-xmlns="">Presentation Stream</a>, <a href="#id863975" data-xmlns="">Node Styles</a>, <a href="#id864824" data-xmlns="">Directives</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865585" data-xmlns="">Resolved</a>, <a href="#id866900" data-xmlns="">Recognized and Valid</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id895107" data-xmlns="">YAML Character Stream</a>, <a href="#id895631" data-xmlns="">“YAML” Directive</a>, <a href="#id896411" data-xmlns="">Tag Prefixes</a>, <a href="#id896876" data-xmlns="">Tag Handles</a>, <a href="#id897596" data-xmlns="">Document Boundary Markers</a>, <a href="#id898031" class="preferred" data-xmlns="">Documents</a>, <a href="#id898785" data-xmlns="">Complete Stream</a>, <a href="#id902561" data-xmlns="">Alias Nodes</a>

<span id="index-entry-document boundary  marker"></span>document boundary marker, [Structures](#id857577), [Presentation Stream](#id863676), [YAML Character Stream](#id895107), <a href="#id897596" class="preferred">Document Boundary Markers</a>, [Documents](#id898031), [Complete Stream](#id898785), [Plain](#id907281)

<span id="index-entry-double-quoted style"></span>double-quoted style

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867381" data-xmlns="">Productions Conventions</a>, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id872840" data-xmlns="">Escape Sequences</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id904245" class="preferred" data-xmlns="">Double Quoted</a>, <a href="#id905860" data-xmlns="">Single Quoted</a>

<span id="index-entry-dump"></span>dump, <a href="#id859109" class="preferred">Processing YAML Information</a>

</div>

<div class="indexdiv">

### E

<span id="index-entry-empty line"></span>empty line, [Prior Art](#id838686), [Scalars](#id858081), [Comments](#id892353), <a href="#id893482" class="preferred">Ignored Line Prefix</a>, [Line Folding](#id894136), [Flow Scalar Styles](#id904158), [Plain](#id907281), [Block Indentation Indicator](#id927035), [Block Chomping Indicator](#id927557), [Literal](#id928909), [Folded](#id929764)

<span id="index-entry-equality"></span>equality, [Represent](#id859497), [Representation Graph](#id861060), [Nodes](#id861435), [Tags](#id861700), <a href="#id862121" class="preferred">Nodes Comparison</a>, [Scalar Formats](#id864510), [Loading Failure Points](#id864977), [Recognized and Valid](#id866900), [Mapping Styles](#id932806)

<span id="index-entry-escaped  (ignored) line break"></span>escaped (ignored) line break, [Line Break Characters](#id871136), <a href="#id904245" class="preferred">Double Quoted</a>

<span id="index-entry-escaping in  double-quoted style"></span>escaping in double-quoted style, [Prior Art](#id838686), [Scalars](#id858081), [Character Set](#id868524), [Miscellaneous Characters](#id871856), <a href="#id872840" class="preferred">Escape Sequences</a>, [Double Quoted](#id904245), [Literal](#id928909)

<span id="index-entry-escaping in single-quoted  style"></span>escaping in single-quoted style, <a href="#id905860" class="preferred">Single Quoted</a>

<span id="index-entry-escaping in URI"></span>escaping in URI, [Tags](#id861700), <a href="#id871856" class="preferred">Miscellaneous Characters</a>, [Node Tags](#id900262)

<span id="index-entry-explicit document"></span>explicit document, <a href="#id898031" class="preferred">Documents</a>, [Complete Stream](#id898785)

<span id="index-entry-explicit key"></span>explicit key, <a href="#id933010" class="preferred">Flow Mappings</a>, [Block Mappings](#id934537)

<span id="index-entry-explicit value"></span>explicit value, <a href="#id933010" class="preferred">Flow Mappings</a>, [Block Mappings](#id934537)

</div>

<div class="indexdiv">

### F

<span id="index-entry-flow collection style"></span>flow collection style

information model, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867381" data-xmlns="">Productions Conventions</a>, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id930798" class="preferred" data-xmlns="">Collection Styles</a>

<span id="index-entry-flow mapping style"></span>flow mapping style

information model, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id933010" class="preferred" data-xmlns="">Flow Mappings</a>

<span id="index-entry-flow scalar style"></span>flow scalar style

information model, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id894136" data-xmlns="">Line Folding</a>, <a href="#id898031" data-xmlns="">Documents</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id904158" class="preferred" data-xmlns="">Flow Scalar Styles</a>, <a href="#id907281" data-xmlns="">Plain</a>

<span id="index-entry-flow sequence style"></span>flow sequence style

information model, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id931088" data-xmlns="">Sequence Styles</a>, <a href="#id931285" class="preferred" data-xmlns="">Flow Sequences</a>, <a href="#id933010" data-xmlns="">Flow Mappings</a>

<span id="index-entry-flow style"></span>flow style

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id894136" data-xmlns="">Line Folding</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id902924" class="preferred" data-xmlns="">Flow Nodes</a>, <a href="#id903421" data-xmlns="">Block Nodes</a>, <a href="#id931285" data-xmlns="">Flow Sequences</a>

<span id="index-entry-flow-in  context"></span>flow-in context, [Production Parameters](#id867808), <a href="#id907281" class="preferred">Plain</a>, [Collection Styles](#id930798)

<span id="index-entry-flow-key context"></span>flow-key context, [Production Parameters](#id867808), [Collection Styles](#id930798), <a href="#id933010" class="preferred">Flow Mappings</a>

<span id="index-entry-flow-out context"></span>flow-out context, [Production Parameters](#id867808), <a href="#id907281" class="preferred">Plain</a>, [Collection Styles](#id930798)

<span id="index-entry-folded  style"></span>folded style

information model, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id894136" data-xmlns="">Line Folding</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id926836" data-xmlns="">Block Style Indicator</a>, <a href="#id928806" data-xmlns="">Block Scalar Styles</a>, <a href="#id929764" class="preferred" data-xmlns="">Folded</a>

<span id="index-entry-format"></span>format, [Present](#id860109), [Construct](#id860557), [Tags](#id861700), [Nodes Comparison](#id862121), [Presentation Stream](#id863676), <a href="#id864510" class="preferred">Scalar Formats</a>

</div>

<div class="indexdiv">

### G

<span id="index-entry-generic line break"></span>generic line break, <a href="#id871136" class="preferred">Line Break Characters</a>, [Escape Sequences](#id872840), [Line Folding](#id894136)

<span id="index-entry-global tag"></span>global tag, [Prior Art](#id838686), [Tags](#id858600), [Represent](#id859497), <a href="#id861700" class="preferred">Tags</a>, [Resolved](#id865585), [Tag Prefixes](#id896411), [Tag Handles](#id896876), [Node Tags](#id900262)

</div>

<div class="indexdiv">

### I

<span id="index-entry-identified"></span>identified, [Structures](#id857577), <a href="#id863390" class="preferred">Anchors and Aliases</a>, [Well-Formed and Identified](#id865423)

<span id="index-entry-identity"></span>identity, <a href="#id862121" class="preferred">Nodes Comparison</a>

<span id="index-entry-ignored line  prefix"></span>ignored line prefix, <a href="#id893482" class="preferred">Ignored Line Prefix</a>, [Double Quoted](#id904245), [Single Quoted](#id905860), [Plain](#id907281)

<span id="index-entry-ill-formed  stream"></span>ill-formed stream, [Parse](#id860341), [Loading Failure Points](#id864977), <a href="#id865423" class="preferred">Well-Formed and Identified</a>

<span id="index-entry-implicit document"></span>implicit document, <a href="#id898031" class="preferred">Documents</a>, [Complete Stream](#id898785)

<span id="index-entry-in-line mapping style"></span>in-line mapping style, <a href="#id934537" class="preferred">Block Mappings</a>

<span id="index-entry-in-line sequence style"></span>in-line sequence style, <a href="#id931893" class="preferred">Block Sequences</a>

<span id="index-entry-in-line style"></span>in-line style

information model, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id930798" data-xmlns="">Collection Styles</a>, <a href="#id931088" data-xmlns="">Sequence Styles</a>, <a href="#id931893" class="preferred" data-xmlns="">Block Sequences</a>, <a href="#id934537" data-xmlns="">Block Mappings</a>

<span id="index-entry-indentation  indicator"></span>indentation indicator, <a href="#id927035" class="preferred">Block Indentation Indicator</a>, [Folded](#id929764)

<span id="index-entry-indentation  space"></span>indentation space, [Introduction](#id838426), [Prior Art](#id838686), [Collections](#id857181), [Present](#id860109), [Construct](#id860557), [Information Models](#id860735), [Node Styles](#id863975), [Resolved](#id865585), [Production Prefixes](#id867562), [Production Parameters](#id867808), [Miscellaneous Characters](#id871856), <a href="#id891751" class="preferred">Indentation Spaces</a>, [Comments](#id892353), [Separation Spaces](#id893014), [Ignored Line Prefix](#id893482), [Line Folding](#id894136), [Directives](#id895217), [Documents](#id898031), [Node Content](#id901659), [Double Quoted](#id904245), [Single Quoted](#id905860), [Plain](#id907281), [Block Indentation Indicator](#id927035), [Block Chomping Indicator](#id927557), [Block Scalar Styles](#id928806), [Literal](#id928909), [Folded](#id929764), [Block Sequences](#id931893), [Block Mappings](#id934537)

<span id="index-entry-indicator"></span>indicator, [Prior Art](#id838686), [Collections](#id857181), [Node Styles](#id863975), [Production Parameters](#id867808), <a href="#id868988" class="preferred">Indicator Characters</a>, [Indentation Spaces](#id891751), [Line Folding](#id894136), [Node Content](#id901659), [Flow Nodes](#id902924), [Block Nodes](#id903421), [Plain](#id907281), [Block Scalar Header](#id926597), [Literal](#id928909)

<span id="index-entry-invalid content"></span>invalid content, [Loading Failure Points](#id864977), <a href="#id866900" class="preferred">Recognized and Valid</a>

</div>

<div class="indexdiv">

### K

<span id="index-entry-keep  chomping"></span>keep chomping, [Production Parameters](#id867808), <a href="#id927557" class="preferred">Block Chomping Indicator</a>

<span id="index-entry-key"></span>key

information model, <a href="#id838426" data-xmlns="">Introduction</a>, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id859873" data-xmlns="">Serialize</a>, <a href="#id860735" data-xmlns="">Information Models</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id862929" data-xmlns="">Serialization Tree</a>, <a href="#id863110" data-xmlns="">Keys Order</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id931285" data-xmlns="">Flow Sequences</a>, <a href="#id932806" class="preferred" data-xmlns="">Mapping Styles</a>

<span id="index-entry-key order"></span>key order, [Serialize](#id859873), [Construct](#id860557), [Information Models](#id860735), [Serialization Tree](#id862929), <a href="#id863110" class="preferred">Keys Order</a>, [Mapping Styles](#id932806)

<span id="index-entry-kind"></span>kind, [Represent](#id859497), [Representation Graph](#id861060), <a href="#id861435" class="preferred">Nodes</a>, [Tags](#id861700), [Nodes Comparison](#id862121), [Node Styles](#id863975), [Resolved](#id865585), [Node Content](#id901659), [Collection Styles](#id930798)

</div>

<div class="indexdiv">

### L

<span id="index-entry-line break character"></span>line break character, [Prior Art](#id838686), [Scalars](#id858081), [Production Prefixes](#id867562), [Production Parameters](#id867808), [Character Set](#id868524), <a href="#id871136" class="preferred">Line Break Characters</a>, [Miscellaneous Characters](#id871856), [Indentation Spaces](#id891751), [Ignored Line Prefix](#id893482), [Line Folding](#id894136), [Block Nodes](#id903421), [Double Quoted](#id904245), [Single Quoted](#id905860), [Plain](#id907281), [Block Scalar Header](#id926597), [Block Chomping Indicator](#id927557), [Literal](#id928909), [Folded](#id929764)

<span id="index-entry-line break normalization"></span>line break normalization, <a href="#id871136" class="preferred">Line Break Characters</a>, [Literal](#id928909)

<span id="index-entry-line folding"></span>line folding, [Prior Art](#id838686), [Scalars](#id858081), <a href="#id894136" class="preferred">Line Folding</a>, [Flow Scalar Styles](#id904158), [Double Quoted](#id904245), [Single Quoted](#id905860), [Plain](#id907281), [Block Chomping Indicator](#id927557), [Literal](#id928909), [Folded](#id929764)

<span id="index-entry-literal style"></span>literal style

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id926836" data-xmlns="">Block Style Indicator</a>, <a href="#id928806" data-xmlns="">Block Scalar Styles</a>, <a href="#id928909" class="preferred" data-xmlns="">Literal</a>, <a href="#id929764" data-xmlns="">Folded</a>

<span id="index-entry-load"></span>load, <a href="#id859109" class="preferred">Processing YAML Information</a>, [Loading Failure Points](#id864977)

<span id="index-entry-load failure point"></span>load failure point, [Compose](#id860452), <a href="#id864977" class="preferred">Loading Failure Points</a>

<span id="index-entry-local tag"></span>local tag, [Tags](#id858600), [Represent](#id859497), <a href="#id861700" class="preferred">Tags</a>, [Resolved](#id865585), [Tag Prefixes](#id896411), [Tag Handles](#id896876), [Node Tags](#id900262)

</div>

<div class="indexdiv">

### M

<span id="index-entry-mapping"></span>mapping

information model, <a href="#id838426" data-xmlns="">Introduction</a>, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id861700" data-xmlns="">Tags</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id863110" data-xmlns="">Keys Order</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id930798" data-xmlns="">Collection Styles</a>, <a href="#id931285" data-xmlns="">Flow Sequences</a>, <a href="#id932806" class="preferred" data-xmlns="">Mapping Styles</a>

<span id="index-entry-may"></span>may, <a href="#id856957" class="preferred">Terminology</a>

<span id="index-entry-“more indented” line"></span>“more indented” line, [Scalars](#id858081), [Line Folding](#id894136), <a href="#id929764" class="preferred">Folded</a>

<span id="index-entry-must"></span>must, <a href="#id856957" class="preferred">Terminology</a>

</div>

<div class="indexdiv">

### N

<span id="index-entry-named  tag handle"></span>named tag handle, [Miscellaneous Characters](#id871856), <a href="#id896876" class="preferred">Tag Handles</a>, [Node Tags](#id900262)

<span id="index-entry-need not"></span>need not, <a href="#id856957" class="preferred">Terminology</a>

<span id="index-entry-node"></span>node

information model, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id859873" data-xmlns="">Serialize</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id861700" data-xmlns="">Tags</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id862929" data-xmlns="">Serialization Tree</a>, <a href="#id863110" data-xmlns="">Keys Order</a>, <a href="#id863390" data-xmlns="">Anchors and Aliases</a>, <a href="#id863676" data-xmlns="">Presentation Stream</a>, <a href="#id863975" data-xmlns="">Node Styles</a>, <a href="#id864687" data-xmlns="">Comments</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865423" data-xmlns="">Well-Formed and Identified</a>, <a href="#id865585" data-xmlns="">Resolved</a>, <a href="#id866900" data-xmlns="">Recognized and Valid</a>

syntax, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id898031" data-xmlns="">Documents</a>, <a href="#id899622" class="preferred" data-xmlns="">Nodes</a>, <a href="#id899912" data-xmlns="">Node Anchors</a>, <a href="#id900262" data-xmlns="">Node Tags</a>, <a href="#id902561" data-xmlns="">Alias Nodes</a>, <a href="#id902924" data-xmlns="">Flow Nodes</a>, <a href="#id931088" data-xmlns="">Sequence Styles</a>, <a href="#id931893" data-xmlns="">Block Sequences</a>

<span id="index-entry-node  property"></span>node property, [Documents](#id898031), <a href="#id899622" class="preferred">Nodes</a>, [Alias Nodes](#id902561), [Flow Nodes](#id902924), [Block Nodes](#id903421), [Flow Sequences](#id931285), [Block Sequences](#id931893), [Flow Mappings](#id933010)

<span id="index-entry-non-specific tag"></span>non-specific tag, [Tags](#id858600), [Present](#id860109), [Loading Failure Points](#id864977), <a href="#id865585" class="preferred">Resolved</a>, [Productions Conventions](#id867381), [Node Tags](#id900262), [Scalar Styles](#id903915)

</div>

<div class="indexdiv">

### O

<span id="index-entry-optional"></span>optional, <a href="#id856957" class="preferred">Terminology</a>

</div>

<div class="indexdiv">

### P

<span id="index-entry-parse"></span>parse, <a href="#id860341" class="preferred">Parse</a>, [Presentation Stream](#id863676), [Resolved](#id865585), [Line Break Characters](#id871136), [Escape Sequences](#id872840), [Tag Handles](#id896876), [Complete Stream](#id898785), [Node Tags](#id900262)

<span id="index-entry-partial representation"></span>partial representation, <a href="#id864977" class="preferred">Loading Failure Points</a>, [Resolved](#id865585), [Recognized and Valid](#id866900)

<span id="index-entry-plain  style"></span>plain style

information model, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id898031" data-xmlns="">Documents</a>, <a href="#id900262" data-xmlns="">Node Tags</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id902924" data-xmlns="">Flow Nodes</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id907281" class="preferred" data-xmlns="">Plain</a>

<span id="index-entry-present"></span>present, [Processing YAML Information](#id859109), [Represent](#id859497), <a href="#id860109" class="preferred">Present</a>, [Parse](#id860341), [Nodes](#id861435), [Nodes Comparison](#id862121), [Presentation Stream](#id863676), [Scalar Formats](#id864510), [Resolved](#id865585), [Production Parameters](#id867808), [Character Set](#id868524), [Line Break Characters](#id871136), [Escape Sequences](#id872840), [Line Folding](#id894136), [YAML Character Stream](#id895107), [Documents](#id898031), [Node Tags](#id900262), [Node Content](#id901659), [Alias Nodes](#id902561), [Flow Nodes](#id902924), [Flow Scalar Styles](#id904158), [Plain](#id907281), [Block Chomping Indicator](#id927557), [Sequence Styles](#id931088), [Block Sequences](#id931893), [Mapping Styles](#id932806), [Block Mappings](#id934537)

<span id="index-entry-presentation"></span>presentation, [Processing YAML Information](#id859109), [Information Models](#id860735), <a href="#id863676" class="preferred">Presentation Stream</a>, [Documents](#id898031), [Node Tags](#id900262)

<span id="index-entry-presentation  detail"></span>presentation detail, <a href="#id860109" class="preferred">Present</a>, [Parse](#id860341), [Construct](#id860557), [Information Models](#id860735), [Presentation Stream](#id863676), [Node Styles](#id863975), [Scalar Formats](#id864510), [Comments](#id864687), [Directives](#id864824), [Resolved](#id865585), [Line Break Characters](#id871136), [Escape Sequences](#id872840), [Indentation Spaces](#id891751), [Comments](#id892353), [Separation Spaces](#id893014), [Ignored Line Prefix](#id893482), [Line Folding](#id894136), [Directives](#id895217), [Tag Handles](#id896876), [Document Boundary Markers](#id897596), [Documents](#id898031), [Scalar Styles](#id903915), [Block Chomping Indicator](#id927557), [Collection Styles](#id930798)

<span id="index-entry-primary tag handle"></span>primary tag handle, <a href="#id896876" class="preferred">Tag Handles</a>, [Node Tags](#id900262)

<span id="index-entry-printable character"></span>printable character, [Introduction](#id838426), [Prior Art](#id838686), <a href="#id868524" class="preferred">Character Set</a>, [Escape Sequences](#id872840), [Single Quoted](#id905860), [Plain](#id907281), [Literal](#id928909)

<span id="index-entry-processor"></span>processor, [Terminology](#id856957), <a href="#id859109" class="preferred">Processing YAML Information</a>, [Processes](#id859458), [Serialize](#id859873), [Present](#id860109), [Nodes Comparison](#id862121), [Presentation Stream](#id863676), [Directives](#id864824), [Well-Formed and Identified](#id865423), [Resolved](#id865585), [Recognized and Valid](#id866900), [Available](#id867229), [Character Set](#id868524), [Character Encoding](#id868742), [Line Break Characters](#id871136), [Directives](#id895217), [“YAML” Directive](#id895631), [Tag Handles](#id896876), [Document Boundary Markers](#id897596), [Node Anchors](#id899912), [Node Tags](#id900262), [Block Indentation Indicator](#id927035), [Mapping Styles](#id932806), [Flow Mappings](#id933010), [Block Mappings](#id934537)

</div>

<div class="indexdiv">

### Q

<span id="index-entry-quoted style"></span>quoted style

information model, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id871856" data-xmlns="">Miscellaneous Characters</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id903915" class="preferred" data-xmlns="">Scalar Styles</a>

</div>

<div class="indexdiv">

### R

<span id="index-entry-recognized tag"></span>recognized tag, <a href="#id866900" class="preferred">Recognized and Valid</a>

<span id="index-entry-recommended"></span>recommended, <a href="#id856957" class="preferred">Terminology</a>

<span id="index-entry-represent"></span>represent, [Introduction](#id838426), [Prior Art](#id838686), <a href="#id859497" class="preferred">Represent</a>, [Tags](#id861700), [Nodes Comparison](#id862121), [Keys Order](#id863110)

<span id="index-entry-representation"></span>representation, [Processing YAML Information](#id859109), [Serialize](#id859873), [Compose](#id860452), [Construct](#id860557), [Information Models](#id860735), <a href="#id861060" class="preferred">Representation Graph</a>, [Nodes Comparison](#id862121), [Serialization Tree](#id862929), [Keys Order](#id863110), [Anchors and Aliases](#id863390), [Presentation Stream](#id863676), [Node Styles](#id863975), [Scalar Formats](#id864510), [Comments](#id864687), [Directives](#id864824), [Available](#id867229), [Comments](#id892353), [Directives](#id895217), [Node Anchors](#id899912), [Node Tags](#id900262), [Block Chomping Indicator](#id927557), [Mapping Styles](#id932806)

<span id="index-entry-required"></span>required, <a href="#id856957" class="preferred">Terminology</a>

<span id="index-entry-reserved  directive"></span>reserved directive, [Directives](#id864824), <a href="#id895217" class="preferred">Directives</a>

<span id="index-entry-reserved indicator"></span>reserved indicator, <a href="#id868988" class="preferred">Indicator Characters</a>

<span id="index-entry-root node"></span>root node, <a href="#id861060" class="preferred">Representation Graph</a>, [Resolved](#id865585), [YAML Character Stream](#id895107), [Documents](#id898031)

</div>

<div class="indexdiv">

### S

<span id="index-entry-scalar"></span>scalar

information model, <a href="#id838426" data-xmlns="">Introduction</a>, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id861700" data-xmlns="">Tags</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id863975" data-xmlns="">Node Styles</a>, <a href="#id864510" data-xmlns="">Scalar Formats</a>, <a href="#id864687" data-xmlns="">Comments</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865585" data-xmlns="">Resolved</a>, <a href="#id866900" data-xmlns="">Recognized and Valid</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id871136" data-xmlns="">Line Break Characters</a>, <a href="#id871856" data-xmlns="">Miscellaneous Characters</a>, <a href="#id872840" data-xmlns="">Escape Sequences</a>, <a href="#id892353" data-xmlns="">Comments</a>, <a href="#id893014" data-xmlns="">Separation Spaces</a>, <a href="#id893482" data-xmlns="">Ignored Line Prefix</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id903915" class="preferred" data-xmlns="">Scalar Styles</a>, <a href="#id904245" data-xmlns="">Double Quoted</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id927557" data-xmlns="">Block Chomping Indicator</a>, <a href="#id928909" data-xmlns="">Literal</a>

<span id="index-entry-secondary tag handle"></span>secondary tag handle, <a href="#id896876" class="preferred">Tag Handles</a>

<span id="index-entry-separation space"></span>separation space, [Miscellaneous Characters](#id871856), [Comments](#id892353), <a href="#id893014" class="preferred">Separation Spaces</a>, [Block Sequences](#id931893), [Flow Mappings](#id933010), [Block Mappings](#id934537)

<span id="index-entry-sequence"></span>sequence

information model, <a href="#id838426" data-xmlns="">Introduction</a>, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id861700" data-xmlns="">Tags</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id863110" data-xmlns="">Keys Order</a>

syntax, <a href="#id930798" data-xmlns="">Collection Styles</a>, <a href="#id931088" class="preferred" data-xmlns="">Sequence Styles</a>

<span id="index-entry-serialization"></span>serialization, [Processing YAML Information](#id859109), [Serialize](#id859873), [Present](#id860109), [Parse](#id860341), [Compose](#id860452), [Construct](#id860557), [Information Models](#id860735), <a href="#id862929" class="preferred">Serialization Tree</a>, [Anchors and Aliases](#id863390), [Presentation Stream](#id863676), [Node Styles](#id863975), [Scalar Formats](#id864510), [Comments](#id864687), [Directives](#id864824), [Comments](#id892353), [Directives](#id895217), [Node Anchors](#id899912), [Node Tags](#id900262), [Block Chomping Indicator](#id927557), [Mapping Styles](#id932806)

<span id="index-entry-serialization  detail"></span>serialization detail, <a href="#id859873" class="preferred">Serialize</a>, [Compose](#id860452), [Information Models](#id860735), [Keys Order](#id863110), [Anchors and Aliases](#id863390), [Node Anchors](#id899912), [Mapping Styles](#id932806)

<span id="index-entry-serialize"></span>serialize, [Prior Art](#id838686), <a href="#id859873" class="preferred">Serialize</a>, [Compose](#id860452), [Keys Order](#id863110), [Anchors and Aliases](#id863390), [Alias Nodes](#id902561)

<span id="index-entry-shall"></span>shall, <a href="#id856957" class="preferred">Terminology</a>

<span id="index-entry-should"></span>should, <a href="#id856957" class="preferred">Terminology</a>

<span id="index-entry-simple  key"></span>simple key, [Production Parameters](#id867808), [Separation Spaces](#id893014), [Flow Scalar Styles](#id904158), [Double Quoted](#id904245), [Single Quoted](#id905860), [Plain](#id907281), [Collection Styles](#id930798), <a href="#id933010" class="preferred">Flow Mappings</a>, [Block Mappings](#id934537)

<span id="index-entry-single pair style"></span>single pair style

information model, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id931088" data-xmlns="">Sequence Styles</a>, <a href="#id931285" data-xmlns="">Flow Sequences</a>, <a href="#id933010" class="preferred" data-xmlns="">Flow Mappings</a>

<span id="index-entry-single-quoted style"></span>single-quoted style

information model, <a href="#id858081" data-xmlns="">Scalars</a>, <a href="#id863975" class="preferred" data-xmlns="">Node Styles</a>

syntax, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id901659" data-xmlns="">Node Content</a>, <a href="#id903915" data-xmlns="">Scalar Styles</a>, <a href="#id905860" class="preferred" data-xmlns="">Single Quoted</a>, <a href="#id907281" data-xmlns="">Plain</a>

<span id="index-entry-specific line break"></span>specific line break, <a href="#id871136" class="preferred">Line Break Characters</a>, [Escape Sequences](#id872840), [Line Folding](#id894136)

<span id="index-entry-specific  tag"></span>specific tag, <a href="#id865585" class="preferred">Resolved</a>, [Node Tags](#id900262)

<span id="index-entry-stream"></span>stream

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id859109" data-xmlns="">Processing YAML Information</a>, <a href="#id860109" data-xmlns="">Present</a>, <a href="#id860341" data-xmlns="">Parse</a>, <a href="#id863676" class="preferred" data-xmlns="">Presentation Stream</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865423" data-xmlns="">Well-Formed and Identified</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id867381" data-xmlns="">Productions Conventions</a>, <a href="#id867808" data-xmlns="">Production Parameters</a>, <a href="#id868524" data-xmlns="">Character Set</a>, <a href="#id868742" data-xmlns="">Character Encoding</a>, <a href="#id891751" data-xmlns="">Indentation Spaces</a>, <a href="#id895107" data-xmlns="">YAML Character Stream</a>, <a href="#id896411" data-xmlns="">Tag Prefixes</a>, <a href="#id897596" data-xmlns="">Document Boundary Markers</a>, <a href="#id898785" class="preferred" data-xmlns="">Complete Stream</a>, <a href="#id899622" data-xmlns="">Nodes</a>, <a href="#id902924" data-xmlns="">Flow Nodes</a>, <a href="#id928806" data-xmlns="">Block Scalar Styles</a>, <a href="#id932806" data-xmlns="">Mapping Styles</a>, <a href="#id933010" data-xmlns="">Flow Mappings</a>, <a href="#id934537" data-xmlns="">Block Mappings</a>

<span id="index-entry-strip chomping"></span>strip chomping, [Production Parameters](#id867808), <a href="#id927557" class="preferred">Block Chomping Indicator</a>

<span id="index-entry-style"></span>style, [Present](#id860109), [Construct](#id860557), [Information Models](#id860735), [Presentation Stream](#id863676), <a href="#id863975" class="preferred">Node Styles</a>, [Scalar Formats](#id864510), [Resolved](#id865585), [Documents](#id898031)

</div>

<div class="indexdiv">

### T

<span id="index-entry-tab"></span>tab, [Prior Art](#id838686), [Character Set](#id868524), <a href="#id871856" class="preferred">Miscellaneous Characters</a>, [Escape Sequences](#id872840), [Indentation Spaces](#id891751), [Comments](#id892353), [Separation Spaces](#id893014), [Ignored Line Prefix](#id893482), [Double Quoted](#id904245), [Single Quoted](#id905860), [Plain](#id907281), [Block Indentation Indicator](#id927035), [Literal](#id928909), [Folded](#id929764)

<span id="index-entry-tag"></span>tag

information model, <a href="#id838686" data-xmlns="">Prior Art</a>, <a href="#id858600" data-xmlns="">Tags</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id860109" data-xmlns="">Present</a>, <a href="#id861060" data-xmlns="">Representation Graph</a>, <a href="#id861435" data-xmlns="">Nodes</a>, <a href="#id861700" class="preferred" data-xmlns="">Tags</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id864510" data-xmlns="">Scalar Formats</a>, <a href="#id864977" data-xmlns="">Loading Failure Points</a>, <a href="#id865585" data-xmlns="">Resolved</a>, <a href="#id866900" data-xmlns="">Recognized and Valid</a>, <a href="#id867229" data-xmlns="">Available</a>, <a href="#id896876" data-xmlns="">Tag Handles</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id871856" data-xmlns="">Miscellaneous Characters</a>, <a href="#id896044" data-xmlns="">“TAG” Directive</a>, <a href="#id896411" data-xmlns="">Tag Prefixes</a>, <a href="#id899622" data-xmlns="">Nodes</a>, <a href="#id900262" class="preferred" data-xmlns="">Node Tags</a>

<span id="index-entry-TAG directive"></span>TAG directive, [Tags](#id861700), [Directives](#id864824), [Directives](#id895217), <a href="#id896044" class="preferred">“TAG” Directive</a>, [Node Tags](#id900262)

<span id="index-entry-tag handle"></span>tag handle, [Tags](#id858600), [Present](#id860109), [“TAG” Directive](#id896044), [Tag Prefixes](#id896411), <a href="#id896876" class="preferred">Tag Handles</a>, [Node Tags](#id900262)

<span id="index-entry-tag  prefix"></span>tag prefix, [“TAG” Directive](#id896044), <a href="#id896411" class="preferred">Tag Prefixes</a>, [Node Tags](#id900262)

<span id="index-entry-tag resolution"></span>tag resolution, [Tags](#id861700), [Loading Failure Points](#id864977), <a href="#id865585" class="preferred">Resolved</a>, [Productions Conventions](#id867381), [Node Tags](#id900262), [Scalar Styles](#id903915)

<span id="index-entry-tag shorthand"></span>tag shorthand, [Tags](#id858600), [Productions Conventions](#id867381), [Miscellaneous Characters](#id871856), [“TAG” Directive](#id896044), [Tag Prefixes](#id896411), [Tag Handles](#id896876), <a href="#id900262" class="preferred">Node Tags</a>

</div>

<div class="indexdiv">

### U

<span id="index-entry-unavailable  tag"></span>unavailable tag, [Construct](#id860557), [Loading Failure Points](#id864977), <a href="#id867229" class="preferred">Available</a>

<span id="index-entry-unidentified alias"></span>unidentified alias, [Loading Failure Points](#id864977), <a href="#id865423" class="preferred">Well-Formed and Identified</a>

<span id="index-entry-unrecognized tag"></span>unrecognized tag, [Loading Failure Points](#id864977), <a href="#id866900" class="preferred">Recognized and Valid</a>

<span id="index-entry-unresolved tag"></span>unresolved tag, [Loading Failure Points](#id864977), <a href="#id865585" class="preferred">Resolved</a>

</div>

<div class="indexdiv">

### V

<span id="index-entry-valid content"></span>valid content, <a href="#id866900" class="preferred">Recognized and Valid</a>

<span id="index-entry-value"></span>value

information model, <a href="#id838426" data-xmlns="">Introduction</a>, <a href="#id857181" data-xmlns="">Collections</a>, <a href="#id857577" data-xmlns="">Structures</a>, <a href="#id859497" data-xmlns="">Represent</a>, <a href="#id861435" class="preferred" data-xmlns="">Nodes</a>, <a href="#id862121" data-xmlns="">Nodes Comparison</a>, <a href="#id863110" data-xmlns="">Keys Order</a>, <a href="#id865585" data-xmlns="">Resolved</a>

syntax, <a href="#id868988" data-xmlns="">Indicator Characters</a>, <a href="#id907281" data-xmlns="">Plain</a>, <a href="#id932806" class="preferred" data-xmlns="">Mapping Styles</a>

<span id="index-entry-verbatim  tag"></span>verbatim tag, [Productions Conventions](#id867381), <a href="#id900262" class="preferred">Node Tags</a>

</div>

<div class="indexdiv">

### W

<span id="index-entry-well-formed stream"></span>well-formed stream, <a href="#id865423" class="preferred">Well-Formed and Identified</a>

<span id="index-entry-white space"></span>white space, <a href="#id871856" class="preferred">Miscellaneous Characters</a>, [Ignored Line Prefix](#id893482), [Line Folding](#id894136), [Double Quoted](#id904245), [Single Quoted](#id905860), [Folded](#id929764)

</div>

<div class="indexdiv">

### Y

<span id="index-entry-YAML  directive"></span>YAML directive, [Directives](#id864824), [Directives](#id895217), <a href="#id895631" class="preferred">“YAML” Directive</a>

</div>

</div>

</div>

</div>

# ArkTS Specification

## Release 1.2.

#### 2025.10.



## CONTENTS









- 1 Introduction
   - 1.1 Overall Description
   - 1.2 Lexical and Syntactic Notation
   - 1.3 Terms and Definitions
- 2 Lexical Elements
   - 2.1 Use of Unicode Characters
   - 2.2 Lexical Input Elements
   - 2.3 White Spaces
   - 2.4 Line Separators
   - 2.5 Tokens
   - 2.6 Identifiers
   - 2.7 Keywords
   - 2.8 Operators and Punctuators
   - 2.9 Literals
      - 2.9.1 Numeric Literals
      - 2.9.2 Integer Literals
      - 2.9.3 Floating-Point Literals
      - 2.9.4 Bigint Literals
      - 2.9.5 Boolean Literals
      - 2.9.6 String Literals
      - 2.9.7 Multiline String Literal
      - 2.9.8 NullLiteral
      - 2.9.9 UndefinedLiteral
   - 2.10 Comments
   - 2.11 Semicolons
- 3 Types
   - 3.1 Predefined Types
   - 3.2 User-Defined Types
   - 3.3 Using Types
   - 3.4 Named Types
   - 3.5 Type References
   - 3.6 Value Types
      - 3.6.1 Numeric Types
      - 3.6.2 Integer Types and Operations
      - 3.6.3 Floating-Point Types and Operations
      - 3.6.4 Typeboolean.
   - 3.7 Reference Types
   - 3.8 TypeAny
   - 3.9 TypeObject
   - 3.10 Typenever.
   - 3.11 Typevoid
   - 3.12 Typeundefined.
   - 3.13 Typenull
   - 3.14 Typestring
   - 3.15 Typebigint
   - 3.16 Literal Types
      - 3.16.1 String Literal Types
   - 3.17 Array Types
      - 3.17.1 Resizable Array Types
      - 3.17.2 Readonly Array Types
   - 3.18 Tuple Types
      - 3.18.1 Readonly Tuple Types
   - 3.19 Function Types
      - 3.19.1 TypeFunction.
   - 3.20 Union Types
      - 3.20.1 Union Types Normalization
      - 3.20.2 Access to Common Union Members
      - 3.20.3 KeyofTypes
   - 3.21 Nullish Types
   - 3.22 Default Values for Types
- 4 Names, Declarations and Scopes
   - 4.1 Names
   - 4.2 Declarations
   - 4.3 Scopes
   - 4.4 Accessible
   - 4.5 Type Declarations
      - 4.5.1 Type Alias Declaration
   - 4.6 Variable and Constant Declarations
      - 4.6.1 Variable Declarations
      - 4.6.2 Constant Declarations
      - 4.6.3 Assignability with Initializer
      - 4.6.4 Type Inference from Initializer
   - 4.7 Function Declarations
      - 4.7.1 Signatures
      - 4.7.2 Parameter List
      - 4.7.3 Readonly Parameters
      - 4.7.4 Optional Parameters
      - 4.7.5 Rest Parameter
      - 4.7.6 Shadowing by Parameter
      - 4.7.7 Return Type
      - 4.7.8 Return Type Inference
- 5 Generics
   - 5.1 Type Parameters
      - 5.1.1 Type Parameter Constraint
      - 5.1.2 Type Parameter Default
      - 5.1.3 Type Parameter Variance
   - 5.2 Generic Instantiations
      - 5.2.1 Type Arguments
      - 5.2.2 Explicit Generic Instantiations
      - 5.2.3 Implicit Generic Instantiations
   - 5.3 Utility Types
      - 5.3.1 Awaited Utility Type
      - 5.3.2 NonNullable Utility Type
      - 5.3.3 Partial Utility Type
      - 5.3.4 Required Utility Type
      - 5.3.5 Readonly Utility Type
      - 5.3.6 Record Utility Type
      - 5.3.7 Utility Type Private Fields
- 6 Contexts and Conversions
   - 6.1 Assignment-like Contexts
   - 6.2 String Operator Contexts
   - 6.3 Numeric Operator Contexts
      - 6.3.1 Numeric Conversions for Relational and Equality Operands
   - 6.4 Implicit Conversions
      - 6.4.1 Widening Numeric Conversions
      - 6.4.2 Enumeration to Constants Type Conversions
   - 6.5 Numeric Casting Conversions
- 7 Expressions
   - 7.1 Evaluation of Expressions
      - 7.1.1 Type of Expression
      - 7.1.2 Normal and Abrupt Completion of Expression Evaluation
      - 7.1.3 Order of Expression Evaluation
      - 7.1.4 Operator Precedence
      - 7.1.5 Evaluation of Arguments
      - 7.1.6 Evaluation of Other Expressions
   - 7.2 Literal
   - 7.3 Named Reference
      - 7.3.1 Function Reference
      - 7.3.2 Method Reference
   - 7.4 Array Literal
      - 7.4.1 Array Literal Type Inference from Context
      - 7.4.2 Array Type Inference from Types of Elements
   - 7.5 Object Literal
      - 7.5.1 Object Literal of Class Type
      - 7.5.2 Object Literal of Interface Type
      - 7.5.3 Object Literal ofRecordType
      - 7.5.4 Object Literal Evaluation
   - 7.6 Spread Expression
   - 7.7 Parenthesized Expression
   - 7.8 thisExpression
   - 7.9 Field Access Expression
      - 7.9.1 Accessing Current Object Fields
      - 7.9.2 Accessing SuperClass Properties
   - 7.10 Method Call Expression
      - 7.10.1 Step 1: Selection of Type to Use
      - 7.10.2 Step 2: Selection of Method
      - 7.10.3 Step 3: Checking Method Modifiers
      - 7.10.4 Type of Method Call Expression
   - 7.11 Function Call Expression
   - 7.12 Indexing Expressions
      - 7.12.1 Array Indexing Expression
      - 7.12.2 String Indexing Expression
   - 7.12.3 Record Indexing Expression
- 7.13 Chaining Operator
- 7.14 NewExpressions
- 7.15 InstanceOfExpression
- 7.16 CastExpression
   - 7.16.1 Type Inference in Cast Expression
   - 7.16.2 Runtime Checking in Cast Expression
- 7.17 TypeOfExpression
- 7.18 Ensure-Not-Nullish Expression
- 7.19 Nullish-Coalescing Expression
- 7.20 Unary Expressions
   - 7.20.1 Postfix Increment
   - 7.20.2 Postfix Decrement
   - 7.20.3 Prefix Increment
   - 7.20.4 Prefix Decrement
   - 7.20.5 Unary Plus
   - 7.20.6 Unary Minus
   - 7.20.7 Bitwise Complement
   - 7.20.8 Logical Complement
- 7.21 Multiplicative Expressions
   - 7.21.1 Multiplication
   - 7.21.2 Division
   - 7.21.3 Remainder
   - 7.21.4 Exponentiation
- 7.22 Additive Expressions
   - 7.22.1 String Concatenation
   - 7.22.2 Additive Operators for Numeric Types
- 7.23 Shift Expressions
- 7.24 Relational Expressions
   - 7.24.1 Numeric Relational Operators
   - 7.24.2 String Relational Operators
   - 7.24.3 Boolean Relational Operators
   - 7.24.4 Enumeration Relational Operators
- 7.25 Equality Expressions
   - 7.25.1 Numeric Equality Operators
   - 7.25.2 Function Type Equality Operators
   - 7.25.3 Extended Equality withnullorundefined.
- 7.26 Bitwise and Logical Expressions
   - 7.26.1 Integer Bitwise Operators
   - 7.26.2 Boolean Logical Operators
- 7.27 Conditional-And Expression
- 7.28 Conditional-Or Expression
- 7.29 Assignment
   - 7.29.1 Simple Assignment Operator
   - 7.29.2 Compound Assignment Operators
   - 7.29.3 Left-Hand-Side Expressions
- 7.30 Ternary Conditional Expressions
- 7.31 String Interpolation Expressions
- 7.32 Lambda Expressions
   - 7.32.1 Lambda Signature
   - 7.32.2 Lambda Body
   - 7.32.3 Lambda Expression Type
   - 7.32.4 Runtime Evaluation of Lambda Expressions
- 7.33 Constant Expressions
- 8 Statements
   - 8.1 Normal and Abrupt Statement Execution
   - 8.2 Expression Statements
   - 8.3 Block
   - 8.4 Local Declarations
   - 8.5 ifStatements
   - 8.6 Loop Statements
   - 8.7 whileStatements anddoStatements
   - 8.8 forStatements
   - 8.9 for-ofStatements
   - 8.10 breakStatements
   - 8.11 continueStatements
   - 8.12 returnStatements
   - 8.13 switchStatements
   - 8.14 throwStatements
   - 8.15 tryStatements
      - 8.15.1 catchClause
      - 8.15.2 finallyClause
      - 8.15.3 tryStatement Execution
- 9 Classes
   - 9.1 Class Declarations
      - 9.1.1 Abstract Classes
   - 9.2 Class Extension Clause
   - 9.3 Class Implementation Clause
      - 9.3.1 Implementing Interface Methods
      - 9.3.2 Implementing Required Interface Properties
      - 9.3.3 Implementing Optional Interface Properties
   - 9.4 Class Members
   - 9.5 Access Modifiers
      - 9.5.1 Private Access Modifier
      - 9.5.2 Protected Access Modifier
      - 9.5.3 Public Access Modifier
   - 9.6 Field Declarations
      - 9.6.1 Static and Instance Fields
      - 9.6.2 Readonly (Constant) Fields
      - 9.6.3 Optional Fields
      - 9.6.4 Field Initialization
      - 9.6.5 Fields with Late Initialization
      - 9.6.6 Overriding Fields
   - 9.7 Method Declarations
      - 9.7.1 Static Methods
      - 9.7.2 Instance Methods
      - 9.7.3 Abstract Methods
      - 9.7.4 Async Methods
      - 9.7.5 Overriding Methods
      - 9.7.6 Native Methods
      - 9.7.7 Method Body
      - 9.7.8 Methods Returningthis
   - 9.8 Class Accessor Declarations
   - 9.9 Constructor Declaration
      - 9.9.1 Formal Parameters
      - 9.9.2 Constructor Body
      - 9.9.3 Explicit Constructor Call
      - 9.9.4 Default Constructor
   - 9.10 Inheritance
- 10 Interfaces
   - 10.1 Interface Declarations
   - 10.2 Superinterfaces and Subinterfaces
   - 10.3 Interface Members
   - 10.4 Interface Properties
      - 10.4.1 Required Interface Properties
      - 10.4.2 Optional Interface Properties
   - 10.5 Interface Method Declarations
   - 10.6 Interface Inheritance
- 11 Enumerations
   - 11.1 Enumeration Integer Values
   - 11.2 Enumeration String Values
   - 11.3 Enumeration Operations
- 12 Error Handling
   - 12.1 Errors
- 13 Modules and Namespaces
   - 13.1 Import Directives
      - 13.1.1 Bind All with Qualified Access
      - 13.1.2 Default Import Binding
      - 13.1.3 Selective Binding
      - 13.1.4 Import Type Directive
      - 13.1.5 Import Path
      - 13.1.6 Several Bindings for One Import Path
   - 13.2 Standard Library Usage
   - 13.3 Top-Level Declarations
      - 13.3.1 Exported Declarations
   - 13.4 Namespace Declarations
   - 13.5 Export Directives
      - 13.5.1 Selective Export Directive
      - 13.5.2 Single Export Directive
      - 13.5.3 Export Type Directive
      - 13.5.4 Re-Export Directive
   - 13.6 Top-Level Statements
   - 13.7 Program Entry Point
- 14 Ambient Declarations
   - 14.1 Ambient Constant Declarations
   - 14.2 Ambient Function Declarations
   - 14.3 Ambient Overload Function Declarations
   - 14.4 Ambient Class Declarations
      - 14.4.1 Ambient Indexer
      - 14.4.2 Ambient Call Signature
      - 14.4.3 Ambient Iterable
   - 14.5 Ambient Interface Declarations
   - 14.6 Ambient Namespace Declarations
      - 14.6.1 Implementing Ambient Namespace Declaration
- 15 Semantic Rules
   - 15.1 Semantic Essentials
      - 15.1.1 Type of Standalone Expression
      - 15.1.2 Specifics of Assignment-like Contexts
      - 15.1.3 Specifics of Variable Initialization Context
      - 15.1.4 Specifics of Numeric Operator Contexts
      - 15.1.5 Specifics of String Operator Contexts
      - 15.1.6 Other Contexts
      - 15.1.7 Specifics of Type Parameters
      - 15.1.8 Semantic Essentials Summary
   - 15.2 Subtyping
      - 15.2.1 Subtyping for Non-Generic Classes and Interfaces
      - 15.2.2 Subtyping for Generic Classes and Interfaces
      - 15.2.3 Subtyping for Literal Types
      - 15.2.4 Subtyping for Union Types
      - 15.2.5 Subtyping for Function Types
      - 15.2.6 Subtyping for Fixed-Size Array Types
      - 15.2.7 Subtyping for Intersection Types
      - 15.2.8 Subtyping for Difference Types
   - 15.3 Type Identity
   - 15.4 Assignability
   - 15.5 Invariance, Covariance and Contravariance
   - 15.6 Compatibility of Call Arguments
   - 15.7 Type Inference
      - 15.7.1 Type Inference for Numeric Literals
      - 15.7.2 Smart Types
   - 15.8 Overriding
      - 15.8.1 Overriding in Classes
      - 15.8.2 Overriding and Overloading in Interfaces
      - 15.8.3 Override-Compatible Signatures
   - 15.9 Overloading
      - 15.9.1 Overload Resolution
   - 15.10 Type Erasure
   - 15.11 Static Initialization
      - 15.11.1 Static Initialization Safety
   - 15.12 Dispatch
   - 15.13 Compatibility Features
      - 15.13.1 Extended Conditional Expressions
- 16 Concurrency
   - 16.1 Introductory Note
   - 16.2 Concurrency Subsystem Overview
      - 16.2.1 Major Concurrency Features
   - 16.3 Asynchronous API
      - 16.3.1 AsyncFunctions
      - 16.3.2 AsyncLambdas
      - 16.3.3 AsyncMethods
      - 16.3.4 await.
      - 16.3.5 Promise
      - 16.3.6 Unhandled Rejected Promises
   - 16.4 Coroutines (Experimental)
- 17 Experimental Features
   - 17.1 Typechar
      - 17.1.1 Character Literals
      - 17.1.2 Character Equality Operators
   - 17.2 Fixed-Size Array Types
      - 17.2.1 Fixed-Size Array Creation
   - 17.3 Resizable Array Creation Expressions
      - 17.3.1 Runtime Evaluation of Array Creation Expressions
   - 17.4 Enumerations Experimental
      - 17.4.1 Enumeration Methods
   - 17.5 Indexable Types
   - 17.6 Iterable Types
   - 17.7 Callable Types
      - 17.7.1 Callable Types with$_invokeMethod
      - 17.7.2 Callable Types with$_instantiateMethod
   - 17.8 Statements
      - 17.8.1 For-of Explicit Type Annotation
   - 17.9 Overload Declarations
      - 17.9.1 Function Overload Declarations
      - 17.9.2 Class Method Overload Declarations
      - 17.9.3 Interface Method Overload Declarations
      - 17.9.4 Constructor Overload Declarations
      - 17.9.5 Overload Alias Name Same As Function Name
      - 17.9.6 Overload Alias Name Same As Method Name
   - 17.10 Native Functions and Methods
      - 17.10.1 Native Functions
      - 17.10.2 Native Methods
      - 17.10.3 Native Constructors
   - 17.11 Classes Experimental
      - 17.11.1 Final Classes
      - 17.11.2 Final Methods
      - 17.11.3 Constructor Names
   - 17.12 Default Interface Method Declarations
   - 17.13 Adding Functionality to Existing Types
      - 17.13.1 Functions with Receiver
      - 17.13.2 Receiver Type
      - 17.13.3 Accessors with Receiver
      - 17.13.4 Function Types with Receiver
      - 17.13.5 Lambda Expressions with Receiver
      - 17.13.6 Implicitthisin Lambda with Receiver Body
   - 17.14 Trailing Lambdas
- 18 Annotations
   - 18.1 Declaring Annotations
      - 18.1.1 Types of Annotation Fields
   - 18.2 Using Annotations
      - 18.2.1 Using Single Field Annotations
   - 18.3 Exporting and Importing Annotations
   - 18.4 Ambient Annotations
   - 18.5 Standard Annotations
      - 18.5.1 Retention Annotation
   - 18.6 Runtime Access to Annotations
- 19 Standard Library
- 20 Implementation Details
   - 20.1 Import Path Lookup
   - 20.2 Modules in Host System
   - 20.3 Getting Type Via Reflection
   - 20.4 Ensuring Module Initialization
   - 20.5 Generic and Function Types Peculiarities
   - 20.6 Keywordstructand ArkUI
   - 20.7 OutOfMemoryErrorfor Primitive Type Operations
   - 20.8 Make a Bridge Method for Overriding Method
- 21 Grammar Summary
- 22 Contributors
- Index


**x**


##### CHAPTER

### ONE

### INTRODUCTION

This document presents complete information on the new common-purpose, multiparadigm programming language
called ArkTS.

### 1.1 Overall Description

The ArkTS language combines and supports features that have already proven helpful and powerful in many well-known
programming languages.

ArkTS supports imperative, object-oriented, functional, and generic programming paradigms, and combines them
safely and consistently.

At the same time, ArkTS does not support features that allow software developers to write dangerous, unsafe, or inef-
ficient code. In particular, the language uses the strong static typing principle. Object types are determined by their
declarations, and no dynamic type change is allowed. The semantic correctness is checked at compile time.

ArkTS is designed as a part of the modern language manifold. To provide an efficient and safely executable code, the
language takes flexibility and power from TypeScript and its predecessor JavaScript, and the static typing principle
from Java and Kotlin. The overall design keeps the ArkTS syntax style similar to that of those languages, and some of
its important constructs are almost identical to theirs on purpose.

In other words, there is a significant _common subset_ of features of ArkTS on the one hand, and of TypeScript, JavaScript,
Java, and Kotlin on the other. Consequently, the ArkTS style and constructs are no puzzle for the TypeScript and Java
users who can intuitively sense the meaning of most constructs of the new language even if not understand them
completely.

This stylistic and semantic similarity permits smoothly migrating the applications originally written in TypeScript,
Java, or Kotlin to ArkTS.

Like its predecessors, ArkTS is a relatively high-level language. It means that the language provides no access to
low-level machine representations. As a high-level language, ArkTS supports automatic storage management, i.e., all
dynamically created objects are deallocated automatically soon after they are no longer available, and deallocating them
explicitly is not required.

ArkTS is not merely a language, but rather a comprehensive software development ecosystem that facilitates the creation
of software solutions in various application domains.

The ArkTS ecosystem includes the language along with its compiler, accompanying documents, guidelines, tutorials,
the standard library (see _Standard Library_ ), and a set of additional tools that perform transition from other languages
(currently, TypeScript and Java) to ArkTS automatically or semi-automatically.

The ArkTS language as a whole is characterized by the following:

##### 1


- **Object orientation**
    The ArkTS language supports the _object-oriented programming_ (OOP) approach based on classes and interfaces.
    The major notions of this approach are as follows:
       **-** Classes with single inheritance,
       **-** Interfaces as abstractions to be implemented by classes, and
       **-** Methods (class instance or interface methods) with overriding and dynamic dispatch mechanisms.
    Object orientation is common in many if not all modern programming languages. It enables powerful, flexible,
    safe, clear, and adequate software design.
- **Modularity**
    The ArkTS language supports the _component programming_ approach. It presumes that software is designed and
    implemented as a composition of _modules_.
    A _module_ in ArkTS is a standalone, independently compiled unit that combines various programming resources
    (types, classes, functions, and so on). A module can communicate with other modules by exporting all or some
    of its resources to, or importing from other modules.
- **Genericity**
    Some program entities in ArkTS can be _type-parameterized_. It means that an entity can represent a very high-
    level (abstract) concept. Providing more concrete type information constitutes the instantiation of an entity for a
    particular use case.
    A classical illustration is the notion of a list that represents the ‘idea’ of an abstract data structure. An abstract
    notion can be turned into a concrete list by providing additional information (i.e., type of list elements).
    A similar feature ( _generics_ or _templates_ ) supported by many programming languages enables making programs
    and program structures more generic and reusable, and serves as a basis of the generic programming paradigm.
- **Multitargeting**
    ArkTS provides an efficient application development solution for a wide range of devices. The developer-friendly
    ArkTS ecosystem is a _cross-platform development_ providing a uniform programming environment for many pop-
    ular platforms. It can generate optimized applications capable of operating under the limitations of lightweight
    devices, or realizing the full potential of any specific-target hardware.

### 1.2 Lexical and Syntactic Notation

This section introduces the notation known as _context-free grammar_. The notation is used throughout this specification
to define the lexical and syntactic structure of a program.

The ArkTS lexical notation defines a set of rules, or productions that specify the structure of the elementary lan-
guage parts called _tokens_. All tokens are defined in _Lexical Elements_. The set of tokens (identifiers, keywords, num-
bers/numeric literals, operator signs, delimiters), special characters (white spaces and line separators), and comments
comprises the language’s _alphabet_.

The tokens defined by the lexical grammar are terminal symbols of syntactic notation. Syntactic notation defines a set
of productions starting from the goal symbol _moduleDeclaration_ (see _Modules and Namespaces_ ). It is a sentence that
consists of a single distinguished nonterminal, and describes how sequences of tokens can form syntactically correct
programs.

**2 Chapter 1. Introduction**


Lexical and syntactic grammars are defined as a range of productions, and each production is comprised of the follow-
ing:

- Abstract symbol ( _nonterminal_ ) as its left-hand side,
- Sequence of one or more _nonterminal_ and _terminal_ symbols as its _right-hand side_ ,
- Character ‘:’ as a separator between the left- and right-hand sides, and
- Character ‘;’ as the end marker.

A grammar starts from the goal symbol and specifies the language, i.e., the set of possible sequences of terminal
symbols that can result from repeatedly replacing any nonterminal in the left-hand-side sequence for a right-hand side
of the production.

Grammars can use the following additional symbols (sometimes called _metasymbols_ ) in the right-hand side of a gram-
mar production along with terminal and nonterminal symbols:

- Vertical line ‘|’ to specify alternatives.
- Question mark ‘?’ to specify an optional occurrence (zero- or one-time) of the preceding terminal or nonterminal.
- Asterisk ‘*’ to mark a _terminal_ or _nonterminal_ that can occur zero or more times.
- Parentheses ‘(’ and ‘)’ to enclose any sequence of terminals and/or nonterminals marked with the metasymbols
    ‘?’ or ‘*’.

The metasymbols specify the structuring rules for terminal and nonterminal sequences. However, they are not part of
terminal symbol sequences that comprise the resultant program text.

The example below represents a production that specifies a list of expressions:

expressionList:
expression (','expression)*','?
;

This production introduces the following structure defined by the nonterminal _expressionList_. The expression list
must consist of a sequence of _expressions_ separated by the terminal symbol ‘,’. The sequence must have at least one
_expression_. The list is optionally terminated by the terminal symbol ‘,’.

All grammar rules are presented in the Grammar section (see _Grammar Summary_ ) of this Specification.

### 1.3 Terms and Definitions

This section contains the alphabetical list of important terms found in the Specification, and their ArkTS-specific
definitions. Such definitions are not generic and can differ significantly from the definitions of the same terms as used
in other languages, application areas, or industries.

**abstract declaration**

- an ordinary interface method declaration that specifies the method’s name and signature.

**array length**

- the number of elements in a resizable array.

**array type**

- a type that consists of more than one element.

**1.3. Terms and Definitions 3**


**casting conversion**

- a conversion of an operand of a cast expression to an explicitly specified type.

**class level scope**

- a name that is declared inside a class, and is accessible inside the class and sometimes outside that class by
means of an access modifier, or via a derived class).

**comment**

- a piece of text, insignificant for the syntactic grammar, that is added to a stream in order to document and
compliment source code.

**compile-time error**

- a text message displayed by the compiler if an error is identified in a program code that prevents the code to be
generated.

**compile-time warning**

- a text message displayed by the compiler if a program code is found to have some logical inconsistencies, and
it is recommended that the programmer reconsiders the design and actual coding.

**constant**

- see _constant declaration_.

**constant declaration**

- declaration that introduces a new variable to which an immutable initial value can be assigned only once at the
time of instantiation.

**context-free grammar**

- grammar in which the left-hand side of each production rule consists of only a single nonterminal symbol.

**expression**

- a formula for calculating values. An expression has the syntactic form that is a composition of operators and
parentheses, where parentheses are used to change the order of calculation. The default order of calculation is
determined by operator preferences.

**fit into (v.)**

- belong, or be implicitly convertible to an entity (see _Widening Numeric Conversions_ ).

**fixed-size array type**

- a built-in type that consists of more than one element, and has its length set only once to achieve a better
performance.

**function declaration**

- a declaration that specifies names, signatures, and bodies when introducing a named function.

**function scope**

- same as _method scope_.

**function type parameter scope**

- a scope of a type parameter name in a function declaration. It is identical to that entire declaration.

**function types conversion**

- a conversion of one function type to another.

**generic**

- see _generic type_.

**generic type**

- a named type (class or interface) that has type parameters.

**goal symbol**

- a sentence that consists of a single distinguished nonterminal ( _moduleDeclaration_ ). The _goal symbol_ describes
how sequences of tokens can form syntactically correct programs.

**4 Chapter 1. Introduction**


**grammar**

- set of rules that describe what possible sequences of terminal and nonterminal symbols a programming lan-
guage interprets as correct.
Grammar is a range of productions. Each production comprises an abstract symbol (nonterminal) as its left-hand
side, and a sequence of nonterminal and terminal symbols as its right-hand side. Each production contains the
characters ‘:’ as a separator between the left- and right-hand sides, and ‘;’ as the end marker.

**interface level scope**

- a name declared inside an interface is considered to have interface level scope, and is accessible inside and
outside the interface.

**keyword**

- one of _reserved words_ that have their meanings permanently predefined in the language.

**linearization**

- de-nesting of all nested types in a union type to present them in the form of a flat line that includes no more
union types.

**literal**

- a representation of a value type.

**match (v.)**

- correspond to an entity.

**metasymbol**

- additional symbols ‘|’, ‘?’, ‘*’, ‘(’, and ‘)’ that can be used along with terminal and nonterminal symbols in
the right-hand side of a grammar production.

**method**

- an ordered 3-tuple consisting of type parameters, argument types, and return types.

**method scope**

- a scope of a name declared immediately inside the body of a method (function) declaration. Method scope is
identical to the body of that method (function) declaration from the place of declaration and up to the end of the
body.

**module level scope**

- a name in the module level scope that is applicable to modules only, and is accessible throughout the entire
module and in other modules if exported.

**narrowing conversion**

- a conversion that can cause a loss information about the overall magnitude of a numeric value, and potentially
a loss of precision and range.

**non-generic**

- see _non-generic type_.

**non-generic type**

- a named type (class or interface) that has no type parameters.

**nonterminal**

- see _nonterminal symbol_.

**nonterminal symbol**

- a syntactically variable token that results from the successive application of production rules.

**nullable type**

- a variable declared to have the valuenull, ortype T | nullthat can hold values of typeTand its derived
types.

**1.3. Terms and Definitions 5**


**nullish value**

- a reference which is null or undefined.

**operand**

- an argument of an operation. Syntactically, operands have the form of simple or qualified identifiers that refer
to variables or members of structured objects. In turn, operands can be operators whose preferences (‘priorities’)
are higher than the preference of a given operator.

**operation**

- an informal notion that signifies an action or a process of operator evaluation.

**operation sign**

- a language token that signifies an operator and conventionally denotes a usual mathematical operator, e.g., ‘+’
for addition, ‘/’ for division, etc. However, some languages allow using identifiers to denote operators, and/or
arbitrarily combining characters that are not tokens in the alphabet of that language (i.e., operator signs).

**operator (in programming languages)**

- the term can have several meanings as follows:
(1) a token that denotes the action to be performed on a value (addition, subtraction, comparison, etc.).
(2) a syntactic construct that denotes an elementary calculation within an expression. An operator normally
consists of an operator sign and one or more operands.
In unary operators that have a single operand, the operator sign can be placed either in front of or after an operand
( _prefix_ and _postfix_ unary operator respectively).
If both operands are available, then the operator sign can be placed between the two ( _infix_ binary operator). A
conditional operator with three operands is called _ternary_.
Some operators have special notations. For example, an indexing operator has a conventional form like a[i] while
formally being a binary operator.
Some languages treat operators as _syntactic sugar_ , i.e., a conventional version of a more common construct or
_function call_. Therefore, an operator likea+bis conceptually handled as the call+(a,b), where the operator
sign plays the role of a function name, and the operands are function call arguments.

**overloading**

- a language feature that allows using a single name to call several functions (in the general sense, i.e., including
methods and constructors) with different signatures and different bodies.

**own (adj.)**

- of a member textually declared in a class, interface, type, etc., as opposed to members inherited from base class
(superclass), base interfaces (superinterface), base type (supertype), etc.

**production**

- a sequence of terminal and nonterminal symbols that a programming language interprets as correct.

**punctuator**

- a token that serves to separate, complete, or otherwise organize program elements and parts: commas, semi-
colons, parentheses, square brackets, etc.

**qualified name**

- a name that consists of a sequence of identifiers separated with the token ‘.’.

**resizable array type**

- a built-in type that consists of more than one element, and can have the number of constituent elements changed
at runtime.

**scope of a name**

- a region of program code within which an entity—as declared by that name—can be accessed or referred to
by its simple name without any qualification.

**6 Chapter 1. Introduction**


**simple name**

- a name that consists of a single identifier.

**static member**

- a class member that is not related to a particular class instance. A static member can be used across an entire
program by using a qualified name notation (qualification is the name of a class).

**subcomponent (derived component, child component)**

- a component produced by, inherited from, and dependent from another component.

**supercomponent (base component, parent component)**

- a component from which another component is derived.

**terminal**

- see _terminal symbol_.

**terminal symbol**

- a syntactically invariable token (i.e., a syntactic notation defined directly by an invariable form of the lexical
grammar that defines a set of productions starting from the _goal symbol_ ).

**token**

- an elementary part of a programming language: identifier, keyword, operator and punctuator, or literal. To-
kens are lexical input elements that form the vocabulary of a language, and can act as terminal symbols of the
language’s syntactic grammar.

**tokenization**

- finding the longest sequence of characters that forms a valid token (i.e., _establishing_ a token) in the process of
codebase reading by the machine.

**type parameter scope**

- the scope of a name of a type parameter that is declared in a class or an interface. Type parameter scope is
identical to the entire declaration (except static member declarations).

**type reference**

- references that refer to named types by specifying their type names and type arguments, where applicable, to
be substituted for type parameters of the named type.

**variable**

- see _variable declaration_.

**variable declaration**

- a declaration that introduces a new named variable to which a modifiable initial value can be assigned.

**white space**

- lexical input elements that separates tokens from one another in order to improve the source code readability
and avoid ambiguities.

**widening conversion**

- a conversion that causes no loss of information about the overall magnitude of a numeric value.

**1.3. Terms and Definitions 7**


**8 Chapter 1. Introduction**


##### CHAPTER

### TWO

### LEXICAL ELEMENTS

This chapter discusses the lexical structure of the ArkTS programming language.

### 2.1 Use of Unicode Characters

The ArkTS programming language uses characters of the Unicode Character set^1 as its terminal symbols. It uses the
Unicode UTF-16 encoding to represent text in sequences of 16-bit code units.

The term _Unicode code point_ is used in this specification only where such representation is relevant to refer the reader
to Unicode Character set and UTF-16 encoding. Where such representation is irrelevant to the discussion, the generic
term _character_ is used.

### 2.2 Lexical Input Elements

The language has the following types of _lexical input elements_ :

- _White Spaces_ ,
- _Line Separators_ ,
- _Tokens_ , and
- _Comments_.

### 2.3 White Spaces

_White spaces_ are lexical input elements that separate tokens from one another. White spaces include the following:

- Space (U+0020),
- Horizontal tabulation (U+0009),

(^1) Unicode Standard Version 15.0.0, https://www.unicode.org/versions/Unicode15.0.0/

##### 9


- Vertical tabulation (U+000B),
- Form feed (U+000C),
- No-break space (U+00A0), and
- Zero-width no-break space (U+FEFF).

White spaces improve source code readability and help avoiding ambiguities. White spaces are ignored by the syntactic
grammar (see _Grammar Summary_ ). White spaces never occur within a single token, but can occur within a comment.

### 2.4 Line Separators

_Line separators_ are lexical input elements that separate tokens from one another and divide sequences of Unicode input
characters into lines. Line separators include the following:

- Newline character (U+000A or ASCII <LF>),
- Carriage return character (U+000D or ASCII <CR>),
- Line separator character (U+2028 or ASCII <LS>), and
- Paragraph separator character (U+2029 or ASCII <PS>).

Line separators improve source code readability. Any sequence of line separators is considered a single separator.

Line separators are often treated as white spaces, except where line separators have special meanings. See _Semicolons_
for more details.

### 2.5 Tokens

Tokens form the vocabulary of the language. There are four classes of tokens:

- _Identifiers_ ,
- _Keywords_ ,
- _Operators and Punctuators_ , and
- _Literals_.

_Token_ is the only lexical input element that can act as a terminal symbol of the syntactic grammar (see _Grammar
Summary_ ). In the process of tokenization, the next token is always the longest sequence of characters that form a
valid token. Tokens are separated by white spaces (see _White Spaces_ ), operators, or punctuators (see _Operators and
Punctuators_ ). White spaces are ignored by the syntactic grammar (see _Grammar Summary_ ).

**10 Chapter 2. Lexical Elements**


### 2.6 Identifiers

_Identifier_ is a sequence of one or more valid Unicode characters. The Unicode grammar of identifiers is based on
character properties specified by the Unicode Standard.

The first character in an identifier must be ‘$’, ‘_’, or any Unicode code point with the Unicode property ‘ID_Start’^2.
Other characters must be Unicode code points with the Unicode property, or one of the following characters:

- ‘$’ (\U+0024),
- ‘Zero-Width Non-Joiner’ (<ZWNJ>, \U+200C), or
- ‘Zero-Width Joiner’ (<ZWJ>, \U+200D).

Identifier:
IdentifierStart IdentifierPart*
;

IdentifierStart:
UnicodeIDStart
| '$'
| '_'
| '\\'EscapeSequence
;

IdentifierPart:
UnicodeIDContinue
| '$'
| ZWNJ
| ZWJ
| '\\'EscapeSequence
;

ZWJ:
'\u200C'
;
ZWNJ:
'\u200D'
;

UnicodeIDStart
: Letter
| ['$']
| '\\'UnicodeEscapeSequence;

UnicodeIDContinue
: UnicodeIDStart
| UnicodeDigit
| '\u200C'
| '\u200D';

UnicodeEscapeSequence:
'u'HexDigit HexDigit HexDigit HexDigit
| 'u' '{' HexDigit HexDigit+'}'
(continues on next page)

(^2) https://unicode.org/reports/tr31/
**2.6. Identifiers 11**


```
(continued from previous page)
;
```
Letter
: UNICODE_CLASS_LU
| UNICODE_CLASS_LL
| UNICODE_CLASS_LT
| UNICODE_CLASS_LM
| UNICODE_CLASS_LO
;
UnicodeDigit
: UNICODE_CLASS_ND
;

See _Grammar Summary_ for the Unicode character categories _UNICODE_CLASS_LU_ , _UNICODE_CLASS_LL_ , _UNI-
CODE_CLASS_LT_ , _UNICODE_CLASS_LM_ , _UNICODE_CLASS_LO_ , and _UNICODE_CLASS_ND_.

### 2.7 Keywords

_Keywords_ are reserved words with meanings permanently predefined in ArkTS. Keywords are case-sensitive, and their
exact spelling is presented in the following four tables. The kinds of keywords are discussed below.

1. The following _hard keywords_ are reserved in any context, and cannot be used as identifiers:

```
abstract enum let this
as export native throw
async extends new true
await false null try
break final overload typeof
case for override undefined
class function private while
const if protected
constructor implements public
continue import return
default in static
do instanceof switch
else interface super
```
2. Names and aliases of predefined types are _hard keywords_ , and cannot be used as identifiers:

**12 Chapter 2. Lexical Elements**


```
Primary name Alias
Any
bigint BigInt
boolean Boolean
byte Byte
char Char
double Double
float Float
int Int
long Long
number Number
Object object
short Short
string String
void
```
3. The following _soft keywords_ have special meaning in certain contexts but are valid identifiers elsewhere:

```
catch namespace
declare of
finally out
from readonly
get set
keyof type
```
4. The following identifiers are also treated as _soft keywords_ reserved for the future use, or currently used in TypeScript:

```
is struct var yield
```
### 2.8 Operators and Punctuators

_Operators_ are tokens that denote various actions to be performed on values: addition, subtraction, comparison, and
other. The keywordsinstanceofandtypeofalso act as operators.

_Punctuators_ are tokens that separate, complete, or otherwise organize program elements and parts: commas, semi-
colons, parentheses, square brackets, etc.

The following character sequences represent operators and punctuators:

**2.8. Operators and Punctuators 13**


##### + & += |= &= < ?.

##### - | -= ^= && >!

##### * ^ *= <<= || === <=

##### / >> /= >>= ++ == >=

##### % << %= >>>= -- = ...

##### ( ) [ ] { } ??

##### , ;. : != !== **

##### **=

### 2.9 Literals

_Literals_ are values of certain types (see _Predefined Types_ and _Literal Types_ ).

Literal:
IntegerLiteral
| FloatLiteral
| BigIntLiteral
| BooleanLiteral
| StringLiteral
| MultilineStringLiteral
| NullLiteral
| UndefinedLiteral
| CharLiteral
;

See _Character Literals_ for the experimentalchar literal.

Each literal is described in detail below.

#### 2.9.1 Numeric Literals

_Numeric literals_ include integer and floating-point literals.

#### 2.9.2 Integer Literals

Integer literals represent numbers that have neither a decimal point nor an exponential part. Integer literals can be
written with radices 16 (hexadecimal), 10 (decimal), 8 (octal), and 2 (binary) as follows:

IntegerLiteral:
DecimalIntegerLiteral
(continues on next page)

**14 Chapter 2. Lexical Elements**


```
(continued from previous page)
| HexIntegerLiteral
| OctalIntegerLiteral
| BinaryIntegerLiteral
;
```
```
DecimalIntegerLiteral:
' 0 '
| DecimalDigitNotZero ('_'? DecimalDigit)*
;
```
```
DecimalDigit:
[0-9]
;
```
```
DecimalDigitNotZero:
[1-9]
;
```
```
HexIntegerLiteral:
' 0 '[xX] (HexDigit
| HexDigit(HexDigit| '_')* HexDigit
)
;
```
```
HexDigit:
[0-9a-fA-F]
;
```
```
OctalIntegerLiteral:
' 0 '[oO] (OctalDigit
| OctalDigit(OctalDigit| '_')* OctalDigit )
;
```
```
OctalDigit:
[0-7]
;
```
```
BinaryIntegerLiteral:
' 0 '[bB] (BinaryDigit
| BinaryDigit(BinaryDigit|'_')* BinaryDigit)
;
```
```
BinaryDigit:
[0-1]
;
```
```
Integral literals with different radices are represented by the examples below:
```
1 153 // decimal literal
2 1_153 // decimal literal
3 0xBAD3 // hex literal
4 0xBAD_3// hex literal
(continues on next page)

```
2.9. Literals 15
```

```
(continued from previous page)
5 0o777 // octal literal
6 0b101 // binary literal
```
```
The underscore character ‘_’ between successive digits can be used to improve readability. Underscore characters in
such positions do not change the values of literals. However, the underscore character must be neither the very first nor
the very last symbol of an integer literal.
Type of integer literal is determined by using Type Inference for Numeric Literals if its context allows inferring type.
Otherwise, the type is determened as follows:
```
- intif the literal value can be represented by a non-negative 32-bit number, i.e., the value is in the range
    0..max(int); or
- longotherwise.
A compile-time error occurs if an integer literal value is too large for the values of typelong. The concept is represented
by the examples below:

1 // literals of type int:
2 0
3 1
4 0x7F
5 0x7FFF_FFFF// max(int)
6
7 // literals of type long:
8 0x8000_0000
9 0x7FFF_FFFF_1
10 9223372036854775807 // max(long)
11
12 // compile-time error as value is too large:
13 9223372036854775808 // max(long) + 1
14 0xFFFF_FFFF_FFFF_FFFF_0

#### 2.9.3 Floating-Point Literals

```
Floating-point literals represent decimal numbers and consist of a whole-number part, a decimal point, a fraction part,
an exponent, and afloattype suffix as follows:
```
```
FloatLiteral:
DecimalIntegerLiteral'.' FractionalPart? ExponentPart? FloatTypeSuffix?
|'.'FractionalPart ExponentPart?FloatTypeSuffix?
|DecimalIntegerLiteral ExponentPart? FloatTypeSuffix
;
```
```
ExponentPart:
[eE] [+-]?DecimalIntegerLiteral
;
```
```
FractionalPart:
DecimalDigit
(continues on next page)
```
```
16 Chapter 2. Lexical Elements
```

```
(continued from previous page)
|DecimalDigit(DecimalDigit|'_')*DecimalDigit
;
FloatTypeSuffix:
'f'
;
```
```
The concept is represented by the examples below:
```
1 3.14
2 3.14f
3 3.141_592
4 .5
5 1234f
6 1e10
7 1e10f

```
The underscore character ‘_’ between successive digits can be used to improve readability. Underscore characters in
such positions do not change the values of literals. However, the underscore character must be neither the very first nor
the very last symbol of a literal.
Floating-point literals are of floating-point types that match literals as follows:
```
- floatif _float type suffix_ is present; or
- floatordoublethat is inferred using _Type Inference for Numeric Literals_ if its context allows to infer type; or
- doubleotherwise (typenumberis an alias todouble).
A compile-time error occurs if a floating-point literal is too large for its type:

1 // compile-time error as value is too large for type float:
2 3.4e39f
3
4 // compile-time error as value is too large for type double:
5 1.7e309

#### 2.9.4 Bigint Literals

```
Bigint literals represent integer numbers with an unlimited number of digits.
Bigint literals are always of typebigint(see Type bigint ).
Abigintliteral is an integer literal followed by the symbol ‘n’:
```
```
BigIntLiteral:
' 0 n'
| [1-9] ('_'? [0-9])* 'n'
;
```
```
The concept is represented by the examples below:
```
```
2.9. Literals 17
```

153n// bigint literal
1_153n // bigint literal
-153n// negative bigint literal

The underscore character ‘_’ between successive digits can be used to improve readability. Underscore characters in
such positions do not change the values of literals. However, the underscore character must be neither the very first nor
the very last symbol of abigintliteral.

Strings that represent numbers or any integer value can be converted tobigintby using built-in functions as follows:

BigInt(other:string): bigint
BigInt(other:long): bigint

Two methods allow taking _bitsCount_ lower bits of abigintnumber and return them as a result. Signed and unsigned
versions are both possible as follows:

asIntN(bitsCount:long, bigIntToCut: bigint): bigint
asUintN(bitsCount:long, bigIntToCut: bigint): bigint

#### 2.9.5 Boolean Literals

The two _boolean literal_ values are represented by the keywordstrueandfalse.

BooleanLiteral:
'true' |'false'
;

_Boolean literals_ are of thebooleantype.

#### 2.9.6 String Literals

_String literals_ consist of zero or more characters enclosed between single or double quotes. A special form of string
literals is _multiline string_ literal (see _Multiline String Literal_ ).

_String literals_ are of the literal type that corresponds to the literal. If an operator is applied to the literal, then the literal
type is replaced forstring(see _Type string_ ).

StringLiteral:
'"' DoubleQuoteCharacter*'"'
|'\'' SingleQuoteCharacter* '\''
;

DoubleQuoteCharacter:
~["\\\r\n]
|'\\' EscapeSequence
;

SingleQuoteCharacter:
~['\\\r\n]
|'\\' EscapeSequence
(continues on next page)

**18 Chapter 2. Lexical Elements**


```
(continued from previous page)
;
```
```
EscapeSequence:
['"bfnrtv0\\]
|'x'HexDigit HexDigit
|'u'HexDigit HexDigit HexDigit HexDigit
|'u' '{'HexDigit+ '}'
| ~[1-9xu\r\n]
;
```
```
Characters in string literals normally represent themselves. However, certain non-graphic characters can be represented
by explicit specifications or Unicode codes. Such constructs are called escape sequences.
Escape sequences can represent graphic characters within a string literal , e.g., single quotes ‘'’, double quotes ‘"’,
backslashes ‘\’, and some others. An escape sequence always starts with the backslash character ‘\’, followed by one
of the following characters:
```
- "(double quote, U+0022),
- '(neutral single quote, U+0027),
- b(backspace, U+0008),
- f(form feed, U+000c),
- n(linefeed, U+000a),
- r(carriage return, U+000d),
- t(horizontal tab, U+0009),
- v(vertical tab, U+000b),
- \(backslash, U+005c),
- xand two hexadecimal digits (like7F),
- uand four hexadecimal digits (forming a fixed Unicode escape sequence like\u005c),
- u{and at least one hexadecimal digit followed by}(forming a bounded Unicode escape sequence like\u{5c}),
    and
- any single character except digits from ‘1’ to ‘9’, and characters ‘x’, ‘u’, ‘CR’, and ‘LF’.
The examples are provided below:

1 lets1 ='Hello, world!'
2 lets2 = "Hello, world!"
3 lets3 = "\\"
4 lets4 = ""
5 lets5 = "don’t worry, be happy"
6 lets6 ='don\'t worry, be happy'
7 lets7 ='don\u0027t worry, be happy'

```
2.9. Literals 19
```

#### 2.9.7 Multiline String Literal

```
Multiline strings can contain arbitrary text delimited by backtick characters ‘`’. Multiline strings can contain any
character, except the escape character ‘\’. Multiline strings can contain newline characters:
```
```
MultilineStringLiteral:
'`'(BacktickCharacter)* '`'
;
```
```
BacktickCharacter:
~['\\\r\n]
|'\\' EscapeSequence
|LineContinuation
;
```
```
LineContinuation:
'\\'[\r\n\u2028\u2029]+
;
```
```
The grammar of embeddedExpression is described in String Interpolation Expressions.
An example of a multiline string is provided below:
```
1 letsentence = `This is an example of
2 a multiline string,
3 which should be enclosed in
4 backticks`

```
MultilineString literals are of the literal type that corresponds to a literal. If an operator is applied to a literal, then the
literal type is replaced forstring(see Type string ).
```
#### 2.9.8 NullLiteral

```
Null literal is the only literal of typenull(see Type null ) to denote a reference without pointing at any entity. The null
literal is represented by the keywordnull:
```
```
NullLiteral:
'null'
;
```
```
The value is typically used for types likeT | null(see Nullish Types ).
```
#### 2.9.9 UndefinedLiteral

```
Undefined literal is the only literal of typeundefined(see Type undefined ) to denote a reference with a value that is
not defined. The undefined literal is represented by the keywordundefined:
```
```
20 Chapter 2. Lexical Elements
```

```
UndefinedLiteral:
'undefined'
;
```
### 2.10 Comments

```
Comment is a piece of text added in the stream to document and compliment the source code. Comments are insignif-
icant for the syntactic grammar (see Grammar Summary ).
Line comments begin with the sequence of characters ‘//’ as in the example below, and end with the line separator
character. Any character or sequence of characters between them is allowed but ignored.
```
1 // This is a line comment

```
Multiline comments begin with the sequence of characters ‘\*’ as in the example below, and end with the first subsequent
sequence of characters ‘*/’. Any character or sequence of characters between them is allowed but ignored.
```
1 /*
2 This is a multiline comment
3 */

```
Comments cannot be nested.
```
### 2.11 Semicolons

```
Declarations and statements are usually terminated by a line separator (see Line Separators ). A semicolon must be
used in some cases to separate syntax productions written in one line or to avoid ambiguity.
```
1 functionfoo(x: number): number {
2 x++;
3 x *= x;
4 return x
5 }
6
7 leti = 1
8 i-i++ // one expression
9 i;-i++ // two expressions

```
2.10. Comments 21
```

**22 Chapter 2. Lexical Elements**


##### CHAPTER

### THREE

### TYPES

This chapter introduces the notion of type that is one of the fundamental concepts of ArkTS and other programming
languages. Type classification as accepted in ArkTS is discussed below along with all aspects of using types in programs
written in the language.

The type of an entity is conventionally defined as the set of _values_ the entity (variable) can take, and the set of _operators_
applicable to the entity of a given type.

ArkTS is a statically typed language. It means that the type of every declared entity and every expression is known at
compile time. The type of an entity is either set explicitly by a developer, or inferred implicitly (see _Type Inference_ ) by
the compiler.

The types integral to ArkTS are called _predefined types_ (see _Predefined Types_ ).

The types introduced, declared, and defined by a developer are called _user-defined types_. All _user-defined types_ must
have complete type declarations presented as source code in ArkTS.

ArkTS types are summarized in the table below:

```
Predefined Types User-Defined Types
byte,short, class types,
int,long, interface types,
float,double, array types,
number, fixed array types,
boolean,char, tuple types,
string, union types,
bigint, literal types,
Any,Object, function types,
never,void, type parameters
undefined,null, enumeration types
Array<T>orT[],
FixedArray<T>
```
**Note**. Typenumberis an alias todouble.

Most _predefined types_ have aliases to improve TypeScript compatibility as follows:

##### 23


```
Primary Name Alias
number Number
byte Byte
short Short
int Int
long Long
float Float
double Double
boolean Boolean
char Char
string String
bigint BigInt
Object object
```
Using primary names of _predefined types_ is recommended in all cases.

### 3.1 Predefined Types

Predefined types include the following:

- _Value Types_ ;
- _Type Any_ ;
- _Type Object_ ;
- _Type never_ ;
- _Type void_ ;
- _Type undefined_ ;
- _Type null_ ;
- _Type string_ ;
- _Type bigint_ ;
- _Array Types_ (Array<T>orT[]orFixedArray<T>).

### 3.2 User-Defined Types

_User-defined_ types include the following:

- Class types (see _Classes_ );
- Interface types (see _Interfaces_ );
- Enumeration types (see _Enumerations_ );

**24 Chapter 3. Types**


- _Function Types_ ;
- _Tuple Types_ ;
- _Union Types_ ;
- _Type Parameters_ ; and
- _Literal Types_.

### 3.3 Using Types

Source code can refer to a type by using the following:

- Type reference for:
    **-** _Named Types_ , or
    **-** Type aliases (see _Type Alias Declaration_ );
- In-place type declaration for:
    **-** _Array Types_ ,
    **-** _Tuple Types_ ,
    **-** _Function Types_ ,
    **-** _Function Types with Receiver_ ,
    **-** _Keyof Types_ ,
    **-** _Union Types_ , or
    **-** Type in parentheses.

The syntax of _type_ is presented below:

type:
annotationUsage?
(typeReference
|'readonly'?arrayType
|'readonly'?tupleType
|functionType
|functionTypeWithReceiver
|unionType
|keyofType
|StringLiteral
)
|'('type')'
;

The usage of annotations is discussed in _Using Annotations_.

Types with the prefixreadonlyare discussed in _Readonly Array Types_ and _Readonly Tuple Types_.

The usage of types is represented by the example below:

**3.3. Using Types 25**


1 letn: number // using identifier as a predefined value type name
2 leto: Object // using identifier as a predefined class type name
3 leta: number[] // using array type
4 lett: [number, number] // using tuple type
5 letf: ()=>number // using function type
6 letu: number|string // using union type
7 letl: "xyz" // using string literal type
8
9 class C { n = 1; s = "aa"}
10 letk: keyof C // using keyof to build union type

```
Parentheses are used to specify the required type structure if the type is a combination of array, function, or union
types. Without parentheses, the symbol ‘|’ that constructs a union type has the lowest precedence as represented by
the example below:
```
1 // a nullable array with elements of type string:
2 leta: string[] |null
3 lets: string[] = []
4 a = s // ok
5 a =null// ok, a is nullable
6
7 // an array with elements whose types are string or null:
8 letb1: (string |null)[]
9 b1 =null // error, b1 is an array and is not nullable
10 b1 = ["aa",null]// ok
11
12 // string or array of null elements:
13 letb2:string |null[]
14 b2 =null // error, b2 - string or array of nulls - not nullable
15 b2 = [null,null]// ok
16
17 // a function type that returns string or null
18 letc: () =>string |null
19 c =null// error, c is not nullable
20 c = ():string |null=> {return null} // ok
21
22 // (a function type that returns string) or null
23 letd: (() =>string) |null
24 d =null// ok, d is nullable
25 d = ():string => {return"hi" } // ok

```
If an annotation is used in front of type in parentheses, then the parentheses become a mandatory part of the annotation
to prevent ambiguity.
```
```
1 letvar_name1: @my_annotation() (A|B)// OK
2 letvar_name2: @my_annotation(A|B) // Compile-time error
```
```
26 Chapter 3. Types
```

### 3.4 Named Types

```
Named types are classes, interfaces, enumerations, aliases, type parameters, and predefined types (see Predefined
Types ), except built-in arrays. Other types (i.e., array, function, and union types) are anonymous unless aliased. Re-
spective named types are introduced by the following:
```
- Class declarations (see _Classes_ ),
- Interface declarations (see _Interfaces_ ),
- Enumeration declarations (see _Enumerations_ ),
- Type alias declarations (see _Type Alias Declaration_ ), and
- Type parameter declarations (see _Type Parameters_ ).
Classes, interfaces and type aliases with type parameters are _generic types_ (see _Generics_ ). Named types without type
parameters are _non-generic types_.
_Type references_ (see _Type References_ ) refer to named types by specifying their type names and (where applicable) type
arguments to be substituted for the type parameters of a named type.

### 3.5 Type References

```
Type reference refers to a type by one of the following:
```
- _Simple_ or _qualified_ type name (see _Names_ ),
- Type alias (see _Type Alias Declaration_ ).
_Type reference_ that refers to a generic class or to an interface type is valid if it is a valid instantiation of a generic. Its
type arguments can be provided explicitly or implicitly based on defaults.
The syntax of _type reference_ is presented below:

```
typeReference:
typeReferencePart('.'typeReferencePart)*
;
```
```
typeReferencePart:
identifier typeArguments?
;
```
1 letmap:Map<string,number> // Map<string, number> is the type reference
2
3 class A<T> {...}
4 class C<T> {
5 field1:A<T> // A<T> is a class type reference - class type reference
6 field2:A<number>// A<number> is a type reference - class type reference
7 foo (p:T) {}// T is a type reference - type parameter
8 constructor() {/* some body to init fields */ }
9 }
10
11 typeMyType<T> = A<T>[]
(continues on next page)

```
3.4. Named Types 27
```

```
(continued from previous page)
```
12 letx: MyType<number> = [newA<number>,newA<number>]
13 // MyType<number> is a type reference - alias reference
14 // A<number> is a type reference - class type reference

```
If type reference refers to a type by a type alias (see Type Alias Declaration ), then the type alias is replaced for a
non-aliased type in all cases when dealing with types. The replacement is potentially recursive.
```
```
1 typeT1 = Object
2 typeT2 =number
3 functionfoo(t1:T1, t2: T2) {
4 t1 = t2 // Type compatibility test will use Object and number
5 t2 = t2 + t2// Operator validity test will use type number not T2
6 }
```
### 3.6 Value Types

```
Value types are predefined integer types (see Integer Types and Operations ), floating-point types (see Floating-Point
Types and Operations ), the boolean type (see Type boolean ), character types (see Type char ), and user-defined enumer-
ation types (see Enumerations ). The values of such types do not share state with other values.
```
#### 3.6.1 Numeric Types

```
Numeric types are integer and floating-point types (see Integer Types and Operations and Floating-Point Types and
Operations ).
Larger type values include all values of smaller types:
```
- double>float>long>int>short>byte
A value of a smaller type can be assigned to a variable of a larger type as a consequence (see _Widening Numeric
Conversions_ ).
In terms of operations available for the numeric types (see _Multiplication_ , _Division_ , _Remainder_ , _Additive Expressions_ )
we state thatnumberordoubleis the largest type andlongis larger thanintand so on respectively.
Typebigintdoes not belong to this hierarchy. No implicit conversion from numeric types (see _Numeric Types_ ) to
bigintoccurs in any assignment context (see _Assignment-like Contexts_ ). The methods of classBigInt(which is a
part of _Standard Library_ ) must be used to createbigintvalues from numeric type values.

```
28 Chapter 3. Types
```

#### 3.6.2 Integer Types and Operations

```
Type Corresponding Set of Values
byte All signed 8-bit integers (− 27 to 27 − 1 )
short All signed 16-bit integers (− 215 to 215 − 1 )
int All signed 32-bit integers (− 231 to 231 − 1 )
long All signed 64-bit integers (− 263 to 263 − 1 )
bigint All integers with no limits
```
ArkTS provides a number of operators to act on integer values as discussed below.

- Comparison operators that produce a value of typeboolean:
    **-** Numeric relational operators ‘<’, ‘<=’, ‘>’, and ‘>=’ (see _Numeric Relational Operators_ );
    **-** Numeric equality operators ‘==’ and ‘!=’ (see _Numeric Equality Operators_ );
- Numeric operators that produce values of typesint,long, orbigint:
    **-** Unary plus ‘+’ and minus ‘-’ operators (see _Unary Plus_ and _Unary Minus_ );
    **-** Multiplicative operators ‘*’, ‘/’, and ‘%’ (see _Multiplicative Expressions_ );
    **-** Additive operators ‘+’ and ‘-’ (see _Additive Expressions_ );
    **-** Increment operator ‘++’ used as prefix (see _Prefix Increment_ ) or postfix (see _Postfix Increment_ );
    **-** Decrement operator ‘--’ used as prefix (see _Prefix Decrement_ ) or postfix (see _Postfix Decrement_ );
    **-** Signed and unsigned shift operators ‘<<’, ‘>>’, and ‘>>>’ (see _Shift Expressions_ );
    **-** Bitwise complement operator ‘~’ (see _Bitwise Complement_ );
    **-** Integer bitwise operators ‘&’, ‘^’, and ‘|’ (see _Integer Bitwise Operators_ );
- Ternary conditional operator ‘? :‘ (see _Ternary Conditional Expressions_ );
- String concatenation operator ‘+’ (see _String Concatenation_ ) that, if one operand isstringand the other is of
    an integer type, converts the integer operand tostringwith the decimal form, and then creates a concatenation
    of the two strings as a newstring.

If either operand of a binary integer operation except _Shift Expressions_ is of typelongand the other operand is of a
lesser type, then numeric conversion (see _Widening Numeric Conversions_ ) is used to widen the second operand first to
typelong. In this case:

- Operation implementation uses 64-bit precision; and
- Result of the numeric operator is of typelong.

If otherwise neither operand is of typelongand any operand is of a type other thanint, then numeric conversion is
used to widen the latter first to typeint. In this case:

- Operation implementation uses 32-bit precision; and
- Result of the numeric operator is of typeint.

Conversions between integer types and typebooleanare not allowed. However, the value of integer type can be used
as a logical condition in some cases (see _Extended Conditional Expressions_ )

The integer operators cannot indicate an overflow or an underflow.

An integer operator can throwArithmeticErrorif the right-hand-side operand of an integer division operator ‘/’ (see
_Division_ ) and an integer remainder operator ‘%’ (see _Remainder_ ) is zero. The situation is discussed in _Error Handling_.

Predefined constructors, methods, and constants for _integer types_ are parts of the ArkTS _Standard Library_.

**3.6. Value Types 29**


#### 3.6.3 Floating-Point Types and Operations

```
Type Corresponding Set of Values
float The set of all IEEE 754^3 32-bit floating-point numbers
number,double The set of all IEEE 754 64-bit floating-point numbers
```
ArkTS provides a number of operators to act on floating-point type values as discussed below.

- Comparison operators that produce a value of type _boolean_ :
    **-** Numeric relational operators ‘<’, ‘<=’, ‘>’, and ‘>=’ (see _Numeric Relational Operators_ );
    **-** Numeric equality operators ‘==’ and ‘!=’ (see _Numeric Equality Operators_ );
- Numeric operators that produce values of typefloatordouble:
    **-** Unary plus ‘+’ and minus ‘-’ operators (see _Unary Plus_ and _Unary Minus_ );
    **-** Multiplicative operators ‘*’, ‘/’, and ‘%’ (see _Multiplicative Expressions_ );
    **-** Additive operators ‘+’ and ‘-’ (see _Additive Expressions_ );
    **-** Increment operator ‘++’ used as prefix (see _Prefix Increment_ ) or postfix (see _Postfix Increment_ );
    **-** Decrement operator ‘--’ used as prefix (see _Prefix Decrement_ ) or postfix (see _Postfix Decrement_ );
- Numeric operators that produce values of typeintorlong:
    **-** Signed and unsigned shift operators ‘<<’, ‘>>’, and ‘>>>’ (see _Shift Expressions_ );
    **-** Bitwise complement operator ‘~’ (see _Bitwise Complement_ );
    **-** Integer bitwise operators ‘&’, ‘^’, and ‘|’ (see _Integer Bitwise Operators_ );
- Ternary conditional operator ‘? :‘ (see _Ternary Conditional Expressions_ );
- The string concatenation operator ‘+’ (see _String Concatenation_ ) that, if one operand is of typestringand the
    other is of a floating-point type, converts the floating-point type operand to typestringwith a value represented
    in the decimal form (without loss of information), and then creates a concatenation of the two strings as a new
    string.

An operation is called a _floating-point operation_ if at least one of the operands in a binary operator is of a floating-point
type (even if the other operand is integer), and that is not a string concatenation.

If at least one operand of the numeric operator is of typedouble, then the operation implementation uses the 64-bit
floating-point arithmetic. The result of the numeric operator is a value of typedouble.

If the other operand is not of typedouble, then the numeric conversion (see _Widening Numeric Conversions_ ) is used
to widen the operand first to typedouble.

If neither operand is of typedouble, then the operation implementation is to use the 32-bit floating-point arithmetic.
The result of the numeric operator is a value of typefloat.

If the other operand is not of typefloat, then the numeric conversion is used to widen the operator first to typefloat.

Any floating-point type value can be cast to or from any numeric type (see _Numeric Types_ ).

Conversions between floating-point types and typebooleanare not allowed. However, the value of floating-point type
can be used as a logical condition in some cases (see _Extended Conditional Expressions_ )

(^3) Any mention of IEEE 754 in this Specification refers to the latest revision of “754-2019–IEEE Standard for Floating-Point Arithmetic”.
**30 Chapter 3. Types**


Operators on floating-point numbers, except the remainder operator (see _Remainder_ ), behave in compliance with the
IEEE 754 Standard. For example, ArkTS requires the support of IEEE 754 _denormalized_ floating-point numbers and
_gradual underflow_ which facilitate proving the desirable properties of a particular numeric algorithm. Floating-point
operations do not _flush to zero_ if the calculated result is a denormalized number.

ArkTS requires the floating-point arithmetic to behave as if the floating-point result of every floating-point operator is
rounded to the result precision. An _inexact_ result is rounded to a representable value nearest to the infinitely precise
result. ArkTS uses the _round to nearest_ principle (the default rounding mode in IEEE 754), and prefers the representable
value with the least significant bit zero out of any two equally near representable values.

ArkTS uses _round toward zero_ to convert a floating-point value to an integer value (see _Numeric Casting Conversions_ ).
In this case it acts as if the number is truncated, and the mantissa bits are discarded. The result of _rounding toward zero_
is the value of the format that is closest to and no greater in magnitude than the infinitely precise result.

A floating-point operation with overflow produces a signed infinity.

A floating-point operation with underflow produces a denormalized value or a signed zero.

A floating-point operation with no mathematically definite result producesNaN.

All numeric operations with aNaNoperand result inNaN.

Predefined constructors, methods, and constants for _floating-point types_ are parts of the ArkTS _Standard Library_.

#### 3.6.4 Typeboolean.

Typebooleanrepresents logical valuestrueandfalse.

The boolean operators are as follows:

- Equality operators (see _Equality Expressions_ );
- Logical complement operator ‘!’ (see _Logical Complement_ );
- Logical operators ‘&’, ‘^’, and ‘|’ (see _Boolean Logical Operators_ );
- Conditional-and operator ‘&&’ (see _Conditional-And Expression_ ) and conditional-or operator ‘||’ (see
    _Conditional-Or Expression_ );
- Ternary conditional operator ‘? :‘ (see _Ternary Conditional Expressions_ );
- String concatenation operator ‘+’ (see _String Concatenation_ ) that converts an operand of typebooleanto type
    string(trueorfalse), and then creates a concatenation of the two strings as a newstring.

### 3.7 Reference Types

_Reference types_ can be of the following kinds:

- _Class_ types (see _Type Object_ and _Classes_ );
- _Interface_ types (see _Interfaces_ );
- _Array Types_ ;
- _Fixed-Size Array Types_ ;

**3.7. Reference Types 31**


- _Tuple Types_ ;
- _Function Types_ ;
- _Union Types_ ;
- _Literal Types_ ;
- _Type Any_ ;
- _Type string_ ;
- _Type bigint_ ;
- _Type never_ ;
- _Type null_ ;
- _Type undefined_ ;
- _Type void_ ; and
- _Type Parameters_.

### 3.8 TypeAny

TypeAnyis a predefined type which is the supertype of all types. TypeAnyis a predefined _nullish-type_ (see _Nullish
Types_ ), i.e., a supertype of _Type void_ and _Type null_ in particular.

TypeAnyhas no methods or fields.

### 3.9 TypeObject

TypeObjectis a predefined class type which is the supertype (see _Subtyping_ ) of all types except _Type void_ , _Type
undefined_ , _Type null_ , _Nullish Types_ , _Type Parameters_ , and _Union Types_ that contain type parameters. All subtypes of
Objectinherit the methods of classObject(see _Inheritance_ ). All methods of classObjectare described in full in
_Standard Library_.

The methodtoStringused in the examples in this document returns a string representation of the object.

The term _object_ is used in the Specification to refer to an instance of any type.

Pointers to objects are called _references_. Multiple references to an object are possible.

Objects can have states. A state of an object that is a class instance is stored in its fields. A state of an array or tuple
object is stored in its elements.

If two variables of any type except _Value Types_ contain references to the same object, and the state of that object is
modified in the reference of either variable, then the state so modified can be seen in the reference of the other variable.

**32 Chapter 3. Types**


### 3.10 Typenever.

```
Typeneveris assignable to any type (see Assignability ).
Typeneverhas no instance. Typeneveris used as one of the following:
```
- Return type for functions or methods that never return a value, but throw an error when completing an operation.
- Type of variables that never get a value (however, an assignment statement with typeneverin both left-hand
    and right-hand sides is valid).
- Type of parameters of a function or a method to prevent the body of that function or method from being executed.

1 functionfoo (): never {
2 throw newError("foo() never returns")
3 }
4
5 letx: never= foo()// x will never get a value
6
7 functionbar (p:never) {// body of this
8 // function will never be executed
9 }
10
11 bar (foo())// neither foo nor bar are executed

### 3.11 Typevoid

```
Typevoidis used as a return type to highlight that a function, a method, or a lambda can contain return Statements
with no expression, or no return statement at all:
```
1 functionfoo ():void{} // no return at all
2
3 class C {
4 bar(): void{
5 return // with no expression
6 }
7 }
8
9 typeFunctionWithNoParametersType = () => void
10
11 letfuncTypeVariable:FunctionWithNoParametersType = ():void => {}

```
A compile-time error occurs if:
```
- Typevoidis used as type annotation;
- Expression of typevoidis used as a value.
Typevoidhas no instance by itself. However, that it is a supertype of typeundefined(see _Type undefined_ ) affects
the _Assignability_ as follows:

```
3.10. Type never 33
```

1 letx: void=undefined // compile-time error - void used as type annotation
2
3 functionfoo ():void{}
4 console.log (foo()) // compile-time error - void used as a value
5
6 functionbar1 ():void {
7 return void // compile-time error - void used as a value
8 }
9
10 functionbar2 ():void {
11 return undefined // OK as undefined is a subtype of void
12 }
13
14 typeaType =void| number// compile-time error - void used as type annotation

```
Typevoidcan be used as a type argument that instantiates a generic type, function, or method as follows:
```
1 classA<T> {
2 f:T
3 m(): T {return this.f }
4 constructor(f:T) {this.f = f }
5 }
6 leta1 =newA<void>(undefined) // ok, as undefined is a subtype of void
7 leta2 =newA<undefined>(undefined) // ok
8 leta3 =newA<void>(void) // compile-time error: void is used as value
9
10 console.log (a1.f, a2.m())// Output is "undefined" "undefined"
11
12 functionfoo<T>(p:T): T {return p }
13 foo<void>(undefined) // ok, it returns'undefined'value
14 foo<void>(void) // compile-time error: void is used as value
15
16 typeF1<T> = () => T
17 constf1:F1<void> = ():void=> {}
18 constf2:F1<void> = () => {}
19 constf3:F1<void> = ():undefined=> {return undefined}
20
21 // Array literals can be assigned to the array of void type in any form
22 typeA1<T> = T[]
23 typeA2<T> = Array<T>
24 consta1:A1<void> = [undefined]
25 consta2:A2<void> = [undefined, undefined]
26
27 letx: void[]// compile-time error - void used as type annotation

```
34 Chapter 3. Types
```

### 3.12 Typeundefined.

```
The only value of typeundefinedis the literalundefined(see Undefined Literal ).
Typeundefinedis a subtype of typevoid(see Type void ).
Using typeundefinedas type annotation is not recommended, except in nullish types (see Nullish Types ).
Typeundefinedcan be used also as type argument to instantiate a generic type as follows:
```
1 classA<T> {}
2 leta =newA<undefined>()// ok, type parameter is irrelevant
3 functionfoo<T>(x:T) {}
4
5 foo<undefined>(undefined)// ok

### 3.13 Typenull

```
The only value of typenullis the literalnull(see Null Literal ).
Using typenullas type annotation is not recommended, except in nullish types (see Nullish Types ).
```
### 3.14 Typestring

```
Typestringvalues are all string literals, e.g., ‘abc’. Typestringstores sequences of characters as Unicode UTF-16
code units.
Astringobject is immutable, the value of astringobject cannot be changed after the object is created. The value
of astringobject can be shared.
Typestringhas dual semantics, i.e.:
```
- Typestringbehaves like a reference type (see _Reference Types_ ) if created, assigned, or passed as an argument;
- Typestringis handled as a value (see _Value Types_ ) by allstringoperations (see _String Concatenation_ , _Equal-_
    _ity Expressions_ , and _String Relational Operators_ ).
A number of operators can act onstringvalues as follows:
- Accessing thelengthproperty returns string length asinttype value. String length is a non-negative integer
number. String length is set once at runtime and cannot be changed after that.
- Concatenation operator ‘+’ (see _String Concatenation_ ) produces a value of typestring. If the result is not a
constant expression (see _Constant Expressions_ ), then the string concatenation operator can implicitly create a
newstringobject;
- Indexing a string value (see _String Indexing Expression_ ) returns a value of typestring. A newstringobject
can be created implicitly.
A string value can contain any character, i.e., no character can be used to indicate the end of a string. A character with
the value ‘0’ is an ordinary character inside a string as represented by the following example:

```
3.12. Type undefined 35
```

1 console.log("a\0b".length)// output: 3

```
Usingstringin all cases is recommended, although the nameStringalso refers to typestring.
```
### 3.15 Typebigint

```
ArkTS has the built-inbiginttype that allows handling theoretically arbitrary large integers. Values of typebigint
can hold numbers that are larger than the maximum value of typelong. Typebigintuses the arbitrary-precision
arithmetic. Values of typebigintcan be created from the following:
```
- _Bigint literals_ (see _Bigint Literals_ ); or
- Numeric type values, by using a call to the standard library classBigIntmethods or constructors (see _Standard_
    _Library_ ).
Similarly tostring,biginttype has dual semantics:
- If created, assigned, or passed as an argument, typebigintbehaves like a reference type (see _Reference Types_ ).
- All applicable operations handle typebigintas a value type (see _Value Types_ ). The operations are described
in _Integer Types and Operations_.
Usingbigintis recommended in all cases, although the nameBigIntalso refers to typebigint. UsingBigInt
creates new objects and calls to static methods in order to improve TypeScript compatibility.

1 letb1:bigint= newBigInt(5)// for Typescript compatibility
2 letb2:bigint= 123n

### 3.16 Literal Types

```
Literal types are aligned with some ArkTS literals (see Literals ). Their names are the same as the names of their values,
i.e., literals proper. ArkTS supports only the following literal types:
```
- _String Literal Types_ ,
- null, and
- undefined.

1 leta: "string literal" = "string literal"
2 letb: null=null
3 letc: undefined= undefined
4
5 printThem (a, b, c)
6 functionprintThem (p1: "string literal", p2: null, p3:undefined) {
7 console.log (p1, p2, p3)
8 }

```
There are no operations for literal typesnullandundefined.
```
```
36 Chapter 3. Types
```

#### 3.16.1 String Literal Types

```
Operations on variables of string literal types are identical to the operations of their supertypestring(see Type string ).
The resulting operation type is the type specified for the operation in the supertype:
```
1 lets0: "string literal" = "string literal"
2 lets1:string = s0 + s0 // + for string returns string

### 3.17 Array Types

```
Array type is a data structure intended to comprise any number of same-type elements, including zero elements. ArkTS
supports the following two predefined array types:
```
- _Resizable Array Types_ ; and
- _Fixed-Size Array Types_ as an experimental feature.
_Resizable array types_ are recommended for most cases. _Fixed-size array types_ can be used where performance is the
major requirement.
_Fixed-size arrays_ differ from _resizable arrays_ as follows:
- _Fixed-size arrays_ have their length set only once to achieve a better performance.
- _Fixed-Size arrays_ have no methods defined.
**Note**. The term _array type_ as used in this Specification applies to both _resizable array type_ and _fixed-size array type_.
The same holds true for _array value_ and _array instance_. _Resizable arrays_ and _fixed-size arrays_ are not assignable to
each other.

#### 3.17.1 Resizable Array Types

```
Resizable array type is a built-in type characterized by the following:
```
- Any object of resizable array type contains elements. The number of elements is known as _array length_ , and can
    be accessed by using thelengthproperty.
- Array length is a non-negative integer number.
- Array length can be set and changed at runtime.
- Array element is accessed by its index. The index is an integer number in the range from _0_ to _array length minus_
    _1_.
- Accessing an element by its index is a constant-time operation.
- If passed to non-ArkTS environment, an array is represented as a contiguous memory location.
- Type of each array element is assignable to the element type specified in the array declaration (see _Assignability_ ).

```
3.17. Array Types 37
```

```
Resizable array type with elements of typeTcan have the following two forms of syntax:
```
- T[], and
- Array<T>.
The first form uses the following syntax:

```
arrayType:
type'[' ']'
;
```
```
Note .T[]andArray<T>specify identical, i.e., indistinguishable types (see Type Identity ).
Two basic operations with array elements take elements out of, and put elements into an array by using the operator
‘[]’.
The same syntax can be used to work with Indexable Types , some of such types are parts of Standard Library.
The number of elements in an array can be obtained by accessing the propertylength. The length of an array can be
set and changed in runtime using the methods defined in Standard Library.
An array can be created by using Array Literal , Resizable Array Creation Expressions , or the constructors defined in
Standard Library.
ArkTS allows setting a new value tolengthto shrink an array and provide better TypeScript compatibility. An error
is caused by the following situations:
```
- The value is of typenumberor other floating-point type, and the fractional part differs from 0;
- The value is less then zero; or
- The value is greater then previous length.
The above situations cause errors as follows:
- A runtime error, if the situation is identified at runtime, i.e., during program execution; and
- A compile-time error, if the situation is detected during compilation.
Array operations are illustrated below:

1 leta :number[] = [0, 0, 0, 0, 0]
2 /* allocate array with 5 elements of type number */
3 a[1] = 7/* put 7 as the 2nd element of the array, index of this element is 1 */
4 lety = a[4]/* get the last element of array 'a'*/
5 letcount = a.length// get the number of array elements
6 a.length = 3// shrink array
7 y = a[2]// OK, 2 is the index of the last element now
8 y = a[3]// Will lead to runtime error - attempt to access non-existing array element
9
10 letb: Array<number> = a// 'b'points to the same array as'a'

```
A type alias can set a name for an array type (see Type Alias Declaration ):
```
```
1 typeMatrix =number[][]/* array or array of numbers */
```
```
An array as an object is assignable to a variable of typeObject:
```
```
1 leta: number[] = [1, 2, 3]
2 leto: Object= a
```
```
38 Chapter 3. Types
```

#### 3.17.2 Readonly Array Types

```
Readonly array type is immutable, i.e.:
```
- Length of a variable of a _readonly array type_ cannot be changed;
- Elements of a _readonly array type_ cannot be modified after the initial assignment directly nor through a function
    or method call.
Otherwise, a compile-time error occurs.

1 letx: readonly number [] = [1, 2, 3]
2 x[0] = 42 // compile-time error as array itself is readonly

```
Readonly array type with elements of typeTcan have the following two syntax forms:
```
- readonly T[], and
- ReadonlyArray<T>.
Both forms specify identical (indistinguishable) types (see _Type Identity_ ).
**Note.** In arrays of arrays, all arrays arereadonly.

### 3.18 Tuple Types

```
Tuple type is a reference type created as a fixed set of other types.
The syntax of tuple type is presented below:
```
```
tupleType:
'[' (type(','type)* ','?)?']'
;
```
```
The value of a tuple type is a group of values of types that comprise the tuple type. The number of values in the group
equals the number of types in a tuple type declaration. The order of types in a tuple type declaration specifies the type
of the corresponding value in the group.
It implies that each element of a tuple has its own type. The operator ‘[]’ (square brackets) is used to access the
elements of a tuple in a manner similar to accessing the elements of an array.
An index expression must be of integer type. The index of the first tuple element is 0. Only constant expressions can
be used as the index providing access to tuple elements:
```
1 lettuple: [number, number,string,boolean, Object] =
2 [ 6, 7, "abc", true, 42]
3 tuple[0] = 42
4 console.log (tuple[0], tuple[4]) //`42 42`be printed

```
A tuple does not have length property so the legal TypeScript code like below issues compile-time error in ArkTS:
```
```
3.18. Tuple Types 39
```

1 lettuple : [number, string] = [1, "" ]
2 for(letindex = 0; index < tuple.length; index++ ) { // compile-time error
3 // no'length'property
4 letelement:Object= tuple[index]
5 // do something with the element
6 }

```
Any tuple type is assignable (see Assignability ) to classObject(see Type Object ).
An empty tuple is a corner case. It is only added to support TypeScript compatibility:
```
1 letempty: [] = []// empty tuple with no elements in it

#### 3.18.1 Readonly Tuple Types

```
If an tuple type has the prefixreadonly, then its elements cannot be modified after the initial assignment directly or
through a function or method call. Otherwise, a compile-time error occurs as follows:
```
1 letx: readonly[number,string] = [1, "abc"]
2 x[0] = 42 // compile-time error as tuple itself is readonly

### 3.19 Function Types

```
Function type can be used to express the expected signature of a function. A function type consists of the following:
```
- Optional type parameters;
- List of parameters (which can be empty);
- Optional return type.
The syntax of _function type_ is as follows:

```
functionType:
'(' ftParameterList? ')' ftReturnType
;
```
```
ftParameterList:
ftParameter(',' ftParameter)* (','ftRestParameter)?
|ftRestParameter
;
```
```
ftParameter:
identifier('?')? ':'type
;
```
```
ftRestParameter:
(continues on next page)
```
```
40 Chapter 3. Types
```

```
(continued from previous page)
'...'ftParameter
;
```
```
ftReturnType:
'=>'type
;
```
```
Therestparameter is described in Rest Parameter.
```
```
1 letbinaryOp: (x:number, y:number) =>number
2 functionevaluate(f: (x: number, y: number) =>number) { }
```
```
A type alias can set a name for a function type (see Type Alias Declaration ):
```
```
1 typeBinaryOp = (x: number, y:number) =>number
2 letop:BinaryOp
```
```
If a function type has the ‘?’ mark for a parameter name, then this parameter and all parameters that follow (if any) are
optional. Otherwise, a compile-time error occurs. The actual type of the parameter is then a union of the parameter
type and typeundefined. This parameter has no default value.
```
1 typeFuncTypeWithOptionalParameters = (x?:number, y?:string) =>void
2 letfoo:FuncTypeWithOptionalParameters
3 = ():void=> {} // OK: as arguments are just ignored
4 foo = (p: number):void => {}// CTE as call with zero arguments is invalid
5 foo = (p?: number):void=> {}// OK: as call with zero or one argument is valid
6 foo = (p1: number, p2?: string):void=> {}// Compile-time error: as call with zero␣
˓→arguments is invalid
7 foo = (p1?:number, p2?: string):void=> {}// OK
8
9 foo()
10 foo(undefined)
11 foo(undefined, undefined)
12 foo(42)
13 foo(42,undefined)
14 foo(42, "a string")
15
16 typeIncorrectFuncTypeWithOptionalParameters = (x?:number, y:string) =>void
17 // compile-time error: no mandatory parameter can follow an optional parameter
18
19 functionbar (
20 p1?:number,
21 p2: number|undefined
22 ) {
23 p1 = p2// OK
24 p2 = p1// OK
25 // Types of p1 and p2 are identical
26 }

```
More details on function types assignability are provided in Subtyping for Function Types.
```
```
3.19. Function Types 41
```

#### 3.19.1 TypeFunction.

```
TypeFunctionis a predefined type that is a direct superinterface of any function type.
A value of typeFunctioncannot be called directly. A developer must use theunsafeCallmethod instead. This
method checks the arguments of typeFunction, and calls the underlying function value if the number and types of
the arguments are valid.
```
```
1 functionfoo(n: number) {}
2
3 letf: Function= foo
4
5 f(1)// compile-time error: cannot be called
6
7 f.unsafeCall(3.14)// correct call and execution
8 f.unsafeCall()// runtime error: wrong number of arguments
```
```
Another important property of typeFunctionisname. It is a string that contains the name associated with the function
object in the following way:
```
- If a function or a method is assigned to a function object, then the associated name is that of the function or of
    the method;
- If a lambda is assigned to a variable ofFunctiontype, then the associated name is that of the variable;
- Otherwise, the string is empty.

1 functionprint_name (f: Function) {
2 console.log (f.name)
3 }
4
5 functionfoo() {}
6 print_name (foo)// output: "foo"
7
8 classA {
9 staticsm() {}
10 m() {}
11 }
12 print_name (A.sm) // output: "sm"
13 print_name (newA().m)// output: "m"
14
15 letx: Function= ():void=> {}
16 print_name (x)// output: "x"
17
18 lety = x
19 print_name (y)// output: "x"
20
21 print_name (():void=>{}) // output: ""

```
The declarations of theunsafeCallmethod,nameproperty, and all other methods and properties of typeFunction
are included in the ArkTS Standard Library.
```
```
42 Chapter 3. Types
```

### 3.20 Union Types

```
Union type is a reference type created as a combination of other types.
The syntax of union type is as follows:
```
```
unionType:
type('|'type)*
;
```
```
The values of a union type are valid values of all types the union is created from.
A compile-time error occurs if the type in the right-hand side of a union type declaration leads to a circular reference.
Typical usage examples of union types are represented below:
```
1 typeOperationResult = "Done" | "Not done"
2 functiondo_action(): OperationResult {
3 if (someCondition) {
4 return "Done"
5 }else{
6 return "Not done"
7 }
8 }
9
10 classCat {
11 // ...
12 }
13 classDog {
14 // ...
15 }
16 classFrog {
17 // ...
18 }
19 typeAnimal = Cat | Dog | Frog | number
20 // Cat, Dog, and Frog are some types (class type or interface type)
21
22 letanimal:Animal= newCat()
23 animal =newFrog()
24 animal = 42
25 // One may assign the variable of the union type with any valid value
26
27 enum StringEnum {One = "One", Two = "Two"}
28
29 typeUnion1 =string | StringEnum// OK, will be reduced during normalization

```
Values of particular types can be received from a union by using different mechanisms as follows:
```
```
1 class Cat { sleep () {}; meow () {} }
2 class Dog { sleep () {}; bark () {} }
3 class Frog { sleep () {}; leap () {} }
4
5 typeAnimal = Cat | Dog | Frog
6
7 letanimal:Animal =newCat()
(continues on next page)
```
```
3.20. Union Types 43
```

(continued from previous page)
8 if (animalinstanceof Frog) {
9 // animal is of type Frog here, conversion can be used:
10 letfrog:Frog = animalasFrog
11 frog.leap()
12 }
13
14 animal.sleep () // Any animal can sleep

```
Predefined types are represented by the following example:
```
```
1 typePredefined =number |boolean
2 letp: Predefined= 7
3 if (p instanceof number) {
4 // type of'p' is number here
5 }
```
```
Literal types are represented by the following example:
```
```
1 typeBMW_ModelCode = "325" | "530" | "735"
2 letcar_code:BMW_ModelCode = "325"
3 if (car_code == "325"){
4 car_code = "530"
5 } else if(car_code == "530"){
6 car_code = "735"
7 } else{
8 // pension :-)
9 }
```
```
Note. A compile-time error occurs if an expression of a union type is compared to a literal value or a constant that does
not belong to the values of the union type:
```
1 typeBMW_ModelCode = "325" | "530" | "735"
2 letcar_code:BMW_ModelCode = "325"
3 if (car_code == "234"){ ... }
4 /*
5 compile-time error as "234" does not belong to
6 values of literal type BMW_ModelCode
7 */
8
9 functionmodel_code_test (code: string) {
10 if (car_code == code) { ... }
11 // This test is to be resolved during program execution
12 }

```
44 Chapter 3. Types
```

#### 3.20.1 Union Types Normalization

```
Union types normalization allows minimizing the number of types within a union type, while keeping type safety.
Some types can also be replaced for more general types.
Union typeT 1 | ... |TN, whereN> 1, can be formally reduced to typeU 1 | ... |UM, whereM<=N, or even to a non-union
type V. In this latter case V can be a predefined value type or a literal type.
The normalization process presumes that the following steps are performed one after another:
```
1. All nested union types are linearized.
2. All type aliases (if any and except recursive ones) are recursively replaced for non-alias types.
3. Identical types within a union type are replaced for a single type with account to thereadonlytype flag priority.
4. If at least one type in a union isAny, then all other types are removed.
5. If positioned among union types, typeneveris removed.
6. If one type in a union isstring, then all string literal types (if any) are removed.
    This procedure is performed recursively until none of the above steps can can be performed again.
The normalization process results in a normalized union type. The process is represented by the examples below:

1 ( T1 | T2) | (T3 | T4) // normalized as T1 | T2 | T3 | T4. Linearization
2
3 typeA = A[] | string // No changes. Recursive type alias is kept
4
5 typeB =number
6 typeC =string
7 typeD = B | C // normalized as number | string. Type aliases are unfolded
8
9 number |number // normalized as number. Identical types elimination
10
11 (number[]) | (readonly number[])// normalized as readonly number[]. Readonly version␣
˓→wins
12
13 "1" | string| number// normalized as string | number. Literal type value belongs to␣
˓→another type values
14
15 class Base {}
16 class DerivedextendsBase {}
17 Base | Derived // normalized as Base | Derived (no change)

```
The ArkTS compiler applies normalization while processing union types and handling type inference for array literals
(see Array Type Inference from Types of Elements ).
```
#### 3.20.2 Access to Common Union Members

```
Whereuis a variable of union typeT 1 | ... |TN, ArkTS supports access to a common member ofu.mif the following
conditions are fulfilled:
```
- EachTiis an interface or class type;
- EachTihas a non-static member with the namem; and

```
3.20. Union Types 45
```

- For anyTi,mis one of the following:
    **-** Method or accessor with an equal signature; or
    **-** Same-type field.
Otherwise, a compile-time error occurs as follows:

1 class A {
2 n = 1
3 s = "aa"
4 foo() {}
5 goo(n: number) {}
6 static foo () {}
7 }
8 class B {
9 n = 2
10 s = 3.14
11 foo() {}
12 goo() {}
13 static foo () {}
14 }
15
16 letu: A| B = newA
17
18 letx = u.n// ok, common field
19 u.foo()// ok, common method
20
21 console.log(u.s)// compile-time error as field types differ
22 u.goo()// compile-time error as signatures differ
23
24 typeAB = A | B
25 AB.foo()// compile-time error as foo() is a static method

```
A compile-time error occurs if in someTithe namemis overloaded (see Overloading ):
```
1 class C {
2 overload foo { foo1, foo2 }
3 foo1(a:number): void{}
4 foo2(a:string): void{}
5 }
6 class D {
7 foo(a: number): void{}
8 foo2(a:string): void{}
9 }
10
11 functiontest(x:C | D) {
12 x.foo()// compile-time error, as'foo'in C is the overload alias
13 x.foo2("aa")// ok, as'foo2'in both C and D is a method
14 }

```
46 Chapter 3. Types
```

#### 3.20.3 KeyofTypes

```
Keyoftype is a special form of a union type that is built by using the keywordkeyof. The keywordkeyofis applied
to a class or an interface type (see Classes and Interfaces ). The resultant new type is a union of names (as string literal
types) of all accessible members (see Accessible ) of the class or the interface type.
The syntax ofkeyoftype is presented below:
```
```
keyofType:
'keyof'typeReference
;
```
```
A compile-time error occurs iftypeReferenceis neither a class nor an interface type. The semantics of typekeyof
is represented by the example below:
```
1 class A {
2 field: number
3 method() {}
4 }
5 typeKeysOfA = keyof A // "field" | "method"
6 leta_keys:KeysOfA= "field"// OK
7 a_keys = "any string different from field or method"
8 // Compile-time error: invalid value for the type KeysOfA

```
If a class or an interface is empty, then its typekeyofis equivalent to typenever:
```
1 class A {}// Empty class
2 typeKeysOfA = keyof A // never

### 3.21 Nullish Types

```
ArkTS has nullish types that are in fact a specific form of union types (see Union Types ).
T | nullorT | undefinedorT | undefined | nullcan be used as the type to specify a nullish version of type
T.
All predefined types except Type Any , and all user-defined types are non-nullish types. Non-nullish types cannot have
anullorundefinedvalue at runtime.
A variable declared to have typeT | nullcan hold the values of typeTand its derived types, or the valuenull. Such
a type is called a nullable type.
A variable declared to have typeT | undefinedcan hold the values of typeTand its derived types, or the value
undefined.
A variable declared to have typeT | null | undefinedcan hold values of typeTand its derived types, and the
valuesundefinedornull.
Nullish type is a reference type (see Union Types ). A reference that isnullorundefinedis called a nullish value.
An operation that is safe with no regard to the presence or absence of nullish values (e.g., re-assigning one nullable
value to another) can be used ‘as is’ for nullish types.
The following nullish-safe options exist for dealing with nullish typeT:
```
```
3.21. Nullish Types 47
```

- Using safe operations:
    **-** Safe method call (see _Method Call Expression_ for details);
    **-** Safe field access expression (see _Field Access Expression_ for details);
    **-** Safe indexing expression (see _Indexing Expressions_ for details);
    **-** Safe function call (see _Function Call Expression_ for details);
- Converting fromT | nullorT | undefinedtoT:
    **-** _Cast Expression_ ;
    **-** Ensure-not-nullish expression (see _Ensure-Not-Nullish Expression_ for details);
- Supplying a value to be used if a _nullish value_ is present:
    **-** Nullish-coalescing expression (see _Nullish-Coalescing Expression_ for details).
**Note**. _Nullish types_ are not compatible with typeObject:

1 functionnullish (
2 o:Object, nullish1:null, nullish2:undefined, nullish3:null|undefined,
3 nullish4:AnyClassOrInterfaceType|null|undefined
4 ) {
5 o = nullish1/* compile-time error - type'null'is not compatible with
6 Object */
7 o = nullish2/* compile-time error - type'undefined'is not compatible
8 with Object */
9 o = nullish3/* compile-time error - type'null|undefined'is not
10 compatible with Object */
11 o = nullish4/* compile-time error - type
12 'AnyClassOrInterfaceType|null|undefined'is not
13 compatible with Object */
14 }

### 3.22 Default Values for Types

```
Note. This ArkTS feature is experimental.
So-called default values are used by the following types for variables that require no explicit initialization (see Variable
Declarations ):
```
- _Value Types_ ;
- Typeundefinedand all its supertypes
All other types, including reference types, enumeration types, and type parameters have no default values.
Default values of value types are as follows:

```
48 Chapter 3. Types
```

```
Data Type Default Value
number 0 asnumber
byte 0 asbyte
short 0 asshort
int 0 asint
long 0 aslong
float +0.0 asfloat
double +0.0 asdouble
char u0000
boolean false
```
```
Valueundefinedis the default value of each type to which this value can be assigned.
```
1 classA {
2 f1:string|undefined
3 f2?: boolean
4 }
5 leta =newA()
6 console.log (a.f1, a.f2)
7 // Output: undefined, undefined

```
3.22. Default Values for Types 49
```

**50 Chapter 3. Types**


##### CHAPTER

### FOUR

### NAMES, DECLARATIONS AND SCOPES

This chapter introduces the following three mutually-related notions:

- Names,
- Declarations, and
- Scopes.

Each entity in an ArkTS program—a variable, a constant, a class, a type, a function, a method, etc.—is introduced via
a _declaration_. An entity declaration defines a _name_ of the entity. The name is used to refer to the entity further in the
program text. The declaration binds the entity name with the _scope_ (see _Scopes_ ). The scope affects the accessibility of
a new entity, and how it can be referred to by its qualified or simple (unqualified) name.

### 4.1 Names

A name is a sequence of one or more identifiers. A name allows referring to any declared entity. Names can have two
syntactical forms:

- _Simple name_ that consists of a single identifier;
- _Qualified name_ that consists of a sequence of identifiers with the token ‘.’ as separator.

Both situations are covered by the below syntax rule:

qualifiedName:
identifier ('.'identifier )*
;

In a qualified name _N.x_ (where _N_ is a simple name, andxis an identifier that can follow a sequence of identifiers
separated with ‘.’ tokens), _N_ can name the following:

- Name of a module (see _Modules and Namespaces_ ) that is introduced as a result ofimport * as N(see _Bind_
    _All with Qualified Access_ ) withxto name the exported entity;
- A class or interface type (see _Classes_ , _Interfaces_ ) withxto name its static member;
- A class or interface type variable withxto name its instance member.

##### 51


### 4.2 Declarations

```
A declaration introduces a named entity in an appropriate declaration scope (see Scopes ), see
```
- _Type Declarations_ ;
- _Variable and Constant Declarations_ ;
- _Function Declarations_ ;
- _Classes_ ;
- _Interfaces_ ;
- _Enumerations_ ;
- _Local Declarations_ ;
- _Top-Level Declarations_ ;
- _Overload Declarations_ ;
- _Annotations_ ;
- _Ambient Declarations_.
Each declaration in the declaration scope must be _distinguishable_. Declarations are _distinguishable_ if they have different
names.
Distinguishable declarations are represented by the examples below:

1 const PI = 3.14
2 const pi = 3
3 functionPi() {}
4 typeIP = number[]
5 class A {
6 static method() {}
7 method() {}
8 field: number= PI
9 static field:number = PI + pi
10 }

```
A compile-time error occurs if a declaration is not distinguishable:
```
1 // compile-time error: The constant and the function have the same name.
2 const PI = 3.14
3 functionPI() { return 3.14 }
4
5 // compile-time error: The type and the variable have the same name.
6 class Person {}
7 letPerson:Person
8
9 // compile-time error: The field and the method have the same name.
10 class C {
11 counter:number
12 counter(): number{
13 return this.counter
14 }
15 }
16
(continues on next page)

```
52 Chapter 4. Names, Declarations and Scopes
```

```
(continued from previous page)
```
17 /* compile-time error: Name of the declaration clashes with the predefined
18 type or standard library entity name. */
19 let number:number = 1
20 letString =true
21 functionRecord () {}
22 interface Object {}
23 letArray = 42
24
25
26 /* compile-time error: ambient and non-ambient declarations refer to the
27 same entity in a single module
28 */
29 declare functionfoo()
30 functionfoo() {}

### 4.3 Scopes

```
Different entity declarations introduce new names in different scopes. Scope is the region of program text where an
entity is declared, along with other regions it can be used in. The following entities are always referred to by their
qualified names only:
```
- Class and interface members (both static and instance ones);
- Entities imported via qualified import; and
- Entities declared in namespaces (see _Namespace Declarations_ ).
Other entities are referred to by their simple (unqualified) names.
Entities within the scope are accessible (see _Accessible_ ).
The scope level of an entity depends on the context the entity is declared in:
- _Module level scope_ is applicable to modules only. _Constants_ and _variables_ are accessible (see _Accessible_ ) from
their respective points of declaration to the end of the module. Other entities are accessible through the entire
scope level. If exported, a name can be accessed in other modules.
- _Namespace level scope_ is applicable to namespaces only. _Constants_ and _variables_ are accessible (see _Accessible_ )
from their respective points of declaration to the end of the namespace including all embedded namespaces. Other
entities are accessible through the entire namespace scope level including embedded namespaces. If exported, a
name can be accessed outside the namespace with mandatory namespace name qualification.
- A name declared inside a class ( _class level scope_ ) is accessible (see _Accessible_ ) in the class and sometimes,
depending on the access modifier (see _Access Modifiers_ ), outside the class, or by means of a derived class.
Access to names inside the class is qualified with one of the following:
**-** Keywordsthisorsuper;
**-** Class instance expression for the names of instance entities; or
**-** Name of the class for static entities.
Outside access is qualified with one of the following:

```
4.3. Scopes 53
```

**-** The expression the value stores;
**-** A reference to the class instance for the names of instance entities; or
**-** Name of the class for static entities.
ArkTS supports using the same identifier as names of a static entity and of an instance entity. The two names are
_distinguishable_ by the context, which is either a name of a class for static entities or an expression that denotes
an instance.
- A name declared inside an interface ( _interface level scope_ ) is accessible (see _Accessible_ ) inside and outside that
interface (defaultpublic).
- _The scope of a type parameter_ name in a class or interface declaration is that entire declaration, excluding static
member declarations.
- The scope of a type parameter name in a function declaration is that entire declaration ( _function type parameter
scope_ ).
- The scope of a name declared inside the body of a function or a method declaration is the body of that declaration
from the point of declaration and up to the end of the body ( _method_ or _function scope_ ). This scope is also applied
to function or method parameter names.
- The scope of a name declared inside a block is the body of the block from the point of the name declaration and
up to the end of the block ( _block scope_ ).

1 functionfoo() {
2 letx = y// compile-time error - y is not accessible yet
3 lety = 1
4 }

```
Scopes of two names can overlap (e.g., when statements are nested). If scopes of two names overlap, then:
```
- The innermost declaration takes precedence; and
- Access to the outer name is not possible.
Class, interface, and enum members can only be accessed by applying the dot operator ‘.’ to an instance. Accessing
them otherwise is not possible.

### 4.4 Accessible

```
Entity is considered accessible if it belongs to the current scope (see Scopes ) and means that its name can be used for
different purposes as follows:
```
- Type name is used to declare variables, constants, parameters, class fields, or interface properties;
- Function or method name is used to call the function or method;
- Variable name is used to read or change the value of the variable;
- Name of a module introduced as a result of import with Bind All with Qualified Access (see _Bind All with_
    _Qualified Access_ ) is used to deal with exported entities.

```
54 Chapter 4. Names, Declarations and Scopes
```

### 4.5 Type Declarations

```
An interface declaration (see Interfaces ), a class declaration (see Classes ), an enum declaration (see Enumerations ), or
a type alias (see Type Alias Declaration ) are type declarations.
The syntax of type declaration is presented below:
```
```
typeDeclaration:
classDeclaration
|interfaceDeclaration
|enumDeclaration
|typeAlias
;
```
#### 4.5.1 Type Alias Declaration

```
Type aliases enable using meaningful and concise notations by providing the following:
```
- Names for anonymous types (array, function, and union types); or
- Alternative names for existing types.
Scopes of type aliases are module or namespace level scopes. Names of all type aliases must follow the uniqueness
rules of _Declarations_ in the current context.
The syntax of _type alias_ is presented below:

```
typeAlias:
'type' identifier typeParameters? '=' type
;
```
```
Meaningful names can be provided for anonymous types as follows:
```
1 typeMatrix =number[][]
2 typeHandler = (s: string, no:number) =>string
3 typePredicate<T> = (x: T) =>boolean
4 typeNullableNumber =number |null

```
If the existing type name is too long, then a shorter new name can be introduced by using type alias (particularly for a
generic type).
```
1 typeDictionary = Map<string,string>
2 typeMapOfString<T> = Map<T, string>

```
A type alias acts as a new name only. It neither changes the original type meaning nor introduces a new type.
```
1 typeVector =number[]
2 functionmax(x: Vector): number {
3 letm = x[0]
4 for(letv of x)
5 if (v > m) m = v
6 return m
(continues on next page)

```
4.5. Type Declarations 55
```

(continued from previous page)
7 }
8
9 letx: Vector= [2, 3, 1]
10 console.log(max(x)) // output: 3

```
Type aliases can be recursively referenced inside the right-hand side of a type alias declaration.
In a type alias defined astype A = something, A can be used recursively if it is one of the following:
```
- Array element type:type A = A[]; or
- Type argument of a generic type:type A = C<A>.

```
1 typeA = A[]// ok, used as element type
2
3 class C<T> {/*body*/}
4 typeB = C<B>// ok, used as a type argument
5
6 typeD =string | Array<D>// ok
```
```
Any other use causes a compile-time error, because the compiler does not have enough information about the defined
alias:
```
```
1 typeE = E // compile-time error
2 typeF =string | E// compile-time error
```
```
The same rules apply to a generic type alias defined astype A<T> = something:
```
```
1 typeA<T> = Array<A<T>> // ok, A<T> is used as a type argument
2 typeA<T> = string | Array<A<T>>// ok
3
4 typeA<T> = A<T>// compile-time error
```
```
A compile-time error occurs if a generic type alias is used without a type argument:
```
```
1 typeA<T> = Array<A> // compile-time error
```
```
Note. There is no restriction on using a type parameter T in the right side of a type alias declaration. The following
code is valid:
```
```
1 typeNodeValue<T> = T | Array<T> | Array<NodeValue<T>>;
```
### 4.6 Variable and Constant Declarations

#### 4.6.1 Variable Declarations

```
A non-ambient variable declaration introduces a new variable which is in fact a named storage location. A declared
variable must be assigned an initial value before the first usage. The initial value is assigned either as a part of the
declaration or in various forms via initialization.
The syntax of variable declarations is presented below:
```
```
56 Chapter 4. Names, Declarations and Scopes
```

```
variableDeclarations:
'let'variableDeclarationList
;
```
```
variableDeclarationList:
variableDeclaration (',' variableDeclaration)*
;
```
```
variableDeclaration:
identifier':'type initializer?
|identifier initializer
;
```
```
initializer:
'=' expression
;
```
```
When a variable is introduced by a variable declaration, typeTof the variable is determined as follows:
```
- Tis the type specified in a type annotation (if any) of the declaration.
    **-** If the declaration also has an initializer, then the initializer expression type must be assignable toT(see
       _Assignability with Initializer_ ).
- If no type annotation is available, thenTis inferred from the initializer expression (see _Type Inference from_
    _Initializer_ ).
An ambient variable declaration must have _type_ but has no _initializer_. Otherwise, a compile-time error occurs.

1 leta: number// ok
2 letb = 1 // ok, type'int'is inferred
3 letc: number= 6, d = 1, e = "hello" // ok
4
5 // ok, type of lambda and type of'f'can be inferred
6 letf = (p:number) => b + p
7 letx // compile-time error -- either type or initializer

```
Every variable in a program must have an initial value before it can be used:
```
- If the _initializer_ of a variable is specified explicitly, then its execution produces the initial value for this variable.
- Otherwise, the following situations are possible:
    **-** If the type of a variable isT, andThas a _default value_ (see _Default Values for Types_ ), then the variable is
       initialized with the default value.
    **-** If a variable has no default value, then its value must be set by the _Simple Assignment Operator_ before any
       use of the variable.
Invalid initialization is represented in the example below:

1 leta = b// compile-time error: circular dependency
2 letb = a

```
4.6. Variable and Constant Declarations 57
```

#### 4.6.2 Constant Declarations

```
A constant declaration introduces a named variable with a mandatory explicit value. The value of a constant cannot
be changed by an assignment expression (see Assignment ). If the constant is an object or array, then object fields or
array elements can be modified.
The syntax of constant declarations is presented below:
```
```
constantDeclarations:
'const' constantDeclarationList
;
```
```
constantDeclarationList:
constantDeclaration (',' constantDeclaration)*
;
```
```
constantDeclaration:
identifier(':' type)?initializer
;
```
```
The typeTof a constant declaration is determined as follows:
```
- IfTis the type specified in a type annotation (if any) of the declaration, then the initializer expression must be
    assignable toT(see _Assignability with Initializer_ ).
- If no type annotation is available, thenTis inferred from the initializer expression (see _Type Inference from_
    _Initializer_ ).

1 const a:number = 1// ok
2 const b = 1// ok, int type is inferred
3 const c:number = 1, d = 2, e = "hello"// ok
4 const x// compile-time error -- initializer is mandatory
5 const y:number // compile-time error -- initializer is mandatory

#### 4.6.3 Assignability with Initializer

```
If a variable or constant declaration contains type annotationTand initializer expression E , then the type of E must be
assignable toT(see Assignability ).
```
#### 4.6.4 Type Inference from Initializer

```
The type of a declaration that contains no explicit type annotation is inferred from the initializer expression as follows:
```
- In a variable declaration (not in a constant declaration, though), if the initializer expression is of a literal type, then
    the literal type is replaced for its supertype, if any (see _Subtyping for Literal Types_ ). If the initializer expression
    is of a union type that contains literal types, then each literal type is replaced for its supertype (see _Subtyping for_
    _Literal Types_ ), and then normalized (see _Union Types Normalization_ ).
- Otherwise, the type of a declaration is inferred from the initializer expression.

```
58 Chapter 4. Names, Declarations and Scopes
```

```
If the type of the initializer expression cannot be inferred, then a compile-time error occurs (see Object Literal ):
```
1 leta =null // type of'a'is null
2 letaa =undefined // type of'aa' is undefined
3 letarr = [null,undefined] // type of'arr'is (null | undefined)[]
4
5 letcond: boolean= /*some initialization*/
6
7 letb = cond? 1 : 2 // type of'b'is int
8 letc = cond? 3 : 3.14 // type of'c'is double
9 letd = cond? "one" : "two"// type of'd'is string
10 lete = cond? 1 : "one" // type of'e'is int | string
11
12 const bb = cond? 1 : 2 // type of'bb'is int
13 const cc = cond? 3 :3.14 // type of'cc'is double
14 const dd = cond? "one" : "two"// type of'dd'is "one" | "two"
15 const ee = cond? 1 : "one" // type of'ee'is int | "one"
16
17 letf = {name: "aa"}// compile-time error: type unknown
18
19 declare let x1 = 1 // compile-time error: ambient variable cannot have initializer
20 declare constx2 = 1 // type of 'x2'is int
21 let x3 = 1 // type of 'x3'is int
22 const x4 = 1 // type of 'x4'is int
23
24 declare let s1 = "1" // compile-time error: ambient variable cannot have initializer
25 declare consts2 = "1" // type of's2'is "1"
26 let s3 = "1" // type of's3'is string
27 const s4 = "1" // type of's4'is "1"

### 4.7 Function Declarations

```
Function declarations specify names, signatures, and bodies when introducing named functions. An optional function
body is a block (see Block ).
The syntax of function declarations is presented below:
```
```
functionDeclaration:
modifiers?'function'identifier
typeParameters? signature block?
;
```
```
modifiers:
'native'|'async'
;
```
```
Functions must be declared on the top level (see Top-Level Statements ).
If a function is declared generic (see Generics ), then its type parameters must be specified.
```
```
4.7. Function Declarations 59
```

```
The modifiernativeindicates that the function is a native function (see Native Functions in Experimental Features).
If a native function has a body, then a compile-time error occurs.
Functions with the modifierasyncare discussed in Async Functions.
```
#### 4.7.1 Signatures

```
A signature defines parameters and the return type (see Return Type ) of a function, method, or constructor.
The syntax of signature is presented below:
```
```
signature:
'(' parameterList?')'returnType?
;
```
#### 4.7.2 Parameter List

```
A signature may contain a parameter list that specifies an identifier of each parameter name, and the type of each
parameter. The type of each parameter must be defined explicitly. If the parameter list is omitted, then the function or
the method has no parameters.
The syntax of parameter list is presented below:
```
```
parameterList:
parameter(','parameter)* (','restParameter)? ','?
|restParameter ','?
;
```
```
parameter:
annotationUsage? (requiredParameter| optionalParameter)
;
```
```
requiredParameter:
identifier':'type
;
```
```
If a parameter is required , then each function or method call must contain an argument corresponding to that parameter.
The function below has required parameters :
```
1 functionpower(base:number, exponent:number):number {
2 returnMath.pow(base, exponent)
3 }
4 power(2, 3)// both arguments are required in the call

```
Several parameters can be optional , allowing to omit corresponding arguments in a call (see Optional Parameters ).
A compile-time error occurs if an optional parameter precedes a required parameter.
The last parameter of a function or a method can be a single rest parameter (see Rest Parameter ).
```
```
60 Chapter 4. Names, Declarations and Scopes
```

```
If a parameter type is prefixed withreadonly, then there are additional restrictions on the parameter as described in
Readonly Parameters.
```
#### 4.7.3 Readonly Parameters

```
If the parameter type isreadonlyarray or tuple type, then no assignment and no function or method call can modify
elements of this array or tuple. Otherwise, a compile-time error occurs:
```
1 functionfoo(array: readonly number[], tuple: readonly[number,string]) {
2 letelement = array[0]// OK, one can get array element
3 array[0] = element // compile-time error, array is readonly
4
5 element = tuple[0] // OK, one can get tuple element
6 tuple[0] = element // compile-time error, tuple is readonly
7 }

```
Any assignment of readonly parameters and variables must follow the limitations stated in Type of Expression.
```
#### 4.7.4 Optional Parameters

```
Optional parameters can be of two forms as follows:
```
```
optionalParameter:
identifier(':' type)?'='expression
|identifier '?' ':' type
;
```
```
The first form contains anexpressionthat specifies a default value. It is called a parameter with default value. The
value of the parameter is set to the default value if the argument corresponding to that parameter is omitted in a function
or method call:
```
1 functionpair(x:number, y:number= 7)
2 {
3 console.log(x, y)
4 }
5 pair(1, 2) // prints: 1 2
6 pair(1)// prints: 1 7

```
The second form is a short-cut notation andidentifier '?' ':' typeeffectively means thatidentifierhas
typeT | undefinedwith the default valueundefined.
For example, the following two functions can be used in the same way:
```
1 functionhello1(name:string |undefined =undefined) {}
2 functionhello2(name?: string) {}
3
4 hello1()// 'name'has'undefined'value
(continues on next page)

```
4.7. Function Declarations 61
```

(continued from previous page)
5 hello1("John") //'name'has a string value
6 hello2()// 'name'has'undefined'value
7 hello2("John") //'name'has a string value
8
9 functionfoo1 (p?: number) {}
10 functionfoo2 (p:number |undefined =undefined) {}
11
12 foo1() // 'p'has'undefined'value
13 foo1(5)// 'p'has a numeric value
14 foo2() // 'p'has'undefined'value
15 foo2(5)// 'p'has a numeric value

#### 4.7.5 Rest Parameter

```
Rest parameters allow functions, methods, constructors, or lambdas to take arbitrary numbers of arguments. Rest
parameters have thespreadoperator ‘...’ as a prefix before the parameter name.
The syntax of rest parameter is presented below:
```
```
restParameter:
annotationUsage? '...'identifier ':' type
;
```
```
A compile-time error occurs if a rest parameter:
```
- Is not the last parameter in a parameter list;
- Has a type that is not an array type, a tuple type, nor a type parameter constrained by an array or a tuple type.
A call of entity with a rest parameter of array typeT[](orFixedArray<T>) can accept any number of arguments of
types that are assignable (see _Assignability_ ) toT:

1 functionsum(...numbers: number[]): number{
2 letres = 0
3 for(letnof numbers)
4 res += n
5 returnres
6 }
7
8 sum() // returns 0
9 sum(1) // returns 1
10 sum(1, 2, 3)// returns 6

```
If an argument of array typeT[]is to be passed to a call of entity with the rest parameter, then the spread expression
(see Spread Expression ) must be used with thespreadoperator ‘...’ as a prefix before the array argument:
```
```
1 functionsum(...numbers: number[]): number{
2 letres = 0
3 for(letnof numbers)
4 res += n
(continues on next page)
```
```
62 Chapter 4. Names, Declarations and Scopes
```

(continued from previous page)
5 returnres
6 }
7
8 letx: number[] = [1, 2, 3]
9 sum(...x) // spread an array'x'
10 // returns 6

```
A call of entity with a rest parameter of tuple type [T 1 , ..., Tn] can accept onlynarguments of types that are
assignable (see Assignability ) to the correspondingTi:
```
```
1 functionsum(...numbers: [number,number, number]):number {
2 returnnumbers[0] + numbers[1] + numbers[2]
3 }
4
5 sum() // compile-time error: wrong number of arguments, 0 instead of 3
6 sum(1) // compile-time error: wrong number of arguments, 1 instead of 3
7 sum(1, 2, "a") // compile-time error: wrong type of the 3rd argument
8 sum(1, 2, 3) // returns 6
```
```
It is legal though meaningless to declare a function with an optional parameter followed by a rest parameter of a tuple
type. However, use of such function without explicitly set optional and rest parameters will cause compile-time error:
```
```
1 // optional tuple + rest tuple
2 functiong(opt?: [number,string], ...tail: [number,string]) {// OK
3 // ...
4 }
5
6 g()// CTE - no rest tuple
7 g([1, "str"])// CTE - no rest tuple
8 g([ 1, "str"], 1, "str")// OK
```
```
If an argument of tuple type [T 1 , ..., Tn] is to be passed to a call of entity with the rest parameter, then a spread
expression (see Spread Expression ) must have thespreadoperator ‘...’ as a prefix before the tuple argument:
```
```
1 functionsum(...numbers: [number,number, number]):number {
2 returnnumbers[0] + numbers[1] + numbers[2]
3 }
4
5 letx: [number, number, number] = [1, 2, 3]
6 sum(...x) // spread tuple'x'
7 // returns 6
```
```
If an argument of fixed-size array typeFixedArray<T>is to be passed to a function or a method with the rest parameter,
then the spread expression (see Spread Expression ) must be used with thespreadoperator ‘...’ as a prefix before the
fixed-size array argument:
```
```
1 functionsum(...numbers: Array<number>):number{
2 letres = 0
3 for(letnof numbers)
4 res += n
5 returnres
6 }
7
(continues on next page)
```
```
4.7. Function Declarations 63
```

(continued from previous page)
8 letx: FixedArray<number> = [1, 2, 3]
9 sum(...x) // spread an fixed-size array'x'
10 // returns 6

```
If constrained by an array or a tuple type, a type parameter can be used with generics as a rest parameter.
```
```
1 functionsum<T extendsArray<number>>(...numbers: T): number{
2 letres = 0
3 for(letnof numbers)
4 res += n
5 returnres
6 }
```
```
Note. Any call to a function, method, constructor, or lambda with a rest parameter implies that a new array or tuple is
created from the arguments provided.
```
1 functionfoo(...array_parameter: number[], ...tuple_parameter: [number, string]) {
2 // array_parameter is a new array created from the arguments passed
3 // tuple_parameter is a new tuple created from the arguments passed
4 array_parameter[0] = 1234
5 tuple_parameter[0] = 1234
6 console.log (array_parameter[0], tuple_parameter[0])// 1234 1234 is the output
7 }
8
9 const array_argument:number[] = [1,2,3,4]
10 const tuple_argument: [number,string] = [1,"234"]
11
12 console.log (array_argument[0], tuple_argument[0])// 1 1 is the output
13
14 foo (...array_argument, ...tuple_argument)
15 // array_argument is spread into a sequence of its elements
16 // tuple_argument is spread into a sequence of its elements
17
18 console.log (array_argument[0], tuple_argument[0])// 1 1 is the output

#### 4.7.6 Shadowing by Parameter

```
If the name of a parameter is identical to the name of a top-level variable accessible (see Accessible ) within the body of
a function or a method with that parameter, then the name of the parameter shadows the name of the top-level variable
within the body of that function or method:
```
```
1 letx: number= 1
2 functionfoo (x:string) {
3 // 'x'refers to the parameter and has type string
4 }
5 class SomeClass {
6 method (x:boolean) {
7 // 'x'refers to the method parameter and has type boolean
(continues on next page)
```
```
64 Chapter 4. Names, Declarations and Scopes
```

(continued from previous page)
8 }
9 }
10 x++// 'x'refers to the global variable

#### 4.7.7 Return Type

```
Function, method, or lambda return type defines the resultant type of the function, method, or lambda execution (see
Function Call Expression , Method Call Expression , and Lambda Expressions ). During the execution, the function,
method, or lambda can produce a value of a type that is assignable to the return type (see Assignability ).
The syntax of return type is presented below:
```
```
returnType:
':' (type| 'this')
;
```
```
If function or method return type is notvoid(see Type void ), and the execution path of the function or method body
has no return statement (see return Statements ), then a compile-time error occurs.
A compile-time error occurs if lambda return type is notnever(see Type never ), and the execution path of a function,
method, or lambda body has no return statement (see return Statements ).
A special form of return type with the keywordthisas type annotation can be used in class instance methods only
(see Methods Returning this ).
If function, method, or lambda return type is not specified, then it is inferred from its body (see Return Type Inference ).
If there is no body, then the function, method, or lambda return type isvoid(see Type void ).
```
#### 4.7.8 Return Type Inference

```
A missing function, method, or lambda return type can be inferred from the function, method, or lambda body. A
compile-time error occurs if return type is missing from a native function (see Native Functions ).
The current version of ArkTS allows inferring return types at least under the following conditions:
```
- If there is no return statement, or if all return statements have no expressions, then the return type isvoid(see
    _Type void_ ).
- If there are _k_ return statements (where _k_ is 1 or more) with the same type expression _R_ , thenRis the return type.
- If there are _k_ return statements (where _k_ is 2 or more) with expressions of typesT 1 ,...,Tk, thenRis the _union_
    _type_ (see _Union Types_ ) of these types (T 1 | ... |Tk), and its normalized version (see _Union Types Normalization_ )
    is the return type. If at least one of return statements has no expression, then typeundefinedis added to the
    return type union.
- If a lambda body contains no return statement but at least one throw statement (see _throw Statements_ ), then the
    lambda return type isnever(see _Type never_ ).
- If a function, a method, or a lambda isasync(see _Asynchronous API_ ), a return type is inferred by applying the
    above rules, and the return typeTis notPromise, then the return type is assumed to bePromise<T>.

```
4.7. Function Declarations 65
```

Future compiler implementations are to infer the return type in more cases. Type inference is represented in the example
below:

// Explicit return type
functionfoo(): string{ return"foo" }

// Implicit return type inferred as string
functiongoo() {return "goo" }

classBase {}
classDerived1extendsBase {}
classDerived2extendsBase {}

functionbar (condition: boolean) {
if (condition)
return newDerived1()
else
return newDerived2()
}
// Return type of bar will be Derived1|Derived2 union type

functionboo (condition: boolean) {
if (condition)return 1
}
// That is a compile-time error as there is an execution path with no return

If the compiler fails to recognize a particular type inference case, then a corresponding compile-time error occurs.

**66 Chapter 4. Names, Declarations and Scopes**


##### CHAPTER

### FIVE

### GENERICS

Class, interface, type alias, method, and function are program entities that can be parameterized in ArkTS by one or
several types. An entity so parameterized introduces a _generic declaration_ (called _a generic_ for brevity).

Types used as generic parameters in a generic are called _type parameters_ (see _Type Parameters_ ).

A _generic_ must be instantiated in order to be used. _Generic instantiation_ is the action that transforms a _generic_ into a real
program entity (non-generic class, interface, union, array, method, or function), or into another _generic instantiation_.
Instantiation (see _Generic Instantiations_ ) can be performed either explicitly or implicitly.

ArkTS has special types that look similar to generics syntax-wise, and allow creating new types during compilation
(see _Utility Types_ ).

### 5.1 Type Parameters

_Type parameter_ is declared in the type parameter section. It can be used as an ordinary type inside a _generic_.

Syntax-wise, a _type parameter_ is an unqualified identifier with a proper scope (see _Scopes_ for the scope of type pa-
rameters). Each type parameter can have a _constraint_ (see _Type Parameter Constraint_ ). A type parameter can have a
default type (see _Type Parameter Default_ ), and can specify its _in-_ or _out-_ variance (see _Type Parameter Variance_ ).

The syntax of _type parameter_ is presented below:

typeParameters:
'<' typeParameterList'>'
;

typeParameterList:
typeParameter(','typeParameter)*
;

typeParameter:
('in'| 'out')? identifier constraint? typeParameterDefault?
;

constraint:
'extends'type
;

```
(continues on next page)
```
##### 67


```
(continued from previous page)
typeParameterDefault:
'=' typeReference('[]')?
;
```
```
A generic class, interface, type alias, method, or function defines a set of parameterized classes, interfaces, unions,
arrays, methods, or functions respectively (see Generic Instantiations ). A single type argument can define only one set
for each possible parameterization of the type parameter section.
```
#### 5.1.1 Type Parameter Constraint

```
If possible instantiations need to be constrained, then an individual constraint can be set for each type parameter after
the keywordextends. A constraint can have the form of any type.
If no constraint is specified, then the constraint is Type Any , i.e., the lacking explicit constraint effectively means
extends Any. As a consequence, the type parameter is not compatible with Type Object , and has neither methods nor
fields available for use.
If type parameter T has type constraint S , then the actual type of the generic instantiation must be a subtype of S (see
Subtyping ). If the constraint S is a non-nullish type (see Nullish Types ), then T is also non-nullish.
```
1 class Base {}
2 class DerivedextendsBase { }
3 class SomeType { }
4
5 class G<TextendsBase> { }
6
7 letx =newG<Base> // OK
8 lety =newG<Derived> // OK
9 letz =newG<SomeType> // Compile-time : SomeType is not compatible with Base
10
11 class H<TextendsBase|SomeType> {}
12 leth1 =newH<Base> // OK
13 leth2 =newH<Derived> // OK
14 leth3 =newH<SomeType>// OK
15 leth4 =newH<Object> // Compile-time : Object is not compatible with Base|SomeType
16
17 class Exotic<Textends"aa"| "bb"> {}
18 lete1 =newExotic<"aa"> // OK
19 lete2 =newExotic<"cc"> // Compile-time : "cc" is not compatible with "aa"| "bb"
20
21 class A {
22 f1:number= 0
23 f2:string= ""
24 f3:boolean= false
25 }
26 class B <Textendskeyof A> {}
27 letb1 =newB<'f1'> // OK
28 letb2 =newB<'f0'> // Compile-time error as'f0'does not fit the constraint
29 letb3 =newB<keyof A> // OK

```
68 Chapter 5. Generics
```

```
A type parameter of a generic can depend on another type parameter of the same generic.
If S constrains T , then the type parameter T directly depends on the type parameter S , while T directly depends on the
following:
```
- _S_ ; or
- Type parameter _U_ that depends on _S_.
A compile-time error occurs if a type parameter in the type parameter section depends on itself.

1 class Base {}
2 class DerivedextendsBase { }
3 class SomeType { }
4
5 class G<T, SextendsT> {}
6
7 letx: G<Base, Derived> // correct: the second argument directly
8 // depends on the first one
9 lety: G<Base, SomeType>// error: SomeType does not depend on Base
10
11 class A0<T> {
12 data:T
13 constructor(p:T) {this.data = p }
14 foo () {
15 leto:Object =this.data // error: T not compatible with Object
16 console.log (this.data.toString())// error: T has no methods or fields
17 }
18 }
19
20 class A1<TextendsObject>extendsA0<T> {
21 constructor(p:T) {super(p);this.data = p }
22 overridefoo () {
23 leto:Object =this.data // OK!
24 console.log (this.data.toString())// OK!
25 }
26 }

#### 5.1.2 Type Parameter Default

```
Type parameters of generic types can have defaults. This situation allows dropping a type argument when a particular
type of instantiation is used. However, a compile-time error occurs if:
```
- A type parameter without a default type follows a type parameter with a default type in the declaration of a
    generic type;
- Type parameter default refers to a type parameter defined after the current type parameter.
The application of this concept to both classes and functions is presented in the examples below:

```
1 class SomeType {}
2 interface Interface <T1 = SomeType> { }
3 class Base <T2 = SomeType> { }
(continues on next page)
```
```
5.1. Type Parameters 69
```

(continued from previous page)
4 class Derived1extendsBaseimplementsInterface { }
5 // Derived1 is semantically equivalent to Derived2
6 class Derived2extendsBase<SomeType>implements Interface<SomeType> { }
7
8 functionfoo<T =number>(input:T): T {return input}
9 foo(1) // this call is semantically equivalent to next one
10 foo<number>(1)
11
12 class C1 <T1, T2 =number, T3> {}
13 // That is a compile-time error, as T2 has default but T3 does not
14
15 class C2 <T1, T2 =number, T3 = string> {}
16 letc1 =newC2<number> // equal to C2<number, number, string>
17 letc2 =newC2<number, string> // equal to C2<number, string, string>
18 letc3 =newC2<number, Object, number>// all 3 type arguments provided
19
20 functionfoo <T1 = T2, T2 = T1> () {}
21 // That is a compile-time error,
22 // as T1's default refers to T2, which is defined after the T1
23 // T2's default is valid as it refers to already defined type parameter T1

#### 5.1.3 Type Parameter Variance

```
Normally, two different instantiations of the same generic class or interface (like Array<number> and
Array<string>) are handled as different and unrelated types. ArkTS supports type parameter variance that allows
subtyping relationship between such instantiations (See Subtyping ), depending on the subtyping relationship between
argument types.
When declaring type parameters of a generic type, special keywordsinorout(called variance modifiers ) are used to
specify the variance of the type parameter (see Invariance, Covariance and Contravariance ).
Type parameters with the keywordoutare covariant. Covariant type parameters can be used in the out-position only
as follows:
```
- Constructors can haveouttype parameters as parameters;
- Methods can haveouttype parameters as return types;
- Fields that haveouttype parameters as type must bereadonly.
- Otherwise, a compile-time error occurs.
Type parameters with the keywordinare _contravariant_. Contravariant type parameters can be used in the in-position
only as follows:
- Methods can haveintype parameters as parameter types.
- Otherwise, a compile-time error occurs.
Type parameters with no variance modifier are implicitly _invariant_ , and can occur in any position.

```
1 class X<in T1, out T2, T3> {
2
(continues on next page)
```
```
70 Chapter 5. Generics
```

(continued from previous page)
3 // T1 can be used in in-position only
4 foo (p:T1) {} // OK
5 foo1(p:T1): T1 {return p }// error: T1 in out-position
6 fldT1: T1// error: T1 in invariant position
7
8 constructor(x:T2) {this.fldT2 = x }// OK
9 bar(x:T2) : T2 {returnx } // CTE (x in in-position)
10 readonlyfldT2:T2 // OK
11 bar1() : T2 {return this.fldT2 } // OK
12
13 // T3 can be used in any position (in-out, write-read)
14 fldT3: T3
15 method (p:T3): T3 {this.fldT3 = p;return p} // OK
16 }

```
In case of function types (see Function Types ), variance interleaving occurs.
```
```
1 class X<in T1, out T2> {
2 foo (p:T1): T2 {...} // in - out
3 foo (p: (p:T2)=> T1) {...} // out - in
4 foo (p: (p: (p: T1)=>T2)=> T1) {...} // in - out - in
5 foo (p: (p: (p: (p: T2)=> T1)=>T2)=> T1) {...} // out - in - out - in
6 // and further more
7 }
```
```
A compile-time error occurs if function or method type parameters have a variance modifier specified.
```
### 5.2 Generic Instantiations

```
As mentioned before, a generic declaration defines a set of corresponding generic or non-generic entities. The process
of instantiation is designed to do the following:
```
- Allow producing new generic or non-generic entities;
- Provide every type parameter with a type argument that can be any kind of type, including the type argument
    itself.
As a result of the instantiation process, a new class, interface, union, array, method, or function is created.

```
1 class A <T> {}
2 class B <U, V>extendsA<U> {// Here A<U> is a new generic type
3 field: A<V> // Here A<V> is a new generic type
4 method (p: A<Object>) {} // Here A<Object> is a new non-generic type
5 }
```
```
5.2. Generic Instantiations 71
```

#### 5.2.1 Type Arguments

```
Type arguments are non-empty lists of types that are used for instantiation.
The syntax of type arguments is presented below:
```
```
typeArguments:
'<' type(','type)* '>'
;
```
```
The example below represents instantiations with different forms of type arguments:
```
```
1 Array<number> // instantiated with type number
2 Array<number|string> // instantiated with union type
3 Array<number[]> // instantiated with array type
4 Array<[number, string, boolean]> // instantiated with tuple type
5 Array<()=>void> // instantiated with function type
```
```
A compile-time error occurs if a generic instantiation leads to instantiation of the type FixedArray with the predefined
value type (see Value Types ).
```
1 class A <T> {
2 foo (p:FixedArray<T>) {}
3 }
4 A<int> // compile-time error as such instantiation leads to method foo()
5 // of class A to have type FixedArray<int> in it.
6
7 // The actual code could be like code below - all these fragments result in a compile-
˓→time error
8 newA<int>
9 leta: A<int>|undefined
10 functionfoo (p:A<int>) {}

#### 5.2.2 Explicit Generic Instantiations

```
An explicit generic instantiation is a language construct, which provides a list of type arguments (see Type Arguments )
that specify real types or type parameters to substitute corresponding type parameters of a generic:
```
1 class G<T> {} // Generic class declaration
2 letx: G<number>// Explicit class instantiation, type argument provided
3
4 class A {
5 method <T> () {} // Generic method declaration
6 }
7 leta =newA()
8 a.method<string> ()// Explicit method instantiation, type argument provided
9
10 functionfoo <T> () {} // Generic function declaration
11 foo <string> ()// Explicit function instantiation, type argument provided
12
13 typeMyArray<T> = T[]// Generic type declaration
(continues on next page)

```
72 Chapter 5. Generics
```

```
(continued from previous page)
```
14 letarray: MyArray<boolean> = [true, false]// Explicit array instantiation, type␣
˓→argument provided
15
16 class X <T1, T2> {}
17 // Different forms of explicit instantiations of class X producing new generic entities
18 class Y<T>extendsX<number, T> { // class Y extends X instantiated with number and T
19 f1:X<Object, T>// X instantiated with Object and T
20 f2:X<T,string> // X instantiated with T and string
21 constructor() {
22 this.f1 = newX<Object,T>
23 this.f2 = newX<T,string>
24 }
25 }

```
A compile-time error occurs if type arguments are provided for non-generic class, interface, type alias, method, or
function.
In the explicit generic instantiation G <T 1 ,...,Tn>, G is the generic declaration, and <T 1 ,...,Tn> is the list of its
type arguments.
If type parameters T 1 ,..., T nof a generic declaration are constrained by the correspondingC 1 ,...,Cn, then T iis
assignable to each constraint type C i(see Assignability ). All subtypes of the type listed in the corresponding constraint
have each type argument T iof the parameterized declaration ranging over them.
A generic instantiation G <T 1 ,...,Tn> is well-formed if all of the following is true:
```
- The generic declaration name is _G_ ;
- The number of type arguments equals the number of type parameters of _G_ ; and
- All type arguments are assignable to the corresponding type parameter constraint (see _Assignability_ ).
A compile-time error occurs if an instantiation is not well-formed.
Unless explicitly stated otherwise in appropriate sections, this specification discusses generic versions of class type,
interface type, or function.
Any two generic instantiations are considered _provably distinct_ if:
- Both are parameterizations of distinct generic declarations; or
- Any of their type arguments is provably distinct.

#### 5.2.3 Implicit Generic Instantiations

```
In an implicit instantiation, type arguments are not specified explicitly. Such type arguments are inferred (see Type
Inference ) from the context in which a generic is referred. It is represented in the example below:
```
```
1 functionfoo <G> (x:G, y:G) {}// Generic function declaration
2 foo (newObject,newObject) // Implicit generic function instantiation
3 // based on argument types: the type argument is inferred
4
5
6 functionprocess <P, R> (arg:P, cb?: (p: P) => R): P | R {
(continues on next page)
```
```
5.2. Generic Instantiations 73
```

(continued from previous page)
7 // return the data itself or if the processing function provided the
8 // result of processing
9 return cb !=undefined? cb (arg): arg
10 }
11 process (123, () => {}) // P is inferred as'int', while R is'void'

```
Implicit instantiation is only possible for generic functions and methods.
```
### 5.3 Utility Types

```
ArkTS supports several embedded types, called utility types. Utility types allow constructing new types by adjusting
properties of initial types, for which purpose notations identical to generics are used. If the initial types are class or
interface, then the resultant utility types are also handled as class or interface types. All utility type names are accessible
as simple names (see Accessible ) in any module across all its scopes. Using these names as user-defined entities causes
a compile-time error in accordance with Declarations. An alphabetically sorted list of utility types is provided below.
```
#### 5.3.1 Awaited Utility Type

```
TypeAwaited<T>constructs a type which includes no typePromise. It is similar toawaitinasyncfunctions, or to
the method.then()in Promises. Any occurrence of typePromiseis recursively removed until a generic, a function,
an array, or a tuple type is detected. If typePromiseis not a part of a typeTdeclaration, thenAwaited<T>leavesT
intact.
IfTinAwaited<T>is a type parameter, then subtyping forAwaited<T>is based on the subtyping forT. In other
words,Awaited<T>is a subtype ofAwaited<U>ifTis a subtype ofU. The use of typeAwaited<T>is represented
in the example below:
```
1 typeA = Awaited<Promise<string>> // type A is string
2
3 typeB = Awaited<Promise<Promise<number>>> // type B is number
4
5 typeC = Awaited<boolean| Promise<number>>// type C is boolean | number
6
7 typeD = Awaited <Object> // type D is Object
8
9 typeE = Awaited<Promise<Promise<number>|Promise<string>|Promise<boolean>>>
10 // type E is number|string|boolean
11
12 typeF = Awaited<Promise<(p: Promise<string>) => Promise<number>>>
13 // type F is (p: Promise<string>) => Promise
˓→<number>>
14
15 typeG = Awaited<Promise<Array<Promise<number>>>>
16 // type F is Array<Promise<number>>
(continues on next page)

```
74 Chapter 5. Generics
```

```
(continued from previous page)
```
17
18 functionfoo <T extendsSuperType> (p:Awaited<T>) {}
19 functionbar <T extendsSubType> (p:Awaited<T>) {
20 foo (p)// is a valid call as Awaited<T extends SubType> <: Awaited<T extends␣
˓→SuperType>
21 }

#### 5.3.2 NonNullable Utility Type

```
TypeNonNullable<T>constructs a type by excludingnullandundefinedtypes. If typeTcontains neithernullnor
undefined, thenNonNullable<T>leavesTintact. The use of typeNonNullable<T>is represented in the example
below:
```
1 typeX = Object |null |undefined
2 typeY = NonNullable<X> // type of'Y'is Object
3
4 class A <T> {
5 field:NonNullable<T>// This is a non-nullable version of the type parameter
6 constructor(field:NonNullable<T>) {
7 this.field = field
8 }
9 }
10
11 const a =newA<Object|null> (newObject)
12 a.field// type of field is Object

#### 5.3.3 Partial Utility Type

```
TypePartial<T>constructs a type with all properties ofTset to optional. Tmust be a class or an interface type.
Otherwise, a compile-time error occurs. No method (not even any getter or setter) ofTis a part of thePartial<T>
type. The use is represented in the example below:
```
1 interface Issue {
2 title: string
3 description:string
4 }
5
6 functionprocess(issue: Partial<Issue>) {
7 if (issue.title != undefined) {
8 /* process title */
9 }
10 }
11
12 process({title: "aa"}) // description is undefined

```
5.3. Utility Types 75
```

```
In the example above, typePartial<Issue>is transformed to a distinct but analogous type as follows:
```
```
1 interface /*some name*/ {
2 title?:string
3 description?:string
4 }
```
```
TypeTis not assignable toPartial<T>(see Assignability ), and variables ofPartial<T>are to be initialized with
valid object literals.
Note. If classThas a user-defined getter, setter, or both, then none of those is called when object literal is used with
Partial<T>variables. Object literal has its own built-in getters and setters to modify its variables. It is represented
in the example below:
```
1 interface I {
2 property:number
3 }
4 class Aimplements I {
5 _property: number
6 set property(property:number) {
7 console.log ("Setter called")
8 this._property = property
9 }
10 get property(): number{
11 console.log ("Getter called");
12 return this._property
13 }
14 }
15
16 functionfoo (partial: Partial<A>) {
17 partial.property = 42// setter to be called
18 console.log(partial.property)// getter to be called
19 }
20
21 foo ({property: 1 }) // No getter or setter from class A is called
22 // 42 is printed as object literal has its own setter and getter

#### 5.3.4 Required Utility Type

```
TypeRequired<T>is opposite toPartial<T>, and constructs a type with all properties ofTset to required (i.e., not
optional). Tmust be a class or an interface type, otherwise a compile-time error occurs. No method (not even any
getter or setter) ofTis part of theRequired<T>type. Its usage is represented in the example below:
```
```
1 interface Issue {
2 title?:string
3 description?:string
4 }
5
6 letc: Required<Issue> = {// CTE:'description'should be defined
(continues on next page)
```
```
76 Chapter 5. Generics
```

```
(continued from previous page)
```
7 title: "aa"
8 }

```
In the example above, typeRequired<Issue>is transformed to a distinct but analogous type as follows:
```
1 interface /*some name*/ {
2 title: string
3 description:string
4 }

```
TypeTis not assignable (see Assignability ) toRequired<T>, and variables ofRequired<T>are to be initialized with
valid object literals.
```
#### 5.3.5 Readonly Utility Type

```
TypeReadonly<T>constructs a type with all properties ofTset toreadonly. It means that the properties of the
constructed value cannot be reassigned.Tmust be a class or an interface type, otherwise a compile-time error occurs.
No method (not even any getter or setter) ofTis part of theReadonly<T>type. Its usage is represented in the example
below:
```
1 interface Issue {
2 title: string
3 }
4
5 const myIssue:Readonly<Issue> = {
6 title: "One"
7 };
8
9 myIssue.title = "Two"// compile-time error: readonly property

```
TypeTis assignable (see Assignability ) toReadonly<T>, and allows assignments as a consequence:
```
1 class A {
2 f1:string= ""
3 f2:number= 1
4 f3:boolean=true
5 }
6 letx =newA
7 lety: Readonly<A> = x// OK

#### 5.3.6 Record Utility Type

```
TypeRecord<K, V>constructs a container that maps keys (of typeK) to values of typeV.
TypeKis restricted to numeric types (see Numeric Types ), typestring, string literal types and union types constructed
from these types.
```
```
5.3. Utility Types 77
```

```
A compile-time error occurs if any other type, or literal of any other type is used in place of this type.
Its usage is represented in the example below:
```
```
1 typeR1 = Record<number, Object> // ok
2 typeR2 = Record<boolean, Object> // compile-time error
3 typeR3 = Record<"salary" | "bonus", Object> // ok
4 typeR4 = Record<"salary" | boolean, Object> // compile-time error
5 typeR5 = Record<"salary" | number, Object> // ok
6 typeR6 = Record<string |number, Object> // ok
```
```
TypeVhas no restrictions.
A special form of object literals is supported for instances of typeRecord(see Object Literal of Record Type ).
Access toRecord<K, V>values is performed by an indexing expression like r[index] , where r is an instance of type
Record, and index is the expression of typeK(see Record Indexing Expression for detail).
Variables of typeRecord<K, V>can be initialized by a valid object literal of Record type (see Object Literal of
Record Type ) where the literal is valid if the type of key expression is compatible with key typeK, and the type of value
expression is compatible with the value typeV.
```
1 typeKeys = 'key1' |'key2' |'key3'
2
3 letx: Record<Keys, number> = {
4 'key1': 1,
5 'key2': 2,
6 'key3': 4,
7 }
8 console.log(x['key2']) // prints 2
9 x['key2'] = 8
10 console.log(x['key2']) // prints 8

```
In the example above,Kis a union of literal types and thus the result of an indexing expression is of typeV. In this case
it isnumber.
```
#### 5.3.7 Utility Type Private Fields

```
Utility types are built on top of other types. Private fields of the initial type stay in the utility type but they are not
accessible (see Accessible ) and cannot be accessed in any way. It is represented in the example below:
```
1 functionfoo(): string{ // Potentially some side effect
2 return"private field value"
3 }
4
5 classA {
6 public_field = 444
7 privateprivate_field = foo()
8 }
9
10 functionbar (part_a:Readonly<A>) {
11 console.log (part_a)
(continues on next page)

```
78 Chapter 5. Generics
```

```
(continued from previous page)
```
12 }
13
14 bar ({public_field: 777 }) // OK, object literal has no field`private_field`
15 bar ({public_field: 777 , private_field: ""})// compile-time error, incorrect field name
16
17 bar (newA) // OK, object of type Readonly<A> has field`private_field`

```
5.3. Utility Types 79
```

**80 Chapter 5. Generics**


##### CHAPTER

### SIX

### CONTEXTS AND CONVERSIONS

```
This Chapter defines expression contexts and conversions that can be applied to expressions in different contexts.
Contexts can be of the following kinds:
```
- _Assignment-like Contexts_ ;
- _String Operator Contexts_ withstringconcatenation (operator ‘+’);
- _Numeric Operator Contexts_ with all numeric operators (’+’, ‘-’, etc.).

### 6.1 Assignment-like Contexts

```
Assignment-like contexts include the following:
```
- _Declaration contexts_ that allow setting an initial value to a variable (see _Variable Declarations_ ), a constant (see
    _Constant Declarations_ ), or a field (see _Field Declarations_ ) with an explicit type annotation;
- _Assignment contexts_ that allow assigning (see _Assignment_ ) an expression value to a variable;
- _Call contexts_ that allow assigning an argument value to a corresponding formal parameter of a function, method,
    constructor or lambda call (see _Function Call Expression_ , _Method Call Expression_ , _Explicit Constructor Call_ ,
    and _New Expressions_ );
- _Return contexts_ (see _return Statements_ ) the allow specifying a resultant value of a function, method or lambda
    call;
- _Composite literal contexts_ that allow setting an expression value to an array element (see _Array Literal Type_
    _Inference from Context_ ), a class, or an interface field (see _Object Literal_ );
The examples are presented below:

1 // declaration contexts:
2 letx:number = 1
3 conststr:string = "done"
4 classC {
5 f: string= "aa"
6 }
7
8 // assignment contexts:
9 x = str.length
10 newC().f = "bb"
(continues on next page)

##### 81


```
(continued from previous page)
```
11 functionfoo<T1, T2> (p1:T1, p2: T2) {
12 lett1:T1 = p1
13 lett2:T2 = p2
14 }
15
16 // call contexts:
17 functionfoo(s:string) {}
18 foo("hello")
19
20 // composite literal contexts:
21 leta:number[] = [str.length, 11]

```
In all these cases, the expression type must be assignable to the target type (see Assignability ). Assignability allows
using of one of Implicit Conversions. If there is no applicable conversion, then a compile-time error occurs.
```
### 6.2 String Operator Contexts

```
String context applies only to a non- string operand of the binary operator ‘+’ if the other operand isstring.
String conversion for a non-stringoperand is evaluated as follows:
```
- An operand of an integer type (see _Integer Types and Operations_ ) is converted to typestringwith a value that
    represents the operand in the decimal form.
- An operand of a floating-point type (see _Floating-Point Types and Operations_ ) is converted to typestringwith
    a value that represents the operand in the decimal form without the loss of information.
- An operand of typebooleanis converted to typestringwith the valuestrueorfalse.
- An operand of enumeration type (see _Enumerations_ ) is converted to typestringwith the value of the corre-
    sponding enumeration constant if values of enumeration are of typestring.
- The operand of a nullish type that has a nullish value is converted as follows:
    **-** Operandnullis converted to stringnull.
    **-** Operandundefinedis converted to stringundefined.
- An operand of a reference type or anenumtype with non- _string_ values is converted by applying the method call
    toString().
If there is no applicable conversion, then a compile-time error occurs.
The target type in this context is alwaysstring:

```
1 console.log("" +null) // prints "null"
2 console.log("value is " + 123)// prints "value is 123"
3 console.log("BigInt is " + 123n) // prints "BigInt is 123"
4 console.log(15 + " steps")// prints "15 steps"
5 letx: string| null= null
6 console.log("string is " + x)// prints "string is null"
```
```
82 Chapter 6. Contexts and Conversions
```

### 6.3 Numeric Operator Contexts

```
Numeric contexts apply to the operands of an arithmetic operator. Numeric contexts use numeric types conversions
(see Widening Numeric Conversions ), and ensure that each argument expression can be converted to target typeTwhile
the arithmetic operation for the values of typeTis being defined.
An operand of enumeration type (see Enumerations ) can be used in a numeric context if enumeration base type is a
numeric type. The type of this operand is assumed to be the same as the enumeration base type.
Numeric contexts take the following forms:
```
- _Unary Expressions_ ;
- _Multiplicative Expressions_ ;
- _Additive Expressions_ ;
- _Shift Expressions_ ;
- _Relational Expressions_ ;
- _Equality Expressions_ ;
- _Bitwise and Logical Expressions_ ;
- _Conditional-And Expression_ ;
- _Conditional-Or Expression_.

#### 6.3.1 Numeric Conversions for Relational and Equality Operands

```
Relational and equiality operators (see Relational Expressions and Equality Expressions ) allow the following:
```
- _Implicit conversion_ , where operands are ofnumeric typesbut have different sizes (see _Widening Numeric_
    _Conversions_ ), with their specific details stated in _Specifics of Numeric Operator Contexts_ ; and
- Conversion of operands withBigInt()function, where one operand type isbigintand the other isnumeric.
    The situation for the relational operator ‘<’ is represented in the example below:

1 leta: int= 1
2 letb: long= 0
3 letc: bigint= -1n
4
5 if (b<a) {// `a``converted to`long` prior to comparison
6 ;
7 }
8
9 if (c<b) {// `b` converted to`bigint` prior to comparison
10 ;
11 }

```
6.3. Numeric Operator Contexts 83
```

### 6.4 Implicit Conversions

```
This section describes all implicit conversions that are allowed. Each conversion is allowed in a particular context
(e.g., if an expression that initializes a local variable is subject to Assignment-like Contexts , then the rules of this
context define what specific conversion is implicitly chosen for the expression).
```
#### 6.4.1 Widening Numeric Conversions

```
Widening numeric conversions convert the following:
```
- Values of a smaller numeric type to a larger type (see _Numeric Types_ );
- Values of _enumeration_ type (if enumeration constants of this type are of a numeric type) to the same or a larger
    numeric type.

```
From To
byte short,int,long,float,double
short int,long,float,double
int long,float, ordouble
long floatordouble
float double
enumeration with numeric constants larger numeric type
```
```
The above conversions cause no loss of information about the overall magnitude of a numeric value. Some least
significant bits of the value can be lost only in conversions from an integer type to a floating-point type if the IEEE 754
round-to-nearest mode is used correctly. The resultant floating-point value is properly rounded to the integer value.
Widening numeric conversions never cause runtime errors.
```
#### 6.4.2 Enumeration to Constants Type Conversions

```
The following conversions never cause a runtime error:
```
- Value of _enumeration_ type without explicit base type is converted to the corresponding integer type (see _Enu-_
    _merations_ ).

1 enum IntegerEnum {a, b, c}
2 letint_enum:IntegerEnum= IntegerEnum.a
3 letint_value: int= int_enum// int_value will get the value of 0
4 letnumber_value:number = int_enum
5 /* number_value will get the value of 0 as a result of conversion
6 sequence: enumeration -> int - > number */

```
A value of enumeration type withstringconstants is converted to typestring. This conversion never causes a
runtime error.
```
```
84 Chapter 6. Contexts and Conversions
```

1 enum StringEnum {a = "a", b = "b", c = "c"}
2 letstring_enum:StringEnum = StringEnum.a
3 leta_string:string = string_enum// a_string will get the value of "a"

```
A value of enumeration type with an explicitly declared type of constants is converted to the declared type. This
conversion never causes a runtime error.
```
1 enum DoubleEnum:double {a = 1.0, b = 2.0, c = 3.141592653589}
2 letdbl_enum:DoubleEnum = DoubleEnum.a
3 letdbl_value: double= dbl_enum // dbl_value will get the value of 1.0

### 6.5 Numeric Casting Conversions

```
A numeric casting conversion occurs if the target type and the expression type are bothnumeric. The context for
a numeric casting conversion is where conversion methods are used as defined in the standard library (see Standard
Library ).
The explicit use of methods for numeric cast conversions is represented in the following example:
```
1 functionprocess_int(an_int: int) {/* ... */ }
2
3 letpi = 3.14
4 process_int(pi.toInt())

```
A numeric casting conversion never causes a runtime error.
Numeric casting conversion of an operand of typedoubleto target typefloatis performed in compliance with the
IEEE 754 rounding rules. This conversion can lose precision or range, resulting in the following:
```
- Float zero from a nonzero double; and
- Float infinity from a finite double.
DoubleNaNis converted to floatNaN.
Double infinity is converted to the same-signed floating-point infinity.
A numeric conversion of a floating-point type operand to target typeslongorintis performed by the following rules:
- If the operand isNaN, then the result is 0 (zero).
- If the operand is positive infinity, or if the operand is too large for the target type, then the result is the largest
representable value of the target type.
- If the operand is negative infinity, or if the operand is too small for the target type, then the result is the smallest
representable value of the target type.
- Otherwise, the result is the value that rounds toward zero by using IEEE 754 _round-toward-zero_ mode.
A numeric casting conversion of a floating-point type operand to typesbyteorshortis performed in two steps as
follows:
- The casting conversion tointis performed first (see above);
- Then, theintoperand is cast to the target type.

```
6.5. Numeric Casting Conversions 85
```

A numeric casting conversion from an integer type to a smaller integer typeIdiscards all bits except the _N_ lowest ones,
where _N_ is the number of bits used to represent typeI. This conversion can lose the information on the magnitude of
the numeric value. The sign of the resulting value can differ from that of the original value.

**86 Chapter 6. Contexts and Conversions**


##### CHAPTER

### SEVEN

### EXPRESSIONS

This Chapter describes the meanings of expressions and the rules for the evaluation of expressions, except the expres-
sions related to coroutines (see _Coroutines (Experimental)_ ) and expressions described as experimental (see _Lambda
Expressions with Receiver_ ).

The syntax of _expression_ is presented below:

expression:
primaryExpression
|instanceOfExpression
|castExpression
|typeOfExpression
|nullishCoalescingExpression
|spreadExpression
|unaryExpression
|binaryExpression
|assignmentExpression
|ternaryConditionalExpression
|stringInterpolation
|lambdaExpression
|lambdaExpressionWithReceiver
|awaitExpression
;
primaryExpression:
literal
|namedReference
|arrayLiteral
|objectLiteral
|recordLiteral
|thisExpression
|parenthesizedExpression
|methodCallExpression
|fieldAccessExpression
|indexingExpression
|functionCallExpression
|newExpression
|ensureNotNullishExpression
;
binaryExpression:
multiplicativeExpression
|additiveExpression
|shiftExpression
(continues on next page)

##### 87


```
(continued from previous page)
|relationalExpression
|equalityExpression
|bitwiseAndLogicalExpression
|conditionalAndExpression
|conditionalOrExpression
;
```
The syntax below introduces several productions to be used by other expression syntax rules:

objectReference:
typeReference
|'super'
|primaryExpression
;

objectReferencerefers to one of the following three options:

- Class that is to handle static members;
- superthat is to access constructors declared in the superclass, or the overridden method version of the superclass;
- _primaryExpression_ that is to refer to a variable after evaluation, unless the manner of the evaluation is altered by
    the chaining operator ‘?.’ (see _Chaining Operator_ ).

If the form of _primaryExpression_ is _thisExpression_ , then the pattern “this?.” is handled as a compile-time error.

If the form of _primaryExpression_ is _super_ , then the pattern “super?.” is handled as a compile-time error.

The syntax of _arguments_ is presented below:

arguments:
'(' argumentSequence?')'
;

argumentSequence:
restArgument
|expression (','expression)* (','restArgument)? ','?
;

restArgument:
'...'? expression
;

The _arguments_ grammar rule refers to the list of call arguments. Only the last argument can have the form of a spread
expression (see _Spread Expression_ ).

### 7.1 Evaluation of Expressions

The result of a program expression _evaluation_ denotes the following:

- Variable (the term _variable_ is used here in the general, non-terminological sense to denote a modifiable lvalue
    in the left-hand side of an assignment); or

**88 Chapter 7. Expressions**


- Value (results found elsewhere).
A variable or a value are equally considered the _value of the expression_ if such a value is required for further evaluation.
The type of an expression is determined at compile time (see _Type of Expression_ ).
Expressions can contain assignments, increment operators, decrement operators, method calls, and function calls. The
evaluation of an expression can produce side effects as a result.
_Constant expressions_ (see _Constant Expressions_ ) are the expressions with values that can be determined at compile
time.

#### 7.1.1 Type of Expression

```
Every expression in the ArkTS programming language has a type. The type of an expression is determined at compile
time.
In most contexts, an expression must be compatible with the type expected in a context. This type is called target type.
If no target type is available in a context, then the expression is called a standalone expression :
```
1 leta = expr// no target type is available
2
3 functionfoo() {
4 expr// no target type is available
5 }

```
Otherwise, the expression is non-standalone :
```
1 leta: number= expr // target type of'expr'is number
2
3 functionfoo(s: string) {}
4 foo(expr) // target type of'expr'is string

```
In some cases, the type of an expression cannot be inferred (see Type Inference ) from the expression itself (see Object
Literal as an example). If such an expression is used as a standalone expression , then a compile-time error occurs:
```
1 class P { x:number, y:number}
2
3 letx = { x: 10 , y: 10 }// standalone object literal - compile time error
4 lety: P= { x: 10 , y: 10 } // OK, type of object literal is inferred

```
The evaluation of an expression type requires completing the following steps:
```
1. Collect information for type inference (type annotation, generic constraints, etc);
2. Perform _Type Inference_ ;
3. If the expression type is not yet inferred at a previous step, and the expression is a literal in the general sense,
    including _Array Literal_ , then an attempt is made to evaluate the type from the expression itself.
A compile-time error occurs if none of these steps produces an appropriate expression type.
If the expression type isreadonly, then the target type must also bereadonly. Otherwise, a compile-time error
occurs:

```
7.1. Evaluation of Expressions 89
```

1 letreadonly_array:readonly number[] = [1, 2, 3]
2
3 foo1(readonly_array)// OK
4 foo2(readonly_array)// compile-time error
5
6 functionfoo1 (p:readonly number[]) {}
7 functionfoo2 (p:number[]) {}
8
9 letwritable_array:number [] = [1, 2, 3]
10 foo1 (writable_array)// OK, as always safe

#### 7.1.2 Normal and Abrupt Completion of Expression Evaluation

```
Each expression in a normal mode of evaluation requires certain computational steps. Normal modes of evaluation for
each kind of expression are described in the following sections.
An expression evaluation completes normally if all computational steps are performed without throwing an error.
On the contrary, an expression evaluation completes abruptly if an error is thrown in the process. The information on
the cause of an abrupt completion is provided in the value attached to the error object.
Runtime errors can occur as a result of expression or operator evaluation as follows:
```
- If the value of an array index expression is negative, or greater than, or equal to the length of the array, then an
    _array indexing expression_ (see _Array Indexing Expression_ ) throwsRangeError.
- If the type of a value being assigned to a fixed-size array element is not a subtype of an array element type, then
    an _Assignment_ throws _ArrayStoreError_.
- If a _Cast Expression_ conversion cannot be performed at runtime, then it throwsClassCastError.
- If a right-hand expression has the zero value, then the integer division or integer remainder (see _Division_ and
    _Remainder_ ) operator throwsArithmeticError.
An error during the evaluation of an expression can be caused by a possible hard-to-predict and hard-to-handle linkage
and virtual machine error.
Abrupt completion of the evaluation of a subexpression results in the following:
- Immediate abrupt completion of an expression that contains the subexpression (if the evaluation of the contained
subexpression is required for the evaluation of the entire expression); and
- Cancellation of all subsequent steps of the normal mode of evaluation.
The terms _complete normally_ and _complete abruptly_ can also denote normal and abrupt completion of the execution
of a statement (see _Normal and Abrupt Statement Execution_ ). A statement can complete abruptly for many reasons in
addition to an error being thrown.

```
90 Chapter 7. Expressions
```

#### 7.1.3 Order of Expression Evaluation

The operands of an operator are evaluated from left to right in accordance with the following rules:

- The order of evaluation depends on the assignment operator (see _Assignment_ ).
- Any right-hand expression is evaluated only after the left-hand expression of a binary operator is fully evaluated.
- Any part of the operation can be executed only after every operand of an operator (except conditional operators
    ‘&&’, ‘||’, and ‘? :‘) is fully evaluated.
    The execution of a binary operator that is an integer division ‘/’ (see _Division_ ), or integer remainder ‘%’ (see
    _Remainder_ ) can throwArithmeticErroronly after the evaluations of both operands complete normally.
- The ArkTS programming language follows the order of evaluation as indicated explicitly by parentheses, and im-
    plicitly by the precedence of operators. This rule particularly applies for infinity andNaNvalues of floating-point
    calculations. ArkTS considers integer addition and multiplication as provably associative. However, floating-
    point calculations must not be naively reordered because they are unlikely to be computationally associative (even
    though they appear mathematically associative).

#### 7.1.4 Operator Precedence

The table below summarizes the entire information on the precedence and associativity of operators. Each section on
a particular operator also contains detailed information.

```
Operator Precedence Associativity
grouping () n/a
member access and chaining. ?. left-to-right
access and call []. () new n/a
postfix increment and decrement ++ -- n/a
prefix increment and decrement, unary
operators, typeof, await
```
```
++ -- + -! ~ typeof await n/a
```
```
exponentiation ** right-to-left
multiplicative * / % left-to-right
additive + - left-to-right
cast as left-to-right
shift << >> >>> left-to-right
relational < > <= >= instanceof left-to-right
equality == != left-to-right
bitwise AND & left-to-right
bitwise exclusive OR ^ left-to-right
bitwise inclusive OR | left-to-right
logical AND && left-to-right
logical OR || left-to-right
null-coalescing ?? left-to-right
ternary condition?whenTrue:whenFalse right-to-left
assignment = += -= %= *= /= &= ^= |= <<= >>= >>>= right-to-left
```
**7.1. Evaluation of Expressions 91**


#### 7.1.5 Evaluation of Arguments

An evaluation of arguments always progresses from left to right up to the first error, or through the end of the expression;
i.e., any argument expression is evaluated after the evaluation of each argument expression to its left completes normally
(including comma-separated argument expressions that appear within parentheses in method calls, constructor calls,
class instance creation expressions, or function call expressions).

If the left-hand argument expression completes abruptly, then no part of the right-hand argument expression is evalu-
ated.

#### 7.1.6 Evaluation of Other Expressions

These general rules cannot cover the order of evaluation of certain expressions when they from time to time cause
exceptional conditions. The order of evaluation of the following expressions requires specific explanation:

- Class instance creation expressions (see _New Expressions_ );
- _Resizable Array Creation Expressions_ ;
- _Indexing Expressions_ ;
- Method call expressions (see _Method Call Expression_ );
- Assignments involving indexing (see _Assignment_ );
- _Lambda Expressions_.

### 7.2 Literal

_Literals_ (see _Literals_ ) denote fixed and unchanging values. Type of a literal is the type of an expression.

### 7.3 Named Reference

An expression can have the form of a _named reference_ as described by the syntax rule as follows:

namedReference:
qualifiedName typeArguments?
;

Type of a _named reference_ expression is the type of the entity to which a _named reference_ refers.

_QualifiedName_ (see _Names_ ) is an expression that consists of dot-separated names. If _qualifiedName_ consists of a single
identifier, then it is called a _simple name_.

_Simple name_ refers to the following:

- Entity declared in the current module;

**92 Chapter 7. Expressions**


- Local variable or parameter of the surrounding function or method.
If not a _simple name_ , _qualifiedName_ refers to the following:
- Entity imported from a module,
- Entity exported from a namespace, or
- Member of some class or interface, or
- Special syntax form of _Record Indexing Expression_.
If _typeArguments_ are provided, then _qualifiedName_ is a valid instantiation of the generic method or function. Otherwise,
a compile-time error occurs.
A compile-time error also occurs if a name referred by _qualifiedName_ is one of the following:
- Undefined or inaccessible;
- Named constructor (see _Constructor Names_ ).
Type of a _named reference_ is the type of an expression.
If a _named reference_ refers to a function name, it is called _Function Reference_. If a _named reference_ refers to a method
name, it is called _Method Reference_.

#### 7.3.1 Function Reference

```
A function reference refers to a declared or imported function. Type of a function reference is derived from the function
signature:
```
1 functionfoo(n: number): string{ return n.toString() }
2 letfunc = foo// type of func is '(n: number) => string'
3 letx = func(1) // foo() called via reference

```
A function reference can refer to a generic function but only if Explicit Generic Instantiations is present, otherwise a
compile-time error occurs:
```
1 functiongen<T> (x: T) {}
2
3 leta = gen<string> // ok
4 letb = gen// compile-time error: no explicit type arguments

```
A compile-time error occurs if an overload alias is used in a named reference:
```
1 functionfoo1(n:number) {}
2 functionfoo2(s:string) {}
3 overload foo { foo1, foo2 }
4
5 foo(1) // OK, overload call
6 letx = foo // Error: ref to overload
7 lety = foo2 // ok, ref to foo2

```
7.3. Named Reference 93
```

#### 7.3.2 Method Reference

```
A method reference refers to a static or instance method of a class or an interface. Type of a method reference is derived
from the method signature:
```
1 class C {
2 staticfoo(n: number) {}
3 bar (s:string): boolean{ return true}
4 }
5
6 // Method reference to a static method
7 const m1 = C.foo // type of 'm1'is (n: number) => void
8
9 // Method reference to an instance method
10 const m2 =newC().bar// type of'm1'is (s: string) => boolean

```
If method reference refers to an instance method, that the named reference is bounded with the used instance of that
class or interface.
```
1 class C {
2 field = 123
3 method():number { return this.field }
4 }
5 letc1 =newC
6 letc2 =newC
7 letm1 = c1.method //'c1'is bounded
8 letm2 = c2.method //'c2'is bounded
9 c1.field = 42
10 console.log (m1(), m2())// Outputs: 42 123

```
A method reference can refer to a generic method only if a generic instantiation is explicitly present (see Explicit Generic
Instantiations ). Otherwise, a compile-time error occurs:
```
```
1 class C {
2 gen<T> (x: T) {}
3 }
4
5 leta =newC().gen<string> // ok
6 letb =newC().gen// compile-time error: no explicit type arguments
```
```
A compile-time error occurs if a method overload alias is used in a named reference:
```
```
1 class C {
2 foo1(n:number) {}
3 foo2(s:string) {}
4 overload foo { foo1, foo2 }
5 }
6
7 letf =newC().foo// compile-time error
```
```
94 Chapter 7. Expressions
```

### 7.4 Array Literal

```
Array literal is an expression that can be used to create an array or tuple in some cases, and to provide some initial
values.
The syntax of array literal is presented below:
```
```
arrayLiteral:
'[' expressionSequence?']'
;
```
```
expressionSequence:
expression(',' expression)* ','?
;
```
```
An array literal is a comma-separated list of initializer expressions enclosed in square brackets ‘[’ and ‘]’. A trailing
comma after the last expression in an array literal is ignored:
```
1 letx = [1, 2, 3]// ok
2 lety = [1, 2, 3,] // ok, trailing comma is ignored

```
The number of initializer expressions enclosed in square brackets of the array initializer determines the length of the
array to be constructed.
If memory is allocated as required for an array literal, then an array of the specified length is created, and all elements
of the array are initialized to the values specified by initializer expressions.
On the contrary, the evaluation of an array literal expression completes abruptly if:
```
- Not enough memory is available for a new array, andOutOfMemoryErroris thrown; or
- Some initialization expression completes abruptly.
Initializer expressions are executed from left to right. The _n_ ’th expression specifies the value of the _n-1_ ’th element of
the array.
Array literals can be nested (i.e., the initializer expression that specifies an array element can be an array literal if that
element is of array type).
Type of an _array literal expression_ is inferred by the following rules:
- If a context is available, then type is inferred from the context. If successful, then type of an array literal is the
inferred typeT[],Array<T>, or tuple.
- Otherwise, type is inferred from the types of array literal elements.
More details of both cases are presented below.

#### 7.4.1 Array Literal Type Inference from Context

```
Type of an array literal can be inferred from the context , including explicit type annotation of a variable declaration,
left-hand part type of an assignment, call parameter type, or type of a cast expression:
```
1 leta: number[] = [1, 2, 3] // ok, variable type is used
2 a = [4, 5] // ok, variable type is used
(continues on next page)

```
7.4. Array Literal 95
```

(continued from previous page)
3
4 functionmin(x: number[]):number {
5 letm = x[0]
6 for(letvof x)
7 if (v < m)
8 m = v
9 returnm
10 }
11 min([1., 3.14, 0.99]); // ok, parameter type is used
12
13 // Array of array initialization
14 typeMatrix =number[][]
15 letm: Matrix= [[1, 2], [3, 4], [5, 6]]
16
17 class aClass {}
18 letb1:Array<aClass> = [newaClass,newaClass]
19 letb2:Array<number> = [1, 2, 3]
20 letb3:FixedArray<number> = [1, 2]
21 /* Type of literal is inferred from the context
22 taken from b1, b2 and b3 declarations */

```
Possible kinds of context are represented in the following example:
```
```
1 letarray: number[] = [1, 2, 3] // assignment context
2 functionfoo (array:number[]) {}
3 foo ([1, 2, 3]) // call context
4 letb = [1, 2, 3]as number[] // casting conversion
```
```
All valid conversions are applied to the initializer expression, i.e., each initializer expression type must be assignable
(see Assignability ) to the array element type. Otherwise, a compile-time error occurs.
```
```
1 letvalue: number= 2
2 letlist: Object[] = [1, value, "hello",newError()]// ok
```
```
If the type used in the context is a tuple type (see Tuple Types ), and types of all literal expressions are compatible with
tuple type elements at respective positions, then an array literal is of tuple type.
```
```
1 lettuple: [number, string] = [1, "hello"]// ok
2
3 letincorrect: [number, string] = ["hello", 1]// compile-time error
```
```
If the type used in the context is a union type (see Union Types ), then it is necessary to try inferring the type of the array
literal from its elements (see Array Type Inference from Types of Elements ). If successful, then the type so inferred
must be compatible with one of the types that form the union type. Otherwise, a compile-time error occurs:
```
```
1 letunion_of_arrays_int:int[] |string[] = [1, 2]// OK, literal is int[]
2 // Compatible with union
3 letunion_of_arrays:number[] | string[] = [1, 2]// Error, literal is int[]
4 // incompatible with union
5 letincorrect_union_of_arrays:number[] |string[] = [1, 2, "string"]
6 /* Error: (number|string)[] (type of the literal) is not compatible with
7 number[] | string[] (type of the variable)
8 */
```
```
96 Chapter 7. Expressions
```

```
If the type used in the context is a fixed-size array type (see Fixed-Size Array Types ), and each initializer expression
type is compatible with the array element type, then an array literal is of fixed-size array type.
```
1 letarray: FixedArray<number> = [1, 2]

```
If the type used in the context is a readonly array, then an array literal is of readonly array type.
```
#### 7.4.2 Array Type Inference from Types of Elements

```
Where no context is set, and thus the type of an array literal cannot be inferred from the context (see Type of Expression ),
the type of array literal[ expr 1 ,...,exprN]is inferred from the initialization expression instead by using the
following algorithm:
```
1. If array literal ( _N == 0_ ) includes no element, then the type of the array literal cannot be inferred, and a compile-
    time error occurs.
2. If at least one element of an expression type cannot be determined, then the type of the array literal cannot be
    inferred, and a compile-time error occurs.
3. If each initialization expression is of a numeric type (see _Numeric Types_ ), then the array literal type isnumber[].
4. If all initialization expressions are of the same typeT, then the array literal type isT[].
5. Otherwise, the array literal type is constructed as the union typeT:sub:1| ... | TN, whereTiis the type of
    _expr_ i, and then:
       - IfTiis a literal type, then it is replaced for its supertype;
       - IfTiis a union type comprised of literal types, then each constituent literal type is replaced for its supertype.
       - _Union Types Normalization_ is applied to the resultant union type after the above replacements.

1 typeA =number
2 letu : "A" | "B" = "A"
3
4 leta = [] // compile-time error, type cannot be inferred
5 letb = ["a"] // type is string[]
6 letc = [1, 2, 3] // type is number[]
7 letd = ["a" + "b", 1, 3.14] // type is (string | number)[]
8 lete = [u] // type is string[]
9 letf = [():void=> {}, newA()]// type is (() => void | A)[]

### 7.5 Object Literal

```
Object literal is an expression that can be used to create a class instance with methods and fields with initial values.
In some cases it is more convenient to use an object literal in place of a class instance creation expression (see New
Expressions ).
The syntax of object literal is presented below:
```
```
7.5. Object Literal 97
```

```
objectLiteral:
'{' objectLiteralMembers? '}'
;
```
```
objectLiteralMembers:
objectLiteralMember(','objectLiteralMember)* ','?
;
```
```
objectLiteralMember:
objectLiteralField
;
```
```
objectLiteralField:
identifier':'expression
;
```
```
An object literal field consists of an identifier and an expression as follows:
```
1 class Person {
2 name:string = ""
3 age:number= 0
4 }
5 letb: Person= {name: "Bob", age: 25 }
6 leta: Person= {name: "Alice", age: 18 , }//ok, trailing comma is ignored
7 letc: Person| number = {name: "Mary", age: 17 } // literal will be of type Person

```
An object literal method is a complete declaration of a public method. Examples of object literals with methods are
provided in Object Literal of Interface Type.
Type of an object literal expression is always some classCthat is inferred from the context. A type inferred from the
context can be either a class (see Object Literal of Class Type ), or an anonymous class created for the inferred interface
type (see Object Literal of Interface Type ).
A compile-time error occurs if:
```
- Type of an object literal cannot be inferred from the context (see _Type of Expression_ for an example);
- Inferred type is not a class or interface type, or is an abstract class type (see _Abstract Classes_ );
- Inferred type is not an interface type, and an object literal contains methods;
- Context is a union type, and an object literal can be treated as the value of several union component types.

1 letp = {name: "Bob", age: 25 }
2 // compile-time error, type cannot be inferred
3
4 class A { field = 1 }
5 class B { field = 2 }
6
7 letq: A| B = {field: 6 }
8 // compile-time error, type cannot be inferred as the literal
9 // fits both A and B

```
98 Chapter 7. Expressions
```

#### 7.5.1 Object Literal of Class Type

```
If class typeCis inferred from the context, then type of an object literal isC:
```
1 class Person {
2 name:string = ""
3 age:number= 0
4 }
5 functionfoo(p: Person) {/*some code*/ }
6 // ...
7 letp: Person= {name: "Bob", age: 25 }/* ok, variable type is
8 used */
9 foo({name: "Alice", age: 18 }) // ok, parameter type is used

```
An identifier in each name-value pair must name a field of classC, or a field of any superclass of classC.
A compile-time error occurs if the identifier does not name an accessible member field (see Accessible ) in typeC:
```
1 class Friend {
2 name:string = ""
3 privatenick:string = ""
4 age:number
5 sex?: "male"|"female"
6 }
7 // compile-time error, nick is private:
8 letf: Friend= {name: "Alexander", age: 55 , nick: "Alex"}

```
A compile-time error occurs if type of an expression in a name-value pair is not assignable (see Assignability ) to the
field type:
```
1 letf: Friend= {name: 123 } /* compile-time error - type of right hand-side
2 is not assignable to the type of the left hand-side */

```
If some class fields have default values (see Default Values for Types ) or explicit initializers (see Variable and Constant
Declarations ), then such fields can be skipped in the object literal.
```
1 letf: Friend= {} /* OK, as name, nick, age, and sex have either default
2 value or explicit initializer */

```
If an object literal is to use classC, then classCmust have a parameterless constructor (explicit or default) that is
accessible (see Accessible ) in the class-composite context.
A compile-time error occurs if:
```
- Ccontains no parameterless constructor; or
- No constructor is accessible (see _Accessible_ ).
These situations are presented in the examples below:

1 class C {
2 constructor(x:number) {}
3 }
4 // ...
5 letc: C= {}/* compile-time error - no parameterless
6 constructor */

```
7.5. Object Literal 99
```

1 class C {
2 private constructor() {}
3 }
4 // ...
5 letc: C= {}/* compile-time error - constructor is not
6 accessible */

```
If a class has accessors (see Class Accessor Declarations ) for a property, and its setter is provided, then this property
can be used as a part of an object literal. Otherwise, a compile-time error occurs:
```
1 class OK {
2 set attr (attr: number) {}
3 }
4 const a:OK = {attr: 42 }// OK, as the setter be called
5
6 class Err {
7 get attr ():number {return 42 }
8 }
9 const b:Err= {attr: 42 }// compile-time error - no setter for'attr'

#### 7.5.2 Object Literal of Interface Type

```
If an interface typeIis inferred from the context, then type of an object literal is an anonymous class implicitly created
for interfaceI:
```
1 interface Person {
2 name:string
3 age:number
4 }
5 letb: Person= {name: "Bob", age: 25 }

```
In the example above, type of b is an anonymous class that contains the same fields as the interfaceIproperties.
Any properties that are optional can be skipped in an object literal. The values of such optional properties are set to
undefinedas follows:
```
1 interface Person {
2 name:string
3 age:number
4 sex?: "male"|"female"
5 }
6 letb: Person= {name: "Bob", age: 25 }
7 // 'sex'field will have'undefined'value

```
Properties that are non-optional cannot be skipped in an object literal, despite some property types having default
values (see Default Values for Types ). If a non-optional property (e.g., age in the example above) is skipped, then a
compile-time error occurs.
A compile-time error occurs if an object literal of interface type introduces a new method:
```
```
100 Chapter 7. Expressions
```

```
1 interface I {}
2 const i:I = { foo():void{} } // compile-time error
```
```
If an interface has accessors (see Interface Properties ) for some property, and the property is used in an object literal,
then a compile-time error occurs:
```
1 interface I1 {
2 set attr (attr: number)
3 }
4 const a:I1 = {attr: 42 }/* compile-time error - 'attr'cannot be used
5 in object literal */
6
7 interface I2 {
8 get attr ():number
9 }
10 const b:I2 = {attr: 42 }/* compile-time error - 'attr'cannot be used
11 in object literal */

#### 7.5.3 Object Literal ofRecordType

```
Generic typeRecord<Key, Value>(see Record Utility Type ) is used to map properties of a type (typeKey) to another
type (typeValue). A special form of object literal is used to initialize the value of such type:
The syntax of record literal is presented below:
```
```
recordLiteral:
'{' keyValueSequence? '}'
;
```
```
keyValueSequence:
keyValue(',' keyValue)*','?
;
```
```
keyValue:
expression':'expression
;
```
```
The first expression inkeyValuedenotes a key and must be of typeKey. The second expression denotes a value and
must be of typeValue:
```
```
letmap:Record<string, number> = {
"John": 25,
"Mary": 21,
}
```
```
console.log(map["John"])// prints 25
```
```
interfacePersonInfo {
age:number
(continues on next page)
```
```
7.5. Object Literal 101
```

(continued from previous page)
salary:number
}
letmap:Record<string, PersonInfo> = {
"John": { age: 25 , salary: 10 },
"Mary": { age: 21 , salary: 20 }
}

If a key is a union of literal types, then all variants must be listed in the object literal. Otherwise, a compile-time error
occurs:

letmap:Record<"aa" | "bb", number> = {
"aa": 1,
}// compile-time error: "bb" key is missing

#### 7.5.4 Object Literal Evaluation

The evaluation of an object literal of typeC(whereCis either a named class type or an anonymous class type created
for the interface) is to be performed by the following steps:

- A parameterless constructor is executed to produce an instancexof classC. The execution of the object literal
    completes abruptly if so does the execution of the constructor.
- Name-value pairs of the object literal are then executed from left to right in the textual order they occur in the
    source code. The execution of a name-value pair includes the following:
       **-** Evaluation of the expression; and
       **-** Assignment of the value of expression to the corresponding field ofxas its initial value. This rule also
          applies toreadonlyfields.

The execution of an object literal completes abruptly if so does the execution of a name-value pair.

An object literal completes normally with the value of a newly initialized class instance if so do all name-value pairs.

### 7.6 Spread Expression

_Spread expression_ can be used only within an array literal (see _Array Literal_ ) or argument passing. The _expression_
must be of array type (see _Array Types_ ) or tuple type (see _Tuple Types_ ). Otherwise, a compile-time error occurs.

The syntax of _spread expression_ is presented below:

spreadExpression:
'...'expression
;

A _spread expression_ for arrays or tuples can be evaluated as follows:

- By the compiler at compile time if _expression_ is constant (see _Constant Expressions_ );

**102 Chapter 7. Expressions**


- At runtime otherwise.
An array or tuple object referred by the _expression_ is broken by the evaluation into a sequence of values. This sequence
is used where a _spread expression_ is used. It can be an assignment, a call of a function, method, or constructor. A
sequence of types of these values is the type of the _spread expression_.
A spread expression for arrays is represented in the example below:

1 letarray1 = [1, 2, 3]
2 letarray2 = [4, 5]
3 letarray3 = [...array1, ...array2] // spread array1 and array2 elements
4 // while building new array literal at compile time
5 console.log(array3) // prints [1, 2, 3, 4, 5]
6
7 functionfoo (...array: number[]) {
8 console.log (array)
9 }
10 foo (...array2) // spread array2 elements into arguments of the foo() call
11
12 functionrun_time_spread_application1 (a1:number[], a2:number[]) {
13 console.log ([...a1, 42, ...a2])
14 // array literal will be built at runtime
15 }
16 run_time_spread_application1 (array1, array2) // prints [1, 2, 3, 42, 4, 5]

```
A spread expression for tuples is represented in the example below:
```
1 lettuple1: [number,string, boolean] = [1, "2",true]
2 lettuple2: [number,string] = [4, "5"]
3 // spread tuple1 and tuple2 elements
4 lettuple3: [number,string, boolean,number, string] = [...tuple1, ...tuple2]
5 // while building new tuple object at compile time
6 console.log(tuple3) // prints [1, 2, true, 4, 5]
7
8 functionbar (...tuple: [number, string]) {
9 console.log (tuple)
10 }
11 bar (...tuple2) // spread tuple2 elements into arguments of the foo() call
12
13 functionrun_time_spread_application2 (a1: [number,string,boolean], a2: [number,␣
˓→string]) {
14 console.log ([...a1, 42, ...a2])
15 // such array literal will be built at runtime
16 }
17 run_time_spread_application2 (tuple1, tuple2) // prints [1, 2, true, 42, 4, "5"]

```
Note. If an argument is spread at the call site, then an appropriate parameter must be of the rest kind (see Rest Param-
eter ). A compile-time error occurs if an argument is spread into a sequence of ordinary non-optional parameters as
follows:
```
```
1 functionfoo1 (n1: number, n2:number)// non-rest parameters
2 { ... }
3 letan_array = [1, 2]
4 foo1 (...an_array) // compile-time error
5
(continues on next page)
```
```
7.6. Spread Expression 103
```

```
(continued from previous page)
```
6 functionfoo2 (n1: number, n2:string) // non-rest parameters
7 { ... }
8 leta_tuple: [number,string] = [1, "2"]
9 foo2 (...a_tuple)// compile-time error

### 7.7 Parenthesized Expression

```
The syntax of parenthesized expression is presented below:
```
```
parenthesizedExpression:
'(' expression')'
;
```
```
Type and value of a parenthesized expression are the same as those of the contained expression.
```
### 7.8 thisExpression

```
The syntax of this expression is presented below:
```
```
thisExpression:
'this'
;
```
```
The keywordthiscan be used as an expression in the body of an instance method of a class (see Method Body ) or
an interface (see Default Interface Method Declarations ). A corresponding class or interface type is the type of this
expression. If a method is declared in an object literal (see Object Literal ), then the type of the object literal is the type
ofthis.
The keywordthiscan be used in a lambda expression only if it is allowed in the context in which the lambda expression
occurs. The type ofthisis the type of a class or an interface in which it is declared, but not the type of the surrounding
object literal type, if any.
The keywordthisin a direct call expressionthis( arguments )can only be used in an explicit constructor call
statement (see Explicit Constructor Call ).
The keywordthiscan also be used in the body of a function with receiver (see Functions with Receiver ). The type of
this expression is the declared type of the parameterthisin a function.
A compile-time error occurs if the keywordthisappears elsewhere.
The keywordthisused as a primary expression denotes a value that is a reference to the following:
```
- Object for which the instance method is called; or
- Object being constructed.
The parameterthisin a lambda body and in the surrounding context denote the same value.

```
104 Chapter 7. Expressions
```

```
The class of the actual object referred to at runtime can beTifTis a class type, or a subclass ofT(see Subtyping ).
The semantics ofthisin different contexts is represented in the example below:
```
1 interface anInterface {
2 method() {
3 this// type of 'this'is anInterface
4 }
5 }
6 class aClassimplements anInterface {
7 method() {
8 this// type of 'this'is aClass
9 }
10 field = ():void=> {
11 this// type of 'this'is aClass
12 }
13 }
14 class AnotherClass {
15 anotherMethod() {
16 constobj: aClass= {// Object literal
17 method () {
18 this // type of'this'is aClass
19 },
20 field: ():void=> {
21 this // type of'this'is AnotherClass
22 }
23 }
24 }
25 }

### 7.9 Field Access Expression

```
Field access expression can access a field of an object to which an object reference refers. The object reference can
have different forms as described in detail in Accessing Current Object Fields and in Accessing SuperClass Properties.
The syntax of field access expression is presented below:
```
```
fieldAccessExpression:
objectReference ('.' |'?.') identifier
;
```
```
A field access expression that contains ‘?.’ (see Chaining Operator ) is called safe field access because it handles
nullish object references safely.
If object reference evaluation completes abruptly, then so does the entire field access expression.
An object reference used to access a field must be a non-nullish reference typeT. Otherwise, a compile-time error
occurs.
A field access expression is valid if the identifier refers to an accessible member field (see Accessible ) in typeT. A
compile-time error occurs otherwise.
```
```
7.9. Field Access Expression 105
```

```
Type of a field access expression is the type of a member field.
```
#### 7.9.1 Accessing Current Object Fields

```
The result of a field access expression is computed at runtime as described below.
a. Static field access ( objectReference is evaluated in the form typeReference )
The evaluation of typeReference is performed. The result of a field access expression of a static field in a class is as
follows:
```
- variableif the field is notreadonly. The resultant value can be changed later.
- valueif the field isreadonly, except where _field access_ occurs in a initializer block (see _Static Initialization_ ).
b. _Instance_ field access ( _objectReference_ is evaluated in the form _primaryExpression_ )
The evaluation of _primaryExpression_ is performed. The result of _field access expression_ of an instance field in a class
or interface is as follows:
- variableif the field is notreadonly. The resultant value can be changed later.
- valueif the field isreadonly, except where _field access_ occurs in a constructor (see _Constructor Declaration_ ).
Only the _primaryExpression_ type (not class type of an actual object referred at runtime) is used to determine the field
to be accessed.

#### 7.9.2 Accessing SuperClass Properties

```
The formsuper.identifieris valid when accessing the superclass property via accessor (see Class Accessor Dec-
larations ). A compile-time error occurs if identifier in ‘super.identifier’ denotes a field.
```
1 class Base {
2 get property(): number{ return1 }
3 set property(p: number) { }
4 field = 1234
5 }
6 class DerivedextendsBase {
7 get property(): number{ return super.property }// OK
8 set property(p: number) {super.property = 42 }// OK
9 foo () {
10 super.field = 42 // compile-time error
11 console.log (super.field) // compile-time error
12 }
13 }

```
106 Chapter 7. Expressions
```

### 7.10 Method Call Expression

A _method call expression_ calls a static or instance method of a class or an interface. Dynamic dispatch (see _Dispatch_ )
is used during program execution to perform a call in case of an instance method.

The syntax of _method call expression_ is presented below:

methodCallExpression:
objectReference ('.' |'?.') identifier typeArguments? arguments block?
;

The syntax form that has a block associated with the method call is a special form called _trailing lambda call_ (see
_Trailing Lambdas_ for details).

A method call with ‘?.’ (see _Chaining Operator_ ) is called a _safe method call_ because it handles nullish values safely.

There are several steps that determine and check the method to be called at compile time (see _Step 1: Selection of Type
to Use_ , _Step 2: Selection of Method_ , and _Step 3: Checking Method Modifiers_ ).

#### 7.10.1 Step 1: Selection of Type to Use

The _object reference_ is used to determine the type in which to search for the method. Three forms of _object reference_
are possible:

```
Form of Object Reference Type to Use
typeReference Type denoted bytypeReference.
expressionof type T TifTis a class, interface, or union;T’s constraint ( Type Parameter Constraint )
ifTis a type parameter. A compile-time error occurs otherwise.
super The superclass of the class that contains the method call.
```
#### 7.10.2 Step 2: Selection of Method

After the type to use is known, the method to call must be determined. If a method name in the call refers an _overload
declaration_ (see _Overload Declarations_ ), then _Overload Resolution_ is used to select the method to call. A compile-time
error occurs if no method is available to call.

#### 7.10.3 Step 3: Checking Method Modifiers

In this step, the single method to call is known, and the following set of semantic checks must be performed:

- If the method call has the formtypeReference.identifier, thentypeReferencerefers to a class, and the
    method must be declaredstatic. Otherwise, a compile-time error occurs.

**7.10. Method Call Expression 107**


- If the method call has the formexpression.identifier, then the method must not be declaredstatic.
    Otherwise, a compile-time error occurs.
- If the method call has the formsuper.identifier, then the method must not be declaredabstractorstatic.
    Otherwise, a compile-time error occurs.
A compile-time error occurs if a method has at least one parameter or return type of the type FixedArray parameterized
with a type parameter and _method call expression_ leads to instantiation of the type FixedArray with the predefined
value type (see _Value Types_ ).

#### 7.10.4 Type of Method Call Expression

```
Type of a method call expression is the return type of the method.
```
1 class A {
2 static method() { console.log ("Static method() is called") }
3 method() { console.log ("Instance method() is called") }
4 }
5
6
7 letx = A.method() // compile-time error as void cannot be used as type annotation
8 A.method () // OK
9 lety =newA().method()// compile-time error as void cannot be used as type annotation
10 newA().method() // OK

### 7.11 Function Call Expression

```
Function call expression is used to call a function (see Function Declarations ), a variable of a function type ( Function
Types ), or a lambda expression (see Lambda Expressions ).
The syntax of function call expression is presented below:
```
```
functionCallExpression:
expression('?.' |typeArguments)? arguments block?
;
```
```
A special syntactic form that contains a block associated with the function call is called trailing lambda call (see
Trailing Lambdas for details).
A compile-time error occurs if the expression type is one of the following:
```
- Different than the function type;
- Nullish type without ‘?.’ (see _Chaining Operator_ ).
If the operator ‘?.’ (see _Chaining Operator_ ) is present, and the _expression_ evaluates to a nullish value, then:
- _Arguments_ are not evaluated;
- Call is not performed; and

```
108 Chapter 7. Expressions
```

- Result of _functionCallExpression_ is not produced as a consequence.
The function call is _safe_ because it handles nullish values properly.
If the form of expression in the call is _qualifiedName_ , and _qualifiedName_ refers an _overload declaration_ ( _Overload
Declarations_ ), then _Overload Resolution_ is used to select the function to call.
A compile-time error occurs if no function is available to call.
A compile-time error occurs if a function has at least one parameter or return type of the type FixedArray parameterized
with a type parameter and _function call expression_ leads to instantiation of the type FixedArray with the predefined
value type (see _Value Types_ ).
Semantic check for call is performed in accordance with _Compatibility of Call Arguments_.
Various forms of function calls are represented in the example below:

1 functionfoo() { console.log ("Function foo() is called") }
2 foo() // function call uses function name to call it
3
4 call (foo) // top-level function passed
5 call ((): void=> { console.log ("Lambda is called") })// lambda is passed
6 call (A.method) // static method
7 call ((newA).method)// instance method is passed
8
9 class A {
10 static method() { console.log ("Static method() is called") }
11 method() { console.log ("Instance method() is called") }
12 }
13
14 functioncall (callee: () => void) {
15 callee()// function call uses parameter name to call any functional object passed␣
˓→as an argument
16 }
17
18 (():void => { console.log ("Lambda is called") }) ()// function call uses lambda␣
˓→expression to call it
19
20 letx = foo()// compile-time error as void cannot be used as type annotation

```
Type of a function call expression is the return type of the function.
```
### 7.12 Indexing Expressions

```
Indexing expressions are used to access elements of arrays (see Array Types ), strings (see Type string ), andRecord
instances (see Record Utility Type ). Indexing expressions can also be applied to instances of indexable types (see
Indexable Types ).
The syntax of indexing expression is presented below:
```
```
indexingExpression:
expression('?.')?'['expression ']'
;
```
```
7.12. Indexing Expressions 109
```

```
Any indexing expression has two subexpressions as follows:
```
- _Object reference expression_ before the left bracket; and
- _Index expression_ inside the brackets.
If the operator ‘?.’ (see _Chaining Operator_ ) is present in an indexing expression, then:
- If an object reference expression is not of a nullish type, then the chaining operator has no effect.
- Otherwise, object reference expression must be checked to nullish value. If the value isundefinedornull, then
the evaluation of the entire surrounding _primary expression_ stops. The result of the entire primary expression is
thenundefined.
If no ‘?.’ is present in an indexing expression, then object reference expression must be of array type orRecordtype.
Otherwise, a compile-time error occurs.

#### 7.12.1 Array Indexing Expression

```
Index expression for array indexing must be one of integer types, namelybyte,short, orint. Otherwise, a compile-
time error occurs.
The conversion ofbyteandshorttypes (see Widening Numeric Conversions ) is performed on an index expression
to ensure that the resultant type isint. Otherwise, a compile-time error occurs.
Other numeric types (long,float, anddouble/number) must be converted explicitly by applying the methods defined
in the classes of the Standard Library.
```
1 const a = ["Alice", "Bob", "Carol"]
2 functiondemo (l:long, f:float, d:double, n:number) {
3 console.log (
4 a[l.toInt()], a[f.toInt()],
5 a[d.toInt()], a[n.toInt()]
6 ) // OK to access array using index expression conversion methods
7 }

```
If the chaining operator ‘?.’ (see Chaining Operator ) is present, and after its application the type of object reference ex-
pression is an array type , then it makes a valid array reference expression , and the type of the array indexing expression
isT.
The result of an array indexing expression is a variable of typeT(i.e., an element of the array selected by the value of
that index expression ).
It is essential that, if typeTis a reference type, then the fields of array elements can be modified by changing the
resultant variable fields:
```
1 letnames: string[] = ["Alice", "Bob", "Carol"]
2 console.log(names[1])// prints Bob
3 names[1] = "Martin"
4 console.log(names[1])// prints Martin
5
6 console.log (names["1"])// compile-time error as index of non-numeric type
7
8 class RefType {
9 field: number= 42
(continues on next page)

```
110 Chapter 7. Expressions
```

```
(continued from previous page)
```
10 }
11 const objects:RefType[] = [newRefType(),newRefType()]
12 const obj = objects [1]
13 obj.field = 777 // change the field in the array element
14 console.log(objects[0].field)// prints 42
15 console.log(objects[1].field)// prints 777
16
17 letan_array = [1, 2, 3]
18 letelement = an_array [3.5]// compile-time error as index is not integer
19 functionfoo (index:number) {
20 letelement = an_array [index]// compile-time error as index is not integer
21 }

```
An array indexing expression evaluated at runtime behaves as follows:
```
- Object reference expression is evaluated first.
- If the evaluation completes abruptly, then so does the indexing expression, and the index expression is not eval-
    uated.
- If the evaluation completes normally, then the index expression is evaluated. The resultant value of the object
    reference expression refers to an array.
- If the index expression value of an array is less than zero, greater than or equal to that array’s _length_ , then
    RangeErroris thrown.
- Otherwise, the result of the array access is a typeTvariable within the array selected by the value of the index
    expression.

```
1 functionsetElement(names:string[], i:int, name:string) {
2 names[i] = name // runtime error, if'i'is out of bounds
3 }
```
#### 7.12.2 String Indexing Expression

```
Index expression for string indexing must be of one of integer types, namelybyte,short, orint. The same rules
apply as in Array Indexing Expression.
If the index expression value of a string is less than zero, greater than or equal to that string’s length , thenRangeError
is thrown.
```
```
1 console.log("abc"[1]]) // prints: b
2 console.log("abc"[3]]) // runtime exception
```
```
The result of a string indexing expression is a value ofstringtype.
Note. String value is immutable, and is not allowed to change a value of a string element by indexing.
```
```
1 letx = "abc"
2 x[1] = "d" // compile-time error, string value is immutable
```
```
7.12. Indexing Expressions 111
```

### 7.12.3 Record Indexing Expression

```
Indexing expression for a typeRecord<Key, Value>(see Record Utility Type ) allows getting or setting a value of
typeValueat an index specified by the expression of typeKey.
The following two cases are to be considered separately:
```
1. TypeKeyis a union that contains literal types only;
2. Other cases.
**Case 1.** If typeKeyis a union that contains literal types only, then an _index expression_ can only be one of the literals
listed in the type. The result of the indexing expression is of typeValue.

```
1 typeKeys = 'key1' |'key2' |'key3'
2
3 letx: Record<Keys, number> = {
4 'key1': 1,
5 'key2': 2,
6 'key3': 4,
7 }
8 lety = x['key2']// y value is 2
```
```
A compile-time error occurs if an index expression is not a valid literal:
```
```
1 console.log(x['key4']) // compile-time error
2 x['another key'] = 5// compile-time error
```
```
The compiler guarantees that an object ofRecord<Key, Value>for this typeKeycontains values for allKeykeys.
Case 2. An index expression has no restriction. The result of an indexing expression is of typeValue | undefined.
```
1 letx: Record<number,string> = {
2 1: "hello",
3 2: "buy",
4 }
5
6 functionfoo(n: number): string |undefined{
7 return x[n]
8 }
9
10 functionbar(n: number): string {
11 lets = x[n]
12 if (s == undefined) {return"no" }
13 return s!
14 }
15
16 foo(3) // prints "undefined"
17 bar(3) // prints "no"
18
19 lety = x[3]

```
Type of y in the code above isstring | undefined. The value of y isundefined.
An indexing expression evaluated at runtime behaves as follows:
```
```
112 Chapter 7. Expressions
```

- Object reference expression is evaluated first.
- If the evaluation completes abruptly, then so does the indexing expression, and the index expression is not eval-
    uated.
- If the evaluation completes normally, then the index expression is evaluated. The resultant value of the object
    reference expression refers to arecordinstance.
- If therecordinstance contains a key defined by the index expression, then the result is the value mapped to the
    key.
- Otherwise, the result is the literalundefined.

## 7.13 Chaining Operator

```
The chaining operator ‘?.’ is used to effectively access values of nullish types. It can be used in the following contexts:
```
- _Field Access Expression_ ,
- _Method Call Expression_ ,
- _Function Call Expression_ ,
- _Indexing Expressions_.
If the value of the expression to the left of ‘?.’ isundefinedornull, then the evaluation of the entire surrounding
_primary expression_ stops. The result of the entire primary expression is thenundefined. Thus the type of the entire
primary expression is the unionundefined| _non-nullish type of the entire primary expression_ :

1 class Person {
2 name: string
3 spouse?:Person =undefined
4 constructor(name:string) {
5 this.name = name
6 }
7 }
8
9 letbob = newPerson("Bob")
10 console.log(bob.spouse?.name)// prints "undefined"
11 // type of bob.spouse?.name is undefined|string
12
13 bob.spouse =newPerson("Alice")
14 console.log(bob.spouse?.name)// prints "Alice"
15 // type of bob.spouse?.name is undefined|string

```
If an expression is not of a nullish type, then the chaining operator has no effect.
A compile-time error occurs if a chaining operator is placed in the context where a variable is expected, e.g., in the
left-hand-side expression of an assignment (see Assignment ) or expression (see Postfix Increment , Postfix Decrement ,
Prefix Increment or Prefix Decrement ).
```
```
7.13. Chaining Operator 113
```

## 7.14 NewExpressions

```
There are two syntactical forms of the new expression :
```
```
newExpression:
newClassInstance
|newArrayInstance
;
```
```
Type of a new expression is etherclassorarray.
A new class instance expression creates a new object that is an instance of the specified class and it is described in full
details below.
The creation of array instances is an experimental feature discussed in Resizable Array Creation Expressions.
The syntax of new class instance expression is presented below:
```
```
newClassInstance:
'new'typeArguments? typeReference arguments?
;
```
```
Class instance creation expression specifies a class to be instantiated. It optionally lists all actual arguments for the
constructor.
```
1 class A {
2 constructor(p:number) {}
3 }
4
5 newA(5)// create an instance and call constructor
6 const a =newA(6) /* create an instance, call constructor and store
7 created and initialized instance in 'a'*/

```
Class instance creation expression can throw an error (see Error Handling , Constructor Declaration ).
When a class instance creation expression refers to classes FixedArray , Array , or derived classes of Array instantiated
with an array element type of some class type then it turns out to be a special form of array creation expression. And in
case when such array creation expression defines a number of elements of the created array it leads to a compile-time
error if the type of an array element:
```
- refers to a class that contains neither an accessible (see _Accessible_ ) parameterless constructor nor a constructor
    with all parameters of the second form of optional parameters (see _Optional Parameters_ ); or
- has no default value.
The same restriction applies to ref: _Resizable Array Creation Expressions_.

1 class A<T> {
2 foo () {
3 consta1 =newArray<T> (5)// Array with 5 elements of type T cannot be created
4 consta1 =newFixedArray<T> (5)// Array with 5 elements of type T cannot be␣
˓→created
5 }
6 }

```
The execution of a class instance creation expression is performed as follows:
```
- New instance of class is created;

```
114 Chapter 7. Expressions
```

- Constructor of class is called to fully initialize the created instance.
The validity of the constructor call is similar to the validity of the method call as discussed in _Step 2: Selection of
Method_ , except the cases discussed in _Constructor Body_.
A compile-time error occurs iftypeReferenceis a type parameter.
**Note**. If a _class instance creation expression_ with no argument is used as object reference in a method call expression,
then empty parentheses ‘()’ are to be used.

1 class A { method() {} }
2
3 newA.method() // compile-time error
4 newA().method()// OK
5 (newA).method()// OK
6 leta =newA // OK

## 7.15 InstanceOfExpression

```
The syntax of instanceof expression is presented below:
```
```
instanceOfExpression:
expression'instanceof' type
;
```
```
Anyinstanceofexpression in the formexpr instanceof Tis of typeboolean.
The result of aninstanceofexpression istrueif the actual type of evaluatedexpris a subtype ofT(see Subtyping ).
Otherwise, the result isfalse.
A compile-time error occurs if typeTis not retained by Type Erasure.
Generic type (see Generics ) in the form of type name (see Type References ) can be used asToperand of aninstanceof
expression. In this case, the check is performed against the type name , and type parameters are ignored. Instantiated
generic types (see Explicit Generic Instantiations ) cannot be used because theToperand of aninstanceofmust be
retained by Type Erasure.
```
1 class C<T> {
2 foo() {
3 console.log(this instanceof C) // true
4 console.log(this instanceof C<T>)// compile-time error
5 }
6 }
7
8 letc =newC<number>
9 c.foo()

```
Thetypeof aninstanceofexpression is used for smart cast (see Smart Types ) if applicable.
```
```
7.15. InstanceOf Expression 115
```

## 7.16 CastExpression

```
The syntax of cast expression is as follows:
```
```
castExpression:
expression'as' type
;
```
```
Cast expression in the formexpr as targetapplies the cast operator astoexprby issuing the value of a specified
targettype. Thus, the type of a cast expression is always thetargettype.
```
1 class X {}
2
3 letx1 :X =newX()
4 letob :Object = x1as Object// Object is the target type
5 letx2 :X = obas X// X is the target type

```
A compile-time error occurs if thetargettype is typenever:
```
1 1 asnever // compile-time error

```
A compile-time error occurs iftargettype is not preserved by Type Erasure.
Two specific cases of a cast expression are described in the sections below:
```
- _Type Inference in Cast Expression_ ifexpris a numeric literal (see _Numeric Literals_ ), an _Array Literal_ , or an
    _Object Literal_ ;
- _Runtime Checking in Cast Expression_ otherwise.
If none of conditions stated in these sections are satisfied, then a compile-time error occurs.

### 7.16.1 Type Inference in Cast Expression

```
The following combinations ofexprandtargetare considered for theexpr as targetexpression:
```
- expris a numeric literal, see _Type Inference for Numeric Literals_ for detail;
- expris an _Array Literal_ , andtargetis an _array type_ or a _tuple type_ (see _Array Literal Type Inference from_
    _Context_ for detail);
- expris an _Object Literal_ , andtargetis _class type_ , _interface type_ , or _Record Utility Type_ (see the subsections
    of _Object Literal_ for detail).
This kind of a _cast expression_ results in inferring the target type forexpr. A compile-time error can occur when
processing a _cast expression_ (see corresponding sections for detail), but this expression never causes a runtime error
by itself. However, the evaluation of array literal elements or object literal properties can cause a runtime error.
Casting for numeric literals is represented in the example below:

1 letx = 1as byte// ok
2 lety = 128as byte// compile-time error

```
Casting for array literals is represented in the example below:
```
```
116 Chapter 7. Expressions
```

```
1 leta = [1, 2]as double[]// ok, [1.0, 2.0]
2 letb = [1, 2]as double // compile-time error, wrong target type
3 letc = [1, "cc"]as double[] // compile-time error, wrong element type
4 letd = [1, "cc"]as [double, string] // ok, cast to the tuple type
5 lete = [1.0, "cc"] as[int, string] // compile-time error, wrong element type
```
```
Note. Assignability check is applied to the elements of an array literal.
Examples with object literals are provided in Object Literal.
```
### 7.16.2 Runtime Checking in Cast Expression

```
If none of the previous kinds of cast expression can be applied, thenexpr as targetchecks if the type ofexpris a
subtype oftarget(see Subtyping ).
If the actual type ofexpris a subtype oftarget(see Subtyping ), then the result of anasexpression is the result of
the evaluatedexpr. Otherwise,ClassCastErroris thrown.
Iftargettype is not preserved by Type Erasure , then the check is performed against an effective type of thetarget
type. As the effective type is less specific thantargetin the case described, the usage of the resulting value can cause
type violation, andClassCastErroris thrown as a consequence (see Type Erasure for detail).
Semantically, a cast expression of this kind is coupled tightly with InstanceOf Expression as follows:
```
- If the result ofx instanceof Tistrue, thenx as Tnever causes a runtime error;
- Ifx instanceof Tcauses a compile-time error as a result of _Type Erasure_ , thenx as Talso causes a compile-
    time error.
- If otherwise the result ofx instanceof Tisfalse, thenx as TcausesClassCastErrorthrown at runtime.
This situation is represented in the following example:

```
1 functionfoo (x:Object) {
2 x as string
3 }
4
5 foo("aa") // OK
6 foo(1) // runtime error is thrown in foo by'as'operator application
```
```
InstanceOf Expression can be used to prevent runtime errors. Moreover, the InstanceOf Expression makes cast con-
version unnecessary in many cases as smart cast is applied (see Smart Types ):
```
1 class Person {
2 name: string
3 constructor(name: string) {this.name = name }
4 }
5
6 functionprintName(x:Object) {
7 if (x instanceof Person) {
8 // no need to cast, type of 'x'is'Person'here
9 console.log(x.name)
10 } else{
(continues on next page)

```
7.16. Cast Expression 117
```

```
(continued from previous page)
```
11 console.log("not a Person")
12 }
13 }
14
15 printName(newPerson("Bob")) // output: Bob
16 printName(1) // output: not a Person

## 7.17 TypeOfExpression

```
The syntax of typeof expression is presented below:
```
```
typeOfExpression:
'typeof'expression
;
```
```
Anytypeofexpression is of typestring.
If typeof expression refers to a name of an overloaded function or method, then a compile-time error occurs.
The evaluation of a typeof expression starts with theexpressionevaluation. If this evaluation causes an error, then
thetypeofexpression evaluation terminates abruptly. Otherwise, the value of atypeof expressionis defined as
follows:
```
1. The value of aTypeOfexpression is known at compile time

```
118 Chapter 7. Expressions
```

```
Expression Type TypeOf Result Code Example
string “string”
lets: string= ...
typeof s
```
```
boolean “boolean”
letb: boolean= ...
typeof b
```
```
bigint “bigint”
letb: bigint= ...
typeof b
```
```
any class or interface “object”
leta: Object= ...
typeof a
```
```
any function type “function”
letf: () =>void= ...
typeof f
```
```
undefined “undefined”
typeof undefined
```
```
null “object”
typeof null
```
```
T|null, whenTis a class (but not
Object - see next table), interface or
array
```
```
“object”
classC {}
letx: C| null= ...
typeof x
```
```
enumeration type name of enumeration base type
enumC {R, G, B}
letc: C= ...
typeof c// "int"
```
```
number,double “number”
letn: number= ...
typeof n
```
```
Other numeric types:
byte,short,int,long,float
```
```
“byte”, “short”, “int”, “long” or
“float”, depending on type of expres-
sion
```
```
letx: byte= ...
typeof x// "byte"
```
```
char “char”
letx: char= ...
typeof x
```
2. The value of aTypeOfexpression is determined at runtime

The result is the name of an actual type used at runtime for the following expression types:

**7.17.** TypeOf **Expression 119**


```
Expression Type Code Example
Object
functionf(o:Object) {
typeof o
}
union type
functionf(p:A|B) {
typeof p
}
type parameter
classA<T|null|undefined> {
f:T
m() {
typeof this.f
}
constructor(p:T) {
this.f = p
}
}
```
## 7.18 Ensure-Not-Nullish Expression

_Ensure-not-nullish expression_ is a postfix expression with the operator ‘!’. An _ensure-not-nullish expression_ in the
expression _e!_ checks whether _e_ of a nullish type (see _Nullish Types_ ) evaluates to a nullish value.

The syntax of _ensure-not-nullish expression_ is presented below:

ensureNotNullishExpression:
expression'!'
;

If the expression _e_ is not of a nullish type, then the operator ‘!’ has no effect.

If the result of the evaluation of _e_ is not equal tonullorundefined, then the result of _e!_ is the outcome of the
evaluation of _e_.

If the result of the evaluation of _e_ is equal tonullorundefined, thenNullPointerErroris thrown.

Type of _ensure-not-nullish_ expression is the non-nullish variant of type of _e_.

**120 Chapter 7. Expressions**


## 7.19 Nullish-Coalescing Expression

```
Nullish-coalescing expression is a binary expression that uses the operator ‘??’.
The syntax of nullish-coalescing expression is presented below:
```
```
nullishCoalescingExpression:
expression'??' expression
;
```
```
A nullish-coalescing expression checks whether the evaluation of the left-hand-side expression equals the nullish value:
```
- If so, then the right-hand-side expression evaluation is the result of a nullish-coalescing expression.
- If not so, then the result of the left-hand-side expression evaluation is the result of a nullish-coalescing expression,
    and the right-hand-side expression is not evaluated (the operator ‘??’ is thus _lazy_ ).
The type of a nullish-coalescing expression is a normalized _union type_ (see _Union Types_ ) formed from the following:
- Non-nullish variant of the type of the left-hand-side expression; and
- Type of the right-hand-side expression.
The semantics of a nullish-coalescing expression is represented in the following example:

1 letx = lhs_expression ?? rhs_expression
2
3 letx$ = lhs_expression
4 if (x$ == null) {x = rhs_expression}elsex = x$!
5
6 // Type of x is NonNullishType(lhs_expression)|Type(rhs_expression)

```
A compile-time error occurs if the nullish-coalescing operator is mixed with conditional-and or conditional-or operators
without parentheses.
```
## 7.20 Unary Expressions

```
The syntax of unary expression is presented below:
```
```
unaryExpression:
expression'++'
|expression '--'
|'++' expression
|'--' expression
|'+'expression
|'-'expression
|'~'expression
|'!'expression
;
```
```
All expressions with unary operators (except postfix increment and postfix decrement operators) group right-to-left for
‘~+x’ to have the same meaning as ‘~(+x)’.
```
```
7.19. Nullish-Coalescing Expression 121
```

The type of _unaryExpression_ is not necessarily the same as the type of the _expression_ provided. Further in the text, the
type of _unaryExpression_ is stated explicitly for each _unary operator_.

### 7.20.1 Postfix Increment

_Postfix increment expression_ is an _expression_ followed by the increment operator ‘++’.

The _expression_ must be _left-hand-side expression_ (see _Left-Hand-Side Expressions_ ), so it denotes a variable.

A compile-time error occurs if type of the the _expression_ is not convertible (see _Implicit Conversions_ ) to a numeric
type (see _Numeric Types_ ).

Type of a _postfix increment expression_ is the type of the variable. The result of a _postfix increment expression_ is a value,
not a variable.

If the evaluation of the operand _expression_ completes normally at runtime, then:

- The value _1_ is added to the value of the variable by using necessary conversions (see _Numeric Casting Conver-_
    _sions_ ); and
- The sum is stored back into the variable.

Otherwise, the _postfix increment expression_ completes abruptly, and no incrementation occurs.

The value of the _postfix increment expression_ is the value of the variable _before_ a new value is stored.

### 7.20.2 Postfix Decrement

_Postfix decrement expression_ is an expression followed by the decrement operator ‘--’. The expression must be _left-
hand-side expression_ (see _Left-Hand-Side Expressions_ ).

A compile-time error occurs if type of the expression is not convertible (see _Implicit Conversions_ ) to a numeric type
(see _Numeric Types_ ).

Type of a postfix decrement expression is the type of the variable. The result of a postfix decrement expression is a
value, not a variable.

If evaluation of the operand expression completes at runtime, then:

- The value _1_ is subtracted from the value of the variable by using necessary conversions (see _Numeric Casting_
    _Conversions_ ); and
- The sum is stored back into the variable.

Otherwise, the _postfix decrement expression_ completes abruptly, and no decrementation occurs.

The value of the _postfix decrement expression_ is the value of the variable _before_ a new value is stored.

**122 Chapter 7. Expressions**


### 7.20.3 Prefix Increment

_Prefix increment expression_ is an expression preceded by the operator ‘++’. The expression must be _left-hand-side
expression_ (see _Left-Hand-Side Expressions_ ).

A compile-time error occurs if the type of the expression is not convertible (see _Implicit Conversions_ ) to a numeric
type (see _Numeric Types_ ).

Type of a prefix increment expression is the type of the variable. The result of a prefix increment expression is a value,
not a variable.

If evaluation of the operand _expression_ completes normally at runtime, then:

- The value _1_ is added to the value of the variable by using necessary conversions (see _Numeric Casting Conver-_
    _sions_ ); and
- The sum is stored back into the variable.

Otherwise, the _prefix increment expression_ completes abruptly, and no incrementation occurs.

The value of the _prefix increment expression_ is the value of the variable _after_ a new value is stored.

### 7.20.4 Prefix Decrement

_Prefix decrement expression_ is an expression preceded by the operator ‘--’. The expression must be _left-hand-side
expression_ (see _Left-Hand-Side Expressions_ ).

A compile-time error occurs if type of the expression is not convertible (see _Implicit Conversions_ ) to a numeric type
(see _Numeric Types_ ).

Type of a prefix decrement expression is the type of the variable. The result of a prefix decrement expression is a value,
not a variable.

If evaluation of the operand _expression_ completes normally at runtime, then:

- The value _1_ is subtracted from the value of the variable by using necessary conversions (see _Numeric Casting_
    _Conversions_ ); and
- The sum is stored back into the variable.

Otherwise, the _prefix decrement expression_ completes abruptly, and no decrementation occurs. The value of the _prefix
decrement expression_ remains the value of the variable _after_ a new value is stored.

### 7.20.5 Unary Plus

_Unary plus expression_ is an expression preceded by the operator ‘+’. Type of the operand expression with the unary
operator ‘+’ must be convertible (see _Implicit Conversions_ ) to a numeric type (see _Numeric Types_ ). Otherwise, a
compile-time error occurs.

A numeric types conversion is performed on the operand to ensure that the resultant type is that of the unary plus
expression. The result of a unary plus expression is always a value, not a variable (even if the result of the operand
expression is a variable).

**7.20. Unary Expressions 123**


Numeric widening occurs on the _expression_ before a _unary plus_ operator is applied. The type of the _unary plus_ is
determined as follows:

- Type of result isintforbyte,short, andint;
- Type of result is the same as that of the initial _expression_ forlong,float, anddouble.

### 7.20.6 Unary Minus

_Unary minus expression_ is an expression preceded by the operator ‘-‘. Type of the operand expression with the unary
operator ‘-’ must be convertible (see _Widening Numeric Conversions_ ) to a numeric type (see _Numeric Types_ ). Other-
wise, a compile-time error occurs.

Numeric widening occurs on the _expression_ before a _unary minus_ operator is applied. The type of the _unary minus_ is
determined as follows:

- Type of result is _int_ forbyte,short, andint;
- Type of result is the same as that of the initial _expression_ forlong,float, anddouble.

The result of a unary minus expression is a value, not a variable (even if the result of the operand expression is a
variable).

The unary negation operation is always performed on, and the result is drawn from the same value set as the promoted
operand value.

Further value set conversions are then performed on the same result.

The value of a unary minus expression at runtime is the arithmetic negation of the promoted value of the operand.

The negation of integer values is the same as subtraction from zero. The ArkTS programming language uses two’s-
complement representation for integers. The range of two’s-complement value is not symmetric. The same maximum
negative number results from the negation of the maximum negative _int_ or _long_. In that case, an overflow occurs but
throws no error. For any integer value _x_ , _-x_ is equal to _(~x)+1_.

The negation of floating-point values is _not_ the same as subtraction from zero (if _x_ is _+0.0_ , then _0.0-x_ is _+0.0_ , however
_-x_ is _-0.0_ ).

A unary minus merely inverts the sign of a floating-point number. Special cases to consider are as follows:

- OperandNaNresults inNaN(NaNhas no sign).
- Operand infinity results in the infinity of the opposite sign.
- Operand zero results in zero of the opposite sign.

### 7.20.7 Bitwise Complement

_Bitwise complement_ operator ‘~’ is applied to an operand of a numeric type or typebigint.

If the type of the operand isdoubleorfloat, then it is truncated first tolongorint, respectively. If the type of the
operand isbyteorshort, then the operand is widened toint. If the type of the operand isbigint, then no conversion
is required. Type of result is determined as follows:

- intforbyte,short,int, andfloat.

**124 Chapter 7. Expressions**


- longforlonganddouble.

The result of a unary bitwise complement expression is a value, not a variable (even if the result of the operand expres-
sion is a variable).

The value of a unary bitwise complement expression at runtime is the bitwise complement of the value of the operand.
In all cases, _~x_ equals _(-x)-1_.

### 7.20.8 Logical Complement

_Logical complement expression_ is an expression preceded by the operator ‘!’. Type of the operand expression with the
unary ‘!’ operator must bebooleanor type mentioned in _Extended Conditional Expressions_. Otherwise, a compile-
time error occurs.

The unary logical complement expression type isboolean.

The value of a unary logical complement expression istrueif the (possibly converted) operand value isfalse, and
falseif the operand value (possibly converted) istrue.

## 7.21 Multiplicative Expressions

Multiplicative expressions use _multiplicative operators_ ‘*’, ‘/’, and ‘%’.

The syntax of _multiplicative expression_ is presented below:

multiplicativeExpression:
expression'*'expression
|expression '/' expression
|expression '%' expression
|expression '**'expression
;

Multiplicative operators group left-to-right.

Type of each operand in a multiplicative operator must be convertible (see _Numeric Operator Contexts_ ) to a numeric
type (see _Numeric Types_ ). Otherwise, a compile-time error occurs.

A numeric types conversion (see _Widening Numeric Conversions_ ) is performed on both operands to ensure that the
resultant type is the type of the multiplicative expression.

The resultant type of an expression is inferred by the largest type after promotingbyteandshortoperands toint:

- doubleif any operand isdouble;
- floatif any operand isfloat, and no operand isdouble;
- longif any operand islong, and no operand isdoubleorfloat;
- intif all operands are of typebyte,short, orint.

This situation is represented in the following example:

**7.21. Multiplicative Expressions 125**


1 // Code below prints true 4 times
2 letbyte1: byte= 1
3 letbyte2: byte= 1
4 letlong1: long= 1
5 letfloat1:float= 1
6 letdouble1:double = 1
7
8 letres_byte = byte1 * byte2 // int
9 console.log(res_byteinstanceof int)
10
11 letres_long = byte1 * long1 // long
12 console.log(res_longinstanceof long)
13
14 letres_float = byte1 * float1// float
15 console.log(res_floatinstanceof float)
16
17 letres_double = byte1 * double1 // double
18 console.log(res_doubleinstanceof double)

```
The result of a unary bitwise complement expression is a value, not a variable (even if the operand expression is a
variable).
```
### 7.21.1 Multiplication

```
The binary operator ‘*’ performs multiplication, and returns the product of its operands.
Multiplication is a commutative operation if operand expressions have no side effects.
Integer multiplication is associative when all operands are of the same type.
Floating-point multiplication is not associative.
Type of a multiplication expression is the ‘largest’ (see Numeric Types ) type of its operands.
If overflow occurs during integer multiplication, then:
```
- The result is the low-order bits of the mathematical product as represented in some sufficiently large two’s-
    complement format.
- The sign of the result can be other than the sign of the mathematical product of the two operand values.
A floating-point multiplication result is determined in compliance with the IEEE 754 arithmetic:
- The result isNaNif:
**-** Either operand isNaN;
**-** Infinity is multiplied by zero.
- If the result is notNaN, then the sign of the result is as follows:
**-** Positive, where both operands have the same sign; and
**-** Negative, where the operands have different signs.
- If infinity is multiplied by a finite value, then the multiplication results in a signed infinity (the sign is determined
by the rule above).

```
126 Chapter 7. Expressions
```

- If neitherNaNnor infinity is involved, then the exact mathematical product is computed.
    The product is rounded to the nearest value in the chosen value set by using the IEEE 754 _round-to-nearest_ mode.
    The ArkTS programming language requires gradual underflow support as defined by IEEE 754 (see _Floating-_
    _Point Types and Operations_ ).
    If the magnitude of the product is too large to represent, then the operation overflows, and the result is an appro-
    priately signed infinity.

The evaluation of a multiplication operator ‘*’ never throws an error despite possible overflow, underflow, or loss of
information.

### 7.21.2 Division

The binary operator ‘/’ performs division and returns the quotient of its left-hand-side and right-hand-side expressions
(dividendanddivisorrespectively).

Integer division rounds toward _0_ , i.e., the quotient of integer operands _n_ and _d_ , after a numeric types conversion on
both (see _Widening Numeric Conversions_ for details), is the integer value _q_ with the largest possible magnitude that
satisfies|𝑑·𝑞|≤|𝑛|.

**Note**. The integer value _q_ is:

- Positive, where |n|≥|d|, and _n_ and _d_ have the same sign; but
- Negative, where |n|≥|d|, and _n_ and _d_ have opposite signs.

The only one special case that does not comply with this rule is where integer overflow occurs. The result equals the
dividend if the dividend is a negative integer of the largest possible magnitude for its type, while the divisor is _-1_. No
error is thrown in this case despite the overflow.

However, if the divisor value of integer division is detected to be _0_ during compilation, then a compile-time error
occurs. Otherwise, anArithmeticErroris thrown during execution.

The result of a floating-point division is determined in compliance with the IEEE 754 arithmetic:

- The result isNaNif:
    **-** Either operand is NaN;
    **-** Both operands are infinity; or
    **-** Both operands are zero.
- If the result is notNaN, then the sign of the result is:
    **-** Positive, where both operands have the same sign; or
    **-** Negative, where the operands have different signs.
- Division produces a signed infinity (the sign is determined by the rule above) if:
    **-** Infinity is divided by a finite value; and
    **-** A nonzero finite value is divided by zero.
- Division produces a signed zero (the sign is determined by the rule above) if:
    **-** A finite value is divided by infinity; and
    **-** Zero is divided by any other finite value.

**7.21. Multiplicative Expressions 127**


- If neitherNaNnor infinity is involved, then the exact mathematical quotient is computed.
    If the magnitude of the product is too large to represent, then the operation overflows, and the result is an appro-
    priately signed infinity.

The quotient is rounded to the nearest value in the chosen value set by using the IEEE 754 _round-to-nearest_ mode. The
ArkTS programming language requires gradual underflow support as defined by IEEE 754 (see _Floating-Point Types
and Operations_ ).

The evaluation of a floating-point division operator ‘/’ never throws an error despite possible overflow, underflow,
division by zero, or loss of information.

The type of the _division expression_ is the ‘ _largest_ ’ numeric type (see _Numeric Types_ ) of its operands.

### 7.21.3 Remainder

The binary operator ‘%’ yields the remainder of its operands (dividendas the left-hand-side, anddivisoras the
right-hand-side operand) from an implied division.

The remainder operator in ArkTS accepts floating-point operands (unlike in C and C++).

The remainder operation on integer operands produces a result value, i.e.,(𝑎/𝑏)*𝑏+ (𝑎%𝑏)equals _a_. Numeric type
conversion on remainder operation is discussed in _Widening Numeric Conversions_.

This equality holds even in the special case where the dividend is a negative integer of the largest possible magnitude
of its type, and the divisor is _-1_ (the remainder is then _0_ ). According to this rule, the result of the remainder operation
can only be one of the following:

- Negative if the dividend is negative; or
- Positive if the dividend is positive.

The magnitude of the result is always less than that of the divisor.

If the divisor value of integer remainder operator is detected to be _0_ during compilation, then a compile-time error
occurs. Otherwise, anArithmeticErroris thrown during execution.

The result of a floating-point remainder operation as computed by the operator ‘%’ is different than that produced by the
remainder operation defined by IEEE 754. The IEEE 754 remainder operation computes the remainder from a rounding
division (not a truncating division), and its behavior is different from that of the usual integer remainder operator. On
the contrary, ArkTS presumes that the operator ‘%’ behaves on floating-point operations in the same manner as the
integer remainder operator (comparable to the C library function _fmod_ ). The standard library (see _Standard Library_ )
routineMath.IEEEremaindercan compute the IEEE 754 remainder operation.

The result of a floating-point remainder operation is determined in compliance with the IEEE 754 arithmetic:

- The result isNaNif:
    **-** Either operand isNaN;
    **-** The dividend is infinity;
    **-** The divisor is zero; or
    **-** The dividend is infinity, and the divisor is zero.
- If the result is notNaN, then the sign of the result is the same as the sign of the dividend.
- The result equals the dividend if:
    **-** The dividend is finite, and the divisor is infinity; or

**128 Chapter 7. Expressions**


**-** If the dividend is zero, and the divisor is finite.
- If infinity, zero, orNaNare not involved, then the floating-point remainder _r_ from the division of the dividend _n_
by the divisor _d_ is determined by the mathematical relation𝑟=𝑛−(𝑑·𝑞), where _q_ is an integer that is only:
**-** Negative if𝑛/𝑑is negative, or
**-** Positive if𝑛/𝑑is positive.
- The magnitude of _q_ is the largest possible without exceeding the magnitude of the true mathematical quotient of
_n_ and _d_.

The evaluation of the floating-point remainder operator ‘%’ never throws an error, even if the right-hand operand is
zero. Overflow, underflow, or loss of precision cannot occur.

The type of the _remainder expression_ is the ‘ _largest_ ’ numeric type (see _Numeric Types_ ) of its operands.

### 7.21.4 Exponentiation

The binary operator ‘**’ yields the result of raising the first operand (base) to the power of the second operand (expo-
nent). The operation returns NaN in the following cases:

- Exponent is NaN;
- Base is NaN, and exponent is not 0;
- Base is±1, and exponent is±Infinity; or
- Base is less than 0, and exponent is not an integer.

The binary operator ‘**’ is equivalent to Math.pow(), except it also acceptsbiginttypes as operands.

## 7.22 Additive Expressions

Additive expressions use _additive operators_ ‘+’ and ‘-‘.

The syntax of _additive expression_ is presented below:

additiveExpression:
expression'+'expression
|expression '-' expression
;

Additive operators group left-to-right.

If either operand of the operator is ‘+’ of typestring, then the operation is a string concatenation (see _String Con-
catenation_ ). In all other cases, type of each operand of the operator ‘+’ must be convertible (see _Widening Numeric
Conversions_ ) to a numeric type (see _Numeric Types_ ). Otherwise, a compile-time error occurs.

Type of each operand of the binary operator ‘-’ must be convertible (see _Widening Numeric Conversions_ ) to a numeric
type (see _Numeric Types_ ) in all cases. Otherwise, a compile-time error occurs.

Type of _Additive expression_ is determined as follows:

**7.22. Additive Expressions 129**


- stringif any operand is of typestring;
- Type inferred after widening operands of numeric types by the rules explained in the example in _Multiplicative_
    _Expressions_.

### 7.22.1 String Concatenation

If one operand of an expression is of typestring, then the string conversion (see _String Operator Contexts_ ) is per-
formed on the other operand at runtime to produce a string.

String concatenation produces a reference to astringobject that is a concatenation of two operand strings. The
left-hand-side operand characters precede the right-hand-side operand characters in a newly created string.

If the expression is not a constant expression (see _Constant Expressions_ ), then a newstringobject is created (see _New
Expressions_ ).

### 7.22.2 Additive Operators for Numeric Types

A numeric types conversion (see _Widening Numeric Conversions_ ) performed on a pair of operands ensures that both
operands are of a numeric type. If the conversion fails, then a compile-time error occurs.

The binary operator ‘+’ performs addition and produces the sum of such operands.

The binary operator ‘-’ performs subtraction and produces the difference of two numeric operands.

Type of an additive expression performed on numeric operands is the largest type (see _Numeric Types_ ) to which operands
of that expression are converted (see _Multiplicative Expressions_ for an example).

If the promoted type isintorlong, then integer arithmetic is performed. If the promoted type isfloatordouble,
then floating-point arithmetic is performed.

If operand expressions have no side effects, then addition is a commutative operation.

If all operands are of the same type, then integer addition is associative.

Floating-point addition is not associative.

If overflow occurs on an integer addition, then:

- Result is the low-order bits of the mathematical sum as represented in a sufficiently large two’s-complement
    format.
- Sign of the result is opposite to that of the mathematical sum of the operands’ values.

The result of a floating-point addition is determined in compliance with the IEEE 754 arithmetic as follows:

- The result isNaNif:
    **-** Either operand isNaN; or
    **-** The operands are two infinities of the opposite signs.
- The sum of two infinities of the same sign is the infinity of that sign.
- The sum of infinity and a finite value equals the infinite operand.

**130 Chapter 7. Expressions**


- The sum of two zeros of opposite sign is positive zero.
- The sum of two zeros of the same sign is zero of that sign.
- The sum of zero and a nonzero finite value is equal to the nonzero operand.
- The sum of two nonzero finite values of the same magnitude and opposite sign is positive zero.
- If infinity, zero, orNaNare not involved, and the operands have the same sign or different magnitudes, then the
    exact sum is computed mathematically.

If the magnitude of the sum is too large to represent, then the operation overflows. The result is an appropriately signed
infinity.

Otherwise, the sum is rounded to the nearest value within the chosen value set by using the IEEE 754 _round-to-nearest_
mode. The ArkTS programming language requires gradual underflow support as defined by IEEE 754 (see _Floating-
Point Types and Operations_ ).

When applied to two numeric type operands (see _Numeric Types_ ), the binary operator ‘-’ performs subtraction, and
returns the difference of such operands (minuendas left-hand-side, andsubtrahendas the right-hand-side operand).

The result of _a-b_ is always the same as that of _a+(-b)_ in both integer and floating-point subtraction.

The subtraction from zero for integer values is the same as negation. However, the subtraction from zero for floating-
point operands and negation is _not_ the same (if _x_ is _+0.0_ , then _0.0-x_ is _+0.0_ ; however _-x_ is _-0.0_ ).

The evaluation of a numeric additive operator never throws an error despite possible overflow, underflow, or loss of
information.

## 7.23 Shift Expressions

_Shift expressions_ use _shift operators_ ‘<<’ (left shift), ‘>>’ (signed right shift), and ‘>>>’ (unsigned right shift). The
value to be shifted is the left-hand-side operand in a shift operator, and the right-hand-side operand specifies the shift
distance.

The syntax of _shift expression_ is presented below:

shiftExpression:
expression'<<' expression
|expression '>>'expression
|expression '>>>'expression
;

Shift operators group left-to-right.

Both operands of a _shift expression_ must be of numeric types or typebigint.

If the type of one or both operands isdoubleorfloat, then the operand or operands are truncated first tolongor
int, respectively. If the type of the left-hand-side operand isbyteorshort, then the operand is converted toint.
If both operands are of typebigint, then no conversion is required. A compile-time error occurs if one operand is
typebigint, and the other one is a numeric type. Also, a compile-time error occurs if ‘>>>’ (unsigned right shift) is
applied to operands of typebigint.

The result of a _shift expression_ is of the type to which its first operand converted.

**7.23. Shift Expressions 131**


If the left-hand-side operand is of the promoted typeint, then only five lowest-order bits of the right-hand-side operand
specify the shift distance (as if using a bitwise logical AND operator ‘&’ with the mask value _0x1f_ or _0b11111_ on the
right-hand-side operand). Thus, it is always within the inclusive range of _0_ through _31_.

If the left-hand-side operand is of the promoted typelong, then only six lowest-order bits of the right-hand-side operand
specify the shift distance (as if using a bitwise logical AND operator ‘&’ with the mask value _0x3f_ or _0b111111_ the
right-hand-side operand). Thus, it is always within the inclusive range of _0_ through _63_.

Shift operations are performed on the two’s-complement integer representation of the value of the left-hand-side
operand at runtime.

The value of _n_ << _s_ is _n_ left-shifted by _s_ bit positions. It is equivalent to multiplication by two to the power _s_ even in
case of an overflow.

The value of _n_ >> _s_ is _n_ right-shifted by _s_ bit positions with sign-extension. The resultant value is𝑓𝑙𝑜𝑜𝑟(𝑛/ 2 𝑠). If _n_ is
non-negative, then it is equivalent to truncating integer division (as computed by the integer division operator by 2 to
the power _s_ ).

The value of _n_ >>> _s_ is _n_ right-shifted by _s_ bit positions with zero-extension, where:

- If _n_ is positive, then the result is the same as that of _n_ >> _s_.
- If _n_ is negative, and type of the left-hand-side operand isint, then the result is equal to that of the expression ( _n_
    >> _s_ )+ (2 << ~ _s_ ).
- If _n_ is negative, and type of the left-hand-side operand islong, then the result is equal to that of the expression
    ( _n_ >> _s_ )+ ((2 as long) << ~ _s_ ).

## 7.24 Relational Expressions

Relational expressions use _relational operators_ ‘<’, ‘>’, ‘<=’, and ‘>=’.

The syntax of _relational expression_ is presented below:

relationalExpression:
expression'<'expression
|expression '>' expression
|expression '<='expression
|expression '>='expression
;

Relational operators group left-to-right.

A relational expression is always of typeboolean.

The four kinds of relational expressions are described below. The kind of a relational expression depends on types of
operands. It is a compile-time error if at least one type of operands is different from types described below.

**132 Chapter 7. Expressions**


### 7.24.1 Numeric Relational Operators

Type of each operand in anumeric relational operatormust be convertible to a numeric type (see _Numeric
Types_ ). Otherwise, a compile-time error occurs.

Depending on the converted type of operands, a comparison is performed as follows:

- Signed integer comparison, if the converted operand type isintorlong.
- Floating-point comparison, if the converted operand type isfloatordouble.

The comparison of floating-point values drawn from any value set must be accurate.

A floating-point comparison must be performed in accordance with the IEEE 754 standard specification as follows:

- The result of a floating-point comparison is false if either operand isNaN.
- All values other thanNaNmust be ordered with the following:
    **-** Negative infinity less than all finite values; and
    **-** Positive infinity greater than all finite values.
- Positive zero equals negative zero.

Based on the above presumption, the following rules apply to integer, floating-point, orbigintoperands other than
NaN:

- The value produced by the operator ‘<’ istrueif the value of the left-hand-side operand is less than that of the
    right-hand-side operand. Otherwise, the value isfalse.
- The value produced by the operator ‘<=’ istrueif the value of the left-hand-side operand is less than or equal
    to that of the right-hand-side operand. Otherwise, the value isfalse.
- The value produced by the operator ‘>’ istrueif the value of the left-hand-side operand is greater than that of
    the right-hand-side operand. Otherwise, the value isfalse.
- The value produced by the operator ‘>=’ istrueif the value of the left-hand-side operand is greater than or equal
    to that of the right-hand-side operand. Otherwise, the value isfalse.

### 7.24.2 String Relational Operators

Results of all string comparisons are defined as follows:

- Operator ‘<’ deliverstrueif the string value of the left-hand-side operand is lexicographically less than the
    string value of the right-hand-side operand, orfalseotherwise.
- Operator ‘<=’ deliverstrueif the string value of the left-hand-side operand is lexicographically less than or
    equal to the string value of the right-hand-side operand, orfalseotherwise.
- Operator ‘>’ deliverstrueif the string value of the left-hand-side operand is lexicographically greater than the
    string value of the right-hand-side operand, orfalseotherwise.
- Operator ‘>=’ deliverstrueif the string value of the left-hand-side operand is lexicographically greater than or
    equal to the string value of the right-hand operand, orfalseotherwise.

**7.24. Relational Expressions 133**


### 7.24.3 Boolean Relational Operators

Results of all boolean comparisons are defined as follows:

- Operator ‘<’ deliverstrueif the left-hand-side operand isfalse, and the right-hand-side operand is true, or
    falseotherwise.
- Operator ‘<=’ delivers:
    **-** truewhen both operands aretrue, or the left-hand-side operand isfalsefor any right-hand value;
    **-** falsewhen the left-hand-side operand istrue, and the right-hand-side operand isfalse.
- Operator ‘>’ deliverstrueif the left-hand-side operand istrue, and the right-hand-side operand isfalse, or
    falseotherwise.
- Operator ‘>=’ delivers:
    **-** truewhen both operands arefalse, or the left-hand-side operand istruefor any right-hand-side value;
    **-** falsewhen the left-hand-side operand isfalse, and the right-hand-side operand istrue.

### 7.24.4 Enumeration Relational Operators

If both operands are of the same enumeration type (see _Enumerations_ ), then _Numeric Relational Operators_ or _String
Relational Operators_ are used depending on the kind of enumeration constant value ( _Enumeration Integer Values_ or
_Enumeration String Values_ ). Otherwise, a compile-time error occurs.

## 7.25 Equality Expressions

Equality expressions use _equality operators_ ‘==’, ‘===’, ‘!=’, and ‘!==’.

The syntax of _equality expression_ is presented below:

equalityExpression:
expression('==' |'===' |'!='| '!==')expression
;

Equality operators group left-to-right. Equality operators are commutative if operand expressions cause no side effects.

Similarly to relational operators, equality operators returntrueorfalse. Equality operators have lower precedence
than relational operators, for example,𝑎 < 𝑏==𝑐 < 𝑑is _true_ when both𝑎 < 𝑏and𝑐 < 𝑑aretrue.

Any equality expression is of typeboolean.

The result produced bya != band!(a == b)is the same in all cases. The result produced bya !== band!(a
=== b)is the same.

The result of the operators ‘==’ and ‘===’ is the same in all cases except when comparing the valuesnulland
undefined(see _Extended Equality with null or undefined_ ).

A comparison that uses the operators ‘==’ and ‘===’ is evaluated totruewhen

- Operands of _Type boolean_ have the same value;

**134 Chapter 7. Expressions**


- Operands of _Type string_ or string literal type (see _String Literal Types_ ) have the same contents;
- Operands after a numeric conversion are of _Type bigint_ (see _Numeric Conversions for Relational and Equality_
    _Operands_ ) and have the same value;
- Operands after a numeric conversion (see _Widening Numeric Conversions_ , _Numeric Conversions for Relational_
    _and Equality Operands_ ) are of _Numeric Types_ of the same value exceptNaN(see _Numeric Equality Operators_
    for detail);
- Operands of _Type char_ have the same value (both operands represent the same Unicode code point);
- Operands of the same enumeration type (see _Enumerations_ ) have the same numeric values or the same string
    contents, depending on the type of enumeration constant values;
- Function references that refer to the same functional object (see _Function Type Equality Operators_ for detail).
In all other cases, if typesAandBdo not overlap (and therefore an expression always evaluated tofalseat compile
time), then:
- if each ofAandBis either a predefined type or a union of predefined types, a compile-time-error is issued..
- in all other cases, a compile-time warning is issued.
**Note**. There are two main reasons why compiler do not use always a compile-time error:
- Compatibility with TypeScript code base
- The inferred _smart type_ (see _Smart Types_ ) could lead in some cases to triggering the error even in the case when
it is impossible at runtime (see an example below):

```
1 classB {
2 f(): B|undefined {return undefined}
3 }
4 classD extendsB {
5 f(): D {return this}
6 }
7
8 functionf(c:B) {
9 if (cinstanceof D) {
10 // smart type causes compile-time warning
11 c.f() ==undefined
12 }
13 }
```
```
An evaluation of equality expressions always uses the actual types of operands as in the example below:
```
1 functionequ(a: Object, b:Object): boolean{
2 return a == b
3 }
4
5 equ(1, 1) // true, values are compared
6 equ(1, 2) // false, value are compared
7
8 equ("aa", "aa") // true, string contexts are compared
9 equ(1, "aa") // false, not compatible types
(continues on next page)

```
7.25. Equality Expressions 135
```

```
(continued from previous page)
```
10
11 interface I1 {}
12 interface I2 {}
13
14 functionequ1 (i1: I1, i2:I2) {
15 return i1 == i2// to be resolved during program execution
16 }
17 class Aimplements I1, I2 {}
18 const a =newA
19 equ1 (a, a)// true, the same values

```
An equality with values of two union types is represented in the example below:
```
```
1 functionf1(x: number| string, y:boolean| null):boolean{
2 return x == y// compile-time error, always evaluates to false
3 }
4
5 functionf2(x: number| string, y:boolean| "abc"): boolean{
6 // ok, can be evaluated as true
7 return x == y
8 }
```
### 7.25.1 Numeric Equality Operators

```
Type of each operand in anumeric equality operatormust be convertible to a numeric type (see Numeric Types )
as described in Numeric Conversions for Relational and Equality Operands. Otherwise, a compile-time error occurs.
A widening conversion can occur (see Widening Numeric Conversions ) if type of one operand is smaller than type of
the other operand (see Numeric Types ).
If the converted type of the operands isintorlong, then an integer equality test is performed.
If the converted type isfloatordouble, then a floating-point equality test is performed.
The floating-point equality test must be performed in accordance with the following IEEE 754 standard rules:
```
- The result of ‘==’ or ‘===’ isfalsebut the result of ‘!=’ istrueif either operand isNaN.
    The testx != xorx !== xistrueonly if _x_ isNaN.
- Positive zero equals negative zero.
- Equality operators consider two distinct floating-point values unequal in any other situation.
    For example, if one value represents positive infinity, and the other represents negative infinity, then each com-
    pares equal to itself and unequal to all other values.
Based on the above presumptions, the following rules apply to integer operands or floating-point operands other than
NaN:
- If the value of the left-hand-side operand is equal to that of the right-hand-side operand, then the operator ‘==’
or ‘===’ produces the valuetrue. Otherwise, the result isfalse.
- If the value of the left-hand-side operand is not equal to that of the right-hand-side operand, then the operator
‘!=’ or ‘!==’ produces the valuetrue. Otherwise, the result isfalse.

```
136 Chapter 7. Expressions
```

```
1 5 == 5 // true
2 5 != 5 // false
3
4 5 === 5// true
5
6 5 ==newNumber(5)// true
7 5 ===newNumber(5) // true
8
9 5 == 5.0// true
```
### 7.25.2 Function Type Equality Operators

```
If both operands refer to the same function object, then the comparison returnstrue. When comparing method refer-
ences, not only the same method must be used, but also its bounded instances must be equal.
```
1 functionfoo() {}
2 functionbar() {}
3 functiongoo(p: number) {}
4
5 foo == foo // true, same function object
6 foo == bar // false, different function objects
7 foo == goo // false, different function objects
8
9 class A {
10 method() {}
11 static method() {}
12 foo () {}
13 }
14 const a =newA
15 a.method == a.method// true, same function object
16 A.method == A.method// true, same function object
17
18 const aa =newA
19 a.method == aa.method/* false, different function objects
20 as 'a'and'aa'are different bounded objects */
21 a.method == a.foo// false, different function objects

### 7.25.3 Extended Equality withnullorundefined.

```
ArkTS provides extended semantics for equalities withnullandundefinedto ensure better alignment with Type-
Script.
If one operand in an equality expression isnull, and other isundefined, then the operator ‘!=’ returnstrue, and the
operator ‘!==’ returnsfalse:
```
```
7.25. Equality Expressions 137
```

1 functionfoo(x: Object |null, y:Object |null| undefined) {
2 console.log(x == y, x === y)
3 }
4
5 foo(null, undefined)// output: true, false
6 foo(null, null) // output: true, true

```
Comparison the valuesnullandundefineddirectly is also allowed:
```
1 console.log(null== undefined) // output: true
2 console.log(null===undefined) // output: false

## 7.26 Bitwise and Logical Expressions

```
The bitwise operators and logical operators are as follows:
```
- AND operator ‘&’;
- Exclusive OR operator ‘^’; and
- Inclusive OR operator ‘|’.
The syntax of _bitwise and logical expression_ is presented below:

```
bitwiseAndLogicalExpression:
expression'&'expression
|expression '^' expression
|expression '|' expression
;
```
```
These operators have different precedence. The operator ‘&’ has the highest, while ‘|’ has the lowest precedence.
Operators group left-to-right. Each operator is commutative if the operand expressions have no side effects, and asso-
ciative.
The bitwise and logical operators can compare two operands of a numeric type, or two operands of thebooleantype.
Otherwise, a compile-time error occurs.
```
### 7.26.1 Integer Bitwise Operators

```
Integer bitwise operators are ‘&’, ‘^’, and ‘|’ applied to operands of numeric types or typebigint.
If the type of one or both operands isdoubleorfloat, then the operand or operands are truncated first to the appro-
priate integer type. If the type of any operand isbyteorshort, then the operand is converted toint. If operands are
of different integer types, then the operand of a smaller type is converted to a larger type (see Numeric Types ) by using
Widening Numeric Conversions. If both operands are of typebigint, then no conversion is required. A compile-time
error occurs if one operand of typebigint, and the other operand is of a numeric type.
The resultant type of the bitwise operator is the type of its operands.
```
```
138 Chapter 7. Expressions
```

The resultant value of ‘&’ is the bitwise AND of the operand values.

The resultant value of ‘^’ is the bitwise exclusive OR of the operand values.

The resultant value of ‘|’ is the bitwise inclusive OR of the operand values.

### 7.26.2 Boolean Logical Operators

Boolean logical operators are ‘&’, ‘^’, and ‘|’ applied to operands of typeboolean.

If both operand values aretrue, then the resultant value of ‘&’ istrue. Otherwise, the result isfalse.

If the operand values are different, then the resultant value of ‘^’ istrue. Otherwise, the result isfalse.

If both operand values arefalse, then the resultant value of ‘|’ isfalse. Otherwise, the result istrue.

Thus, _boolean logical expression_ is of the boolean type.

## 7.27 Conditional-And Expression

The _conditional-and_ operator ‘&&’ is similar to ‘&’ (see _Bitwise and Logical Expressions_ ) but evaluates its right-hand-
side operand only if the value of the left-hand-side operand istrue.

The computation results of ‘&&’ and ‘&’ onbooleanoperands are the same. The right-hand-side operand of ‘&&’ is
not necessarily evaluated.

The syntax of _conditional-and expression_ is presented below:

conditionalAndExpression:
expression'&&' expression
;

A _conditional-and_ operator groups left-to-right.

A _conditional-and_ operator is fully associative as regards both the result value and side effects (i.e., the evaluations of
the expressions _((a)_ && _(b))_ && _(c)_ and _(a)_ && _((b)_ && _(c))_ produce the same result, and the same side effects occur in
the same order for any _a_ , _b_ , and _c_ ).

A _conditional-and_ expression is always of typebooleanexcept the extended semantics (see _Extended Conditional
Expressions_ ). A _conditional-and_ expression with extended semantics can be of the first expression type.

Each operand of the _conditional-and_ operator must be of typeboolean, or of a type mentioned in _Extended Conditional
Expressions_. Otherwise, a compile-time error occurs.

The left-hand-side operand expression is first evaluated at runtime.

If the resultant value isfalse, then the value of the _conditional-and_ expression isfalse. The evaluation of the
right-hand-side operand expression is omitted.

If the value of the left-hand-side operand istrue, then the right-hand-side expression is evaluated. The resultant value
is the value of the _conditional-and_ expression.

**7.27. Conditional-And Expression 139**


## 7.28 Conditional-Or Expression

The _conditional-or_ operator ‘||’ is similar to ‘|’ (see _Integer Bitwise Operators_ ) but evaluates its right-hand-side
operand only if the value of its left-hand-side operand isfalse.

The syntax of _conditional-or expression_ is presented below:

conditionalOrExpression:
expression'||' expression
;

A _conditional-or_ operator groups left-to-right.

A _conditional-or_ operator is fully associative as regards both the result value and side effects (i.e., the evaluations of
the expressions _((a)_ || _(b))_ || _(c)_ and _(a)_ || _((b)_ || _(c))_ produce the same result, and the same side effects occur in
the same order for any _a_ , _b_ , and _c_ ).

A _conditional-or_ expression is always of typebooleanexcept the extended semantics (see _Extended Conditional
Expressions_ ). A _conditional-or_ expression with extended semantics can be of the first expression type.

Each operand of the _conditional-or_ operator must be of typebooleanor type mentioned in _Extended Conditional
Expressions_. Otherwise, a compile-time error occurs.

The left-hand-side operand expression is first evaluated at runtime.

If the resultant value istrue, then the value of the _conditional-or_ expression istrue, and the evaluation of the right-
hand-side operand expression is omitted.

If the resultant value isfalse, then the right-hand-side expression is evaluated. The resultant value is the value of the
_conditional-or_ expression.

The computation results of ‘||’ and ‘|’ onbooleanoperands are the same, but the right-hand-side operand in ‘||’
cannot be evaluated.

## 7.29 Assignment

All _assignment operators_ group right-to-left (i.e.,𝑎=𝑏=𝑐means𝑎= (𝑏=𝑐). The value of _c_ is thus assigned to _b_ ,
and then the value of _b_ to _a_ ).

The syntax of _assignment expression_ is presented below:

assignmentExpression:
lhsExpression assignmentOperator rhsExpression
;

assignmentOperator
:'='
|'+=' |'-=' |'*=' |'=' | '%=' |`**=` |`/=`
|'<<=' |'>>=' |'>>>='
|'&=' |'|=' |'^='
(continues on next page)

**140 Chapter 7. Expressions**


```
(continued from previous page)
;
```
lhsExpression:
expression
;

rhsExpression:
expression
;

The first operand in an assignment operator represented by _lhsExpression_ must be _left-hand-side expression_ (see _Left-
Hand-Side Expressions_ ). This first operand denotes a variable.

Type of the variable is the type of the assignment expression.

The result of the _assignment expression_ at runtime is not a variable itself but the value of a variable after the assignment.

### 7.29.1 Simple Assignment Operator

The form of a simple assignment expression islhsExpression = rhsExpression.

A compile-time error occurs in the following situations:

- Type of _rhsExpression_ is not assignable (see _Assignability_ ) to the type of the variable; or
- Type of _lhsExpression_ is one of the following:
    **-** readonlyarray (see _Readonly Parameters_ ), while the converted type of _rhsExpression_ is a non-readonly
       array;
    **-** readonlytuple (see _Readonly Parameters_ ), while the converted type of _rhsExpression_ is a non-readonly
       tuple.

Otherwise, the assignment expression is evaluated at runtime in one of the following ways:

1. If _lhsExpression_ is a field access expressione.f(see _Field Access Expression_ ), possibly enclosed in parentheses,
    then:
       1. _lhsExpression e_ is evaluated: if the evaluation of _e_ completes abruptly, then so does the assignment expres-
          sion.
       2. _rhsExpression_ is evaluated: if the evaluation completes abruptly, then so does the assignment expression.
       3. If that evaluation completes normally, then the value of _rhsExpression_ is converted to the type of the field.
          In that case, the result of the conversion is assigned to the field.
2. If the _lhsExpression_ is an array reference expression (see _Array Indexing Expression_ ), possibly enclosed in paren-
    theses, then:
       1. Array reference subexpression of _lhsExpression_ is evaluated. If this evaluation completes abruptly, then so
          does the assignment expression. In that case, _rhsExpression_ and the index subexpression are not evaluated,
          and no assignment occurs.
       2. If the evaluation completes normally, then the index subexpression of _lhsExpression_ is evaluated. If this
          evaluation completes abruptly, then so does the assignment expression. In that case, _rhsExpression_ is not
          evaluated, and no assignment occurs.

**7.29. Assignment 141**


3. If the evaluation completes normally, then _rhsExpression_ is evaluated. If this evaluation completes abruptly,
    then so does the assignment expression, and no assignment occurs.
4. If the evaluation completes normally, but the value of the index subexpression is less than zero, or greater
    than, or equal to the _length_ of the array, thenRangeErroris thrown, and no assignment occurs.
5. If _lhsExpression_ denotes indexing of _fixed-size array_ , and the type of _rhsExpression_ is not a subtype of
    array element type, then _ArrayStoreError_ is thrown, and no assignment occurs.
6. Otherwise, the value of the index subexpression is used to select an element of the array referred to by the
    value of the array reference subexpression and the value of _rhsExpression_ is converted to the type of the
    array element. In that case, the result of the conversion is assigned to the array element.
3. If _lhsExpression_ is a record access expression (see _Record Indexing Expression_ ), possibly enclosed in parenthe-
ses, then:
1. Object reference subexpression of _lhsExpression_ is evaluated. If this evaluation completes abruptly, then so
does the assignment expression. In that case, _rhsExpression_ and the index subexpression are not evaluated,
and no assignment occurs.
2. If the evaluation completes normally, the index subexpression of _lhsExpression_ is evaluated. If this eval-
uation completes abruptly, then so does the assignment expression. In that case, _rhsExpression_ is not
evaluated, and no assignment occurs.
3. If the evaluation completes normally, _rhsExpression_ is evaluated. If this evaluation completes abruptly,
then so does the assignment expression. In that case, no assignment occurs.
4. Otherwise, the value of the index subexpression is used as thekey, and the value of _rhsExpression_ converted
to the type of the record value is used as thevalue. In that case, the assignment results in storing the key-
value pair in the record instance.

If none of the above is true, then the following three steps are performed:

1. _lhsExpression_ is evaluated to produce a variable. If the evaluation completes abruptly, then so does the assign-
    ment expression. In that case, _rhsExpression_ is not evaluated, and no assignment occurs.
2. If the evaluation completes normally, then _rhsExpression_ is evaluated. If the evaluation completes abruptly, then
    so does the assignment expression. In that case, no assignment occurs.
3. If that evaluation completes normally, then the value of _rhsExpression_ is converted to the type of the left-hand-
    side variable. In that case, the result of the conversion is assigned to the variable.

### 7.29.2 Compound Assignment Operators

A compound assignment expression in the form:

lhsExpression op= rhsExpression

is equivalent to

lhsExpression = ((lhsExpression) op (rhsExpression)) as T

whereTis type of _lhsExpression_ , except that _lhsExpression_ is evaluated only once.

While the nullish-coalescing assignment (??=) only evaluates the right operand, and assigns to the left operand if the
left operand isnullorundefined.

An assignment expression can be evaluated at runtime in one of the following ways:

1. If _lhsExpression_ is not an indexing expression:

**142 Chapter 7. Expressions**


- _lhsExpression_ is evaluated to produce a variable. If the evaluation completes abruptly, then so does the
    assignment expression. In that case, _rhsExpression_ is not evaluated, and no assignment occurs.
- If the evaluation completes normally, then the value of _lhsExpression_ is saved, and _rhsExpression_ is eval-
    uated. If the evaluation completes abruptly, then so does the assignment expression. In that case, no
    assignment occurs.
- If the evaluation completes normally, then the saved value of the left-hand-side variable, and the value of
    _rhsExpression_ are used to perform the binary operation as indicated by the compound assignment operator.
    If the operation completes abruptly, then so does the assignment expression. In that case, no assignment
    occurs.
- If the evaluation completes normally, then the result of the binary operation converts to the type of the
    left-hand-side variable. The result of such conversion is stored into the variable.
2. If _lhsExpression_ is an array reference expression (see _Array Indexing Expression_ ), then:
- Array reference subexpression of _lhsExpression_ is evaluated. If the evaluation completes abruptly, then so
does the assignment expression. In that case, the index subexpression, and _rhsExpression_ are not evaluated,
and no assignment occurs.
- If the evaluation completes normally, then the index subexpression of _lhsExpression_ is evaluated. If the
evaluation completes abruptly, then so does the assignment expression. In that case, _rhsExpression_ is not
evaluated, and no assignment occurs.
- If the evaluation completes normally, the value of the array reference subexpression refers to an array, and
the value of the index subexpression is less than zero, greater than, or equal to the _length_ of the array, then
RangeErroris thrown. In that case, no assignment occurs.
- If the evaluation completes normally, then the value of the index subexpression is used to select an array
element referred to by the value of the array reference subexpression. The value of this element is saved,
and then _rhsExpression_ is evaluated. If the evaluation completes abruptly, then so does the assignment
expression. In that case, no assignment occurs.
- If the evaluation completes normally, consideration must be given to the saved value of the array element
selected in the previous step. While this element is a variable of typeS, andTis type of _lhsExpression_ of
the assignment operator determined at compile time:
**-** IfTis a predefined value type, thenSis the same asT.
The saved value of the array element, and the value of _rhsExpression_ are used to perform the binary
operation of the compound assignment operator.
If this operation completes abruptly, then so does the assignment expression. In that case, no assign-
ment occurs.
If this evaluation completes normally, then the result of the binary operation converts to the type of
the selected array element. The result of the conversion is stored into the array element.
**-** IfTis a reference type, then it must bestring.
Smust also be astringbecause the classstringis the _final_ class. The saved value of the array
element, and the value of _rhsExpression_ are used to perform the binary operation (string concatenation)
of the compound assignment operator ‘+=’. If this operation completes abruptly, then so does the
assignment expression. In that case, no assignment occurs.
**-** If the evaluation completes normally, then thestringresult of the binary operation is stored into the
array element.
3. If _lhsExpression_ is a record access expression (see _Record Indexing Expression_ ):

**7.29. Assignment 143**


- The object reference subexpression of _lhsExpression_ is evaluated. If this evaluation completes abruptly,
    then so does the assignment expression. In that case, the index subexpression and _rhsExpression_ are not
    evaluated, and no assignment occurs.
- If this evaluation completes normally, then the index subexpression of _lhsExpression_ is evaluated. If the
    evaluation completes abruptly, then so does the assignment expression. In that case, _rhsExpression_ is not
    evaluated, and no assignment occurs.
- If this evaluation completes normally, the value of the object reference subexpression and the value of index
    subexpression are saved, then _rhsExpression_ is evaluated. If the evaluation completes abruptly, then so does
    the assignment expression. In that case, no assignment occurs.
- If this evaluation completes normally, the saved values of the object reference subexpression and index
    subexpression (as the _key_ ) are used to get the _value_ that is mapped to the _key_ (see _Record Indexing Expres-_
    _sion_ ), then this _value_ and the value of _rhsExpression_ are used to perform the binary operation as indicated
    by the compound assignment operator. If the operation completes abruptly, then so does the assignment
    expression. In that case, no assignment occurs.
- If the evaluation completes normally, then the result of the binary operation is stored as the key-value pair
    in the record instance (as in _Simple Assignment Operator_ ).

### 7.29.3 Left-Hand-Side Expressions

_Left-hand-side expression_ is an _expression_ that is one of the following:

- Named variable;
- Field or setter resultant from a field access (see _Field Access Expression_ ); or
- Array or record element access (see _Indexing Expressions_ ).

A compile-time error occurs in the following situations:

- _Expression_ contains the chaining operator ‘?.’ (see _Chaining Operator_ );
- Result of _expression_ is not a variable.

## 7.30 Ternary Conditional Expressions

The ternary conditional expression ‘condition?whenTrue:whenFalse’ uses the boolean value of the first expression
(condition) to decide which of other two expressions to evaluate:

ternaryConditionalExpression:
expression'?'expression ':' expression
;

The ternary conditional operator groups right-to-left (i.e., the meaning of𝑎?𝑏:𝑐?𝑑:𝑒?𝑓:𝑔and𝑎?𝑏: (𝑐?𝑑: (𝑒?𝑓:𝑔))
is the same).

The ternary conditional operator ‘condition?whenTrue:whenFalse’ consists of three operand expressions with the
separators ‘?’ between the first and the second expression, and ‘:’ between the second and the third expression.

**144 Chapter 7. Expressions**


```
A compile-time error occurs if the first expression is not of typeboolean, or a type mentioned in Extended Conditional
Expressions.
Type of the ternary conditional expression is determined as the union of types of the second and the third expressions
further normalized in accordance with the process discussed in Union Types Normalization. If the second and the third
expressions are of the same type, then this is the type of the conditional expression.
The following steps are performed as the evaluation of a ternary conditional expression occurs at runtime:
```
1. The first operand (condition) of a ternary conditional expression is evaluated first.
2. If the value of the first operand istrue, then the second operand expression (whenTrue) is evaluated. Otherwise,
    the third operand expression (whenFalse) is evaluated. The result of successful evaluation is the result of the
    ternary conditional expression.
The examples below represent different scenarios with standalone expressions:

1 class A {}
2 class BextendsA {}
3
4 condition ?newA() :newB()// A | B => A
5
6 condition? 5 : 6 // int
7
8 condition? "5" : 6 // "5" | int

## 7.31 String Interpolation Expressions

```
‘ String interpolation expression ’ is a multiline string literal, i.e., a string literal delimited with backticks (see Multiline
String Literal for detail) that contains at least one embedded expression.
The syntax of string interpolation expression is presented below:
```
```
stringInterpolation:
'`'(BacktickCharacter| embeddedExpression)* '`'
;
```
```
embeddedExpression:
'${'expression '}'
;
```
```
An ‘ embedded expression ’ is an expression specified inside curly braces preceded by the dollar sign ‘$’. A string
interpolation expression is of typestring(see Type string ).
When evaluating a string interpolation expression , the result of each embedded expression substitutes that embedded
expression. An embedded expression must be of typestring. Otherwise, the implicit conversion tostringtakes
place in the same way as with the string concatenation operator (see String Concatenation ):
```
1 leta = 2
2 letb = 2
3 console.log(`The result of${a} *${b} is ${a * b}`)
4 // prints: The result of 2 * 2 is 4

```
7.31. String Interpolation Expressions 145
```

```
The string concatenation operator can be used to rewrite the above example as follows:
```
1 leta = 2
2 letb = 2
3 console.log("The result of " + a + " * " + b + " is " + a * b)

```
An embedded expression can contain nested multiline strings.
```
## 7.32 Lambda Expressions

```
Lambda expression fully defines an instance of a function type (see Function Types ) by providing optional annotation
usage (see Using Annotations ), optionalasyncmark (see Async Lambdas ), mandatory lambda signature, and its body.
The declaration of lambda expression is generally similar to that of a function declaration (see Function Declarations ),
except that a lambda expression has no function name specified, and can have types of parameters omitted.
The syntax of lambda expression is presented below:
```
```
lambdaExpression:
annotationUsage? 'async'? lambdaSignature '=>' lambdaBody
;
```
```
lambdaBody:
expression| block
;
```
```
lambdaSignature:
'(' lambdaParameterList? ')' returnType?
|identifier
;
```
```
lambdaParameterList:
lambdaParameter (',' lambdaParameter)* (','restParameter)? ','?
|restParameter ','?
;
```
```
lambdaParameter:
annotationUsage? (lambdaRequiredParameter |lambdaOptionalParameter)
;
```
```
lambdaRequiredParameter:
identifier(':' type)?
;
```
```
lambdaOptionalParameter:
identifier'?'(':' type)?
;
```
```
lambdaRestParameter:
'...'lambdaRequiredParameter
;
```
```
146 Chapter 7. Expressions
```

```
The usage of annotations is discussed in Using Annotations.
The examples of usage are presented below:
```
```
1 (x:number):number => {return Math.sin(x) }// block as lambda body
2 (x:number) => Math.sin(x) // expression as lambda body
3 e => e // shortest form of lambda
```
```
A lambda expression evaluation creates an instance of a function type (see Function Types ) as described in detail in
Runtime Evaluation of Lambda Expressions.
```
### 7.32.1 Lambda Signature

```
Similarly to function declarations (see Function Declarations ), a lambda signature is composed of formal parameters
and optional return types. Unlike function declarations, type annotations of formal parameters can be omitted.
```
1 functionfoo<T> (a: (p1: T, ...p2:T[]) => T) {}
2 // All calls to foo pass valid lambda expressions in different forms
3 foo (e => e)
4 foo ((e1, e2) => e1)
5 foo ((e1, e2:Object) => e1)
6 foo ((e1: Object, e2) => e1)
7 foo ((e1: Object, e2, e3) => e1)
8 foo ((e1: Object, ...e2) => e1)
9
10 foo ((e1: Object, e2:Object) => e1)
11
12 functionbar<T> (a: (...p:T[]) => T) {}
13 // Type can be omitted for the rest parameter
14 bar ((...e) => e)
15
16 functiongoo<T> (a: (p?: T) => T) {}
17 // Type can be omitted for the optional parameter
18 goo ((e?) => e)

```
The specification of scope is discussed in Scopes , and shadowing details of formal parameter declarations in Shadowing
by Parameter.
A compile-time error occurs if:
```
- Lambda expression declares two formal parameters with the same name.
- Formal parameter contains no type provided, and type cannot be derived by _Type Inference_.

```
7.32. Lambda Expressions 147
```

### 7.32.2 Lambda Body

_Lambda body_ can be a single expression or a block (see _Block_ ). Similarly to the body of a method or a function, a
lambda body describes the code to be executed when a lambda expression call occurs (see _Function Call Expression_ ).

The meanings of names, and of the keywordsthisandsuper(along with the accessibility of the referred declarations)
are the same as in the surrounding context. However, lambda parameters introduce new names.

If any local variable or formal parameter of the surrounding context is used but not declared in a lambda body, then the
local variable or formal parameter is _captured_ by the lambda.

If an instance member of the surrounding type is used in the lambda body defined in a method, thenthisis _captured_
by the lambda.

A compile-time error occurs if a local variable is used in a lambda body but is neither declared in nor assigned before
it.

If a _lambda body_ is a singleexpression, then it is handled as follows:

- If the expression is a _call expression_ with return typevoid, then the body is equivalent to the block: {
    expression }.
- Otherwise, the body is equivalent to the block:{ return expression }.

If _lambda signature_ return type is notvoid(see _Type void_ ) ornever(see _Type never_ ), and the execution path of the
lambda body has no return statement (see _return Statements_ ) or no single expression as a body, then a compile-time
error occurs.

### 7.32.3 Lambda Expression Type

_Lambda expression type_ is a function type (see _Function Types_ ) that has the following:

- Lambda parameters (if any) as parameters of the function type; and
- Lambda return type as the return type of the function type.

**Note**. Lambda return type can be inferred from the _lambda body_ and thus the return type can be dropped off.

```
1 const lambda = () => {return123 } // Type of the lambda is () => int
2 const int_var:int= lambda()
```
### 7.32.4 Runtime Evaluation of Lambda Expressions

The evaluation of a lambda expression itself never causes the execution of the lambda body. If completing normally at
runtime, the evaluation of a lambda expression produces a new instance of a function type (see _Function Types_ ) that
corresponds to the lambda signature. In that case, it is similar to the evaluation of a class instance creation expression
(see _New Expressions_ ).

If the available space is not sufficient for a new instance to be created, then the evaluation of the lambda expression
completes abruptly, andOutOfMemoryErroris thrown.

Every time a lambda expression is evaluated, the outer variables referred to by the lambda expression are captured as
follows:

**148 Chapter 7. Expressions**


```
1 function foo() {
2 lety: int= 1
3 letx = () => {returny+1 } // 'y'is *captured*.
4 console.log(x()) // Output: 2
5 }
```
```
The captured variable is not a copy of the original variable. If the value of the variable captured by the lambda changes,
then the original variable is implied to change:
```
```
1 function foo() {
2 lety: int= 1
3 letx = () => { y++ }// 'y'is *captured*.
4 console.log(y)// Output: 1
5 x()
6 console.log(y)// Output: 2
7 }
```
```
Capturing within the function scope is highlighted by the following example:
```
1 function capturingFunction() {// Function scope
2 letv: number= 0// A captured variable
3 return (p:number) => {
4 console.log ("Previous value: ", v, " new value: ", p)
5 v = p
6 }
7 }
8
9 const func1 = capturingFunction ()
10 const func2 = capturingFunction ()
11 // Note: func1 and func2 are two different function type instances
12
13 func1(11) // Previous value: 0 new value: 11
14 func2(22) // Previous value: 0 new value: 22
15 func1(33) // Previous value: 11 new value: 33
16 func2(44) // Previous value: 22 new value: 44
17 /* Note:
18 func1 calls work with their own version of variable'v'
19 func2 calls work with their own version of variable'v'
20 */

```
Capturing within the loop scope is highlighted by the following example:
```
1 const l = () => {}
2 const storage = [l, l, l, l, l] // fill array with some lambdas
3
4 for(letindex = 0; index < 5; index++) {
5 storage [index] = () => { console.log ("Index ", index) }
6 // Every lambda captures loop index variable
7 }
8 for(letindex = 0; index < 5; index++) {
9 storage[index]()// Captured indices printed
10 }

```
7.32. Lambda Expressions 149
```

## 7.33 Constant Expressions

```
Constant expressions are expressions with values that can be evaluated at compile time.
The syntax of constant expression is presented below:
```
```
constantExpression:
expression
;
```
```
A constant expression is an expression of a value type (see Value Types ), or of typestringthat completes normally
while being composed only of the following:
```
- Literals of a predefined value types, and literals of typestring(see _Literals_ );
- Enumeration type constants;
- Unary operators ‘+’, ‘-’, ‘~’, and ‘!’, but not ‘++’ or ‘--’ (see _Unary Plus_ , _Unary Minus_ , _Prefix Increment_ , and
    _Prefix Decrement_ );
- Casting conversions to numeric types (see _Cast Expression_ );
- Multiplicative operators ‘*’, ‘/’, and ‘%’ (see _Multiplicative Expressions_ );
- Additive operators ‘+’ and ‘-’ (see _Additive Expressions_ );
- Shift operators ‘<<’, ‘>>’, and ‘>>>’ (see _Shift Expressions_ );
- Relational operators ‘<’, ‘<=’, ‘>’, and ‘>=’ (see _Relational Expressions_ );
- Equality operators ‘==’ and ‘!=’ (see _Equality Expressions_ );
- Bitwise and logical operators ‘&’, ‘^’, and ‘|’ (see _Bitwise and Logical Expressions_ );
- Conditional-and operator ‘&&’ (see _Conditional-And Expression_ ), and conditional-or operator ‘||’ (see
    _Conditional-Or Expression_ );
- Ternary conditional operator ‘condition?whenTrue:whenFalse’ (see _Ternary Conditional Expressions_ );
- Parenthesized expressions (see _Parenthesized Expression_ ) that contain constant expressions;
- Simple names or qualified names that refer to constants (see _Constant Declarations_ ) with constant expressions
    as initializers, declared in the same module.
The examples of constant expressions are presented below:

1 const a = 2
2
3 // Constant expressions:
4 1 + 2
5 a + 1
6 "aa" + "bb"
7 (a < 0) || (a > 5)

```
Note. The following expressions are not constant expressions:
```
```
150 Chapter 7. Expressions
```

1 letx = 2
2
3 // non-constant expressions:
4 x + 1
5 0x7fas short

```
7.33. Constant Expressions 151
```

**152 Chapter 7. Expressions**


##### CHAPTER

### EIGHT

### STATEMENTS

_Statements_ are designed to control execution.

The syntax of _statements_ is presented below:

statement:
expressionStatement
|block
|localDeclaration
|ifStatement
|loopStatement
|breakStatement
|continueStatement
|returnStatement
|switchStatement
|throwStatement
|tryStatement
;

### 8.1 Normal and Abrupt Statement Execution

The actions that every statement performs in a normal mode of execution are specific for the particular kind of statement.
Normal modes of evaluation for each kind of statement are described in the following sections.

A statement execution is considered to _complete normally_ if the desired action is performed without an error being
thrown. On the contrary, a statement execution is considered to _complete abruptly_ if it causes an error thrown.

### 8.2 Expression Statements

Any expression can be used as a statement.

The syntax of _expression statement_ is presented below:

##### 153


expressionStatement:
expression
;

The execution of a statement leads to the execution of the expression. The result of such execution is discarded.

### 8.3 Block

A sequence of statements (see _Statements_ ) enclosed in balanced braces forms a _block_.

The syntax of _block statement_ is presented below:

block:
'{' statement*'}'
;

The execution of a block means that all block statements, except type declarations, are executed one after another in
the textual order of their appearance within the block while an error is thrown (see _Errors_ ), or until a return occurs (see
_return Statements_ ).

If a block is the body of afunctionDeclaration(see _Function Declarations_ ) or aclassMethodDeclaration
(see _Method Declarations_ ) declared implicitly or explicitly with return typevoid(see _Type void_ ), then the block can
contain no return statement at all. Such a block is equivalent to one that ends in areturnstatement, and is executed
accordingly.

### 8.4 Local Declarations

_Local declarations_ define new mutable or immutable variables within the enclosing context.

Letandconstdeclarations have the initialization part that presumes execution, and actually act as statements.

The syntax of _local declaration_ is presented below:

localDeclaration:
annotationUsage?
(variableDeclaration
|constantDeclaration
)
;

**154 Chapter 8. Statements**


```
The visibility of a local declaration name is determined by the surrounding function or method, and by the block scope
rules (see Scopes ). In order to avoid ambiguous interpretation, appropriate sections of this Specification are dedicated
to a detailed discussion of the following entities:
```
- _if Statements_ ,
- _for Statements_ ,
- _for-of Statements_.
The usage of annotations is discussed in _Using Annotations_.

### 8.5 ifStatements

```
Anifstatement allows executing alternative statements (if provided) under certain conditions.
The syntax of if statement is presented below:
```
```
ifStatement:
'if' '('expression ')' thenStatement
('else' elseStatement)?
;
```
```
thenStatement:
statement
;
```
```
elseStatement:
statement
;
```
```
Type of expression must beboolean, or a type mentioned in Extended Conditional Expressions. Otherwise, a compile-
time error occurs.
If an expression is successfully evaluated astrue, then athenStatementis executed. Otherwise, anelseStatement
is executed (if provided).
Anyelsecorresponds to the nearest precedingifof anifstatement:
```
1 if (Cond1)
2 if (Cond2) statement1
3 elsestatement2 // Executes only if: Cond1 && !Cond2

```
A Block can be used to combine theelsepart with the initialifas follows:
```
1 if (Cond1) {
2 if (Cond2) statement1
3 }
4 elsestatement2 // Executes if: !Cond1

```
IfthenStatementorelseStatementis any kind of a statement but not a block (see Block ), then no block scope (see
Scopes ) is created for such a statement.
```
```
8.5. if Statements 155
```

1 functionfoo(Cond1: boolean) {
2 if (Cond1)letx: number = 1
3 x = 2// OK
4
5 if (Cond1) {
6 letx: number= 10; // OK, then-block scope
7 lety: number= x;
8 }
9 else{
10 letx: number= 20 // OK, no conflict, else-block scope
11 y = x; // CTE, no y in scope
12 }
13
14 console.log(x) // OK, prints 2
15 console.log(y) // CTE, y unknown
16 }

### 8.6 Loop Statements

```
ArkTS has four kinds of loops. A loop of each kind can be optionally labelled with an identifier. The identifier can be
used only by the break Statements and continue Statements contained in the loop body.
The syntax of loop statements is presented below:
```
```
loopStatement:
(identifier ':')?
whileStatement
|doStatement
|forStatement
|forOfStatement
;
```
```
A compile-time error occurs if the label identifier is not used withinloopStatement, or is used in lambda expressions
(see Lambda Expressions ) within a loop body.
```
1 label: for(i = 1; i < 10; i++) {
2 const f1 = () => {
3 while(true) {
4 continuelabel// Compile-time error
5 }
6 }
7 const f2 = () => {
8 do
9 breaklabel // Compile-time error
10 while(true)
11 }
12 }

```
156 Chapter 8. Statements
```

### 8.7 whileStatements anddoStatements

```
Awhilestatement and adostatement evaluate an expression and execute the statement repeatedly till the expression
value istrue. The key difference is that awhileStatementstarts from evaluating and checking the expression value,
and adoStatementstarts from executing the statement.
The syntax of while and do statements is presented below:
```
```
whileStatement:
'while' '(' expression')'statement
;
```
```
doStatement
:'do' statement'while' '(' expression')'
;
```
```
Type of expression must beboolean, or a type mentioned in Extended Conditional Expressions. Otherwise, a compile-
time error occurs.
```
### 8.8 forStatements

```
The syntax of for statements is presented below:
```
```
forStatement:
'for' '('forInit?';'forContinue?';'forUpdate?')'statement
;
```
```
forInit:
expressionSequence
|variableDeclarations
;
```
```
forContinue:
expression
;
```
```
forUpdate:
expressionSequence
;
```
```
Type of forContinue expression must beboolean, or a type mentioned in Extended Conditional Expressions. Other-
wise, a compile-time error occurs.
```
1 // existing variable is used as a loop index variable
2 leti: number
3 for(i = 1; i < 10; i++) {
(continues on next page)

```
8.7. while Statements and do Statements 157
```

(continued from previous page)
4 console.log(i)
5 }
6
7 // new variable is declared as a loop index variable with its type
8 // explicitly specified
9 for(leti: number = 1; i < 10; i++) {
10 console.log(i)
11 }
12
13 // new variable is declared as loop index variable with its type
14 // inferred from its initialization part of the declaration
15 for(leti = 1; i < 10; i++) {
16 console.log(i)
17 }

```
A variable declared in the forInit -part has the loop scope. It can be used in a forContinue expression, a forUpdate
expression, a single-body statement, or in a body block if enclosed in parentheses:
```
```
1 // forInit declaration and no body block
2 letk: number= 0
3 for(leti: number = 1; i < 10; i++)
4 k += i
5 console.log(k)
6 // i = k // CTE when uncommented
7 leti: number= k // OK
```
### 8.9 for-ofStatements

```
Afor-ofloop iterates elements ofarrayorstring, or an instance of iterable class or interface (see Iterable Types ).
The syntax of for-of statements is presented below:
```
```
forOfStatement:
'for' '('forVariable'of'expression ')' statement
;
```
```
forVariable:
identifier| ('let' |'const')identifier (':' type)?
;
```
```
A compile-time error occurs if the type of an expression is notarray,string, or an iterable type.
The execution of afor-ofloop starts from the evaluation ofexpression. If the evaluation is successful, then the
resultant expression is used for loop iterations (execution of thestatement). On each iteration, forVariable is set to
successive elements of thearray,string, or the result of class iterator advancement.
If forVariable has the modifiersletorconst, then a new variable is declared in the loop scope. The new variable
is accessible only inside the loop body. Otherwise, the variable is as declared above. The modifierconstprohibits
assignments into forVariable , whileletallows modifications.
```
```
158 Chapter 8. Statements
```

```
The type of forVariable declared inside the loop is inferred to be that of the iterated elements, namely:
```
- T, ifArray<T>orFixedArray<T>instance is iterated;
- string, ifstringvalue is iterated;
- Type argument of the _iterator_ , if an instance of the _iterable_ type is iterated.
If _forVariable_ is declared outside the loop, then the type of an iterated element must be assignable (see _Assignability_ )
to the type of the variable. Otherwise, a compile-time error occurs.

1 // existing variable's'
2 lets :string
3 for(s of "a string object") {
4 console.log(s)
5 }
6
7 // new variable 's', its type is inferred from expression after'of'
8 for(lets of "a string object") {
9 console.log(s)
10 }
11
12 // new variable 'element', its type is inferred from expression after'of'.
13 // as 'const'it cannot be assigned with a new value in the loop body
14 for(const elementof [1, 2, 3]) {
15 console.log(element)
16 element = 66// Compile-time error as'element'is 'const'
17 }

```
Explicit type annotation of forVariable is allowed as an experimental feature (see For-of Explicit Type Annotation ).
```
### 8.10 breakStatements

```
Abreakstatement transfers control out of the enclosingloopStatementorswitchStatement. If abreakstatement
is used outside aloopStatementor aswitchStatement, then a compile-time error occurs.
The syntax of break statement is presented below:
```
```
breakStatement:
'break' identifier?
;
```
```
Abreakstatement with the label identifier transfers control out of the enclosing statement with the same label identifier.
If there is no enclosing loop statement with the same label identifier (within the body of the surrounding function or
method), then a compile-time error occurs.
A statement without a label transfers control out of the innermost enclosingswitch,while,do,for, orfor-of
statement. IfbreakStatementis placed outsideloopStatementorswitchStatement, then a compile-time error
occurs.
Examples ofbreakstatements with and without a label are presented below:
```
```
8.10. break Statements 159
```

1 // Single iteration
2 while (true) {
3 console.log("iteration") // get printed exactly once
4 break;
5 }
6
7 leta: number= 0
8 outer:
9 do{
10 for(a = 0; a < 10; a++) {
11 if (a == 1)break outer
12 console.log("inner") // get printed only once
13 }
14 console.log(a) // Never reached
15 }while (true) // condition never used

### 8.11 continueStatements

```
Acontinuestatement stops the execution of the current loop iteration, and transfers control to the next iteration.
Appropriate checks of loop exit conditions depend on the kind of the loop.
The syntax of continue statement is presented below:
```
```
continueStatement:
'continue'identifier?
;
```
```
Acontinuestatement with no label transfers control to the next iteration of the enclosingloopstatement. If there is
no enclosingloopstatement within the body of the surrounding function or method, then a compile-time error occurs.
Acontinuestatement with the label identifier transfers control to the next iteration of the enclosing loop statement
with the same label identifier. If there is no enclosing loop statement with the same label identifier (within the body of
the surrounding function or method), then a compile-time error occurs.
Examples ofcontinuestatements with and without a label are presented below:
```
1 // continue // would cause CTE if uncommented
2
3 // continue without label
4 // will print 0, 1, 2, 4 (3 skipped)
5 for(leta: number = 0; a < 5; a++){
6 if (a == 3)continue
7 console.log("a = " + a)
8 }
9
10 leta: number
11 outer:
12 do{
13 for(a = 0; a < 10; a++) {
14 if (a > 1)continueouter
(continues on next page)

```
160 Chapter 8. Statements
```

```
(continued from previous page)
```
15 console.log("inner") // get printed only twice
16 }
17 console.log("Outer")// Never reached
18 }while (false)

### 8.12 returnStatements

```
Areturnstatement can have or not have an expression.
The syntax of return statement is presented below:
```
```
returnStatement:
'return'expression?
;
```
```
Areturnstatement with expression can only occur inside a function, a method, or a lambda body with non-void
return type.
Areturnstatement (with no expression ) can occur inside one of the following:
```
- Initializer block;
- Constructor body;
- Function, method, or lambda body with return typevoid(see _Type void_ );
A compile-time error occurs if areturnstatement is found in:
- Top-level statements (see _Top-Level Statements_ );
- Functions or methods with return typevoid(see _Type void_ ) that have an expression;
- Functions or methods with a non-voidreturn type that have no expression.
The execution of areturnStatementleads to the termination of the surrounding function, method, or initializer. If
an _expression_ is provided, the resultant value is the evaluated _expression_.
In case of constructors, initializer blocks, and top-level statements, the control is transferred out of the scope of the
construction, but no result is required. Other statements of the surrounding function, method body, initializer block, or
top-level statement are not executed.

### 8.13 switchStatements

```
Aswitchstatement transfers control to a statement or a block by using the result of successful evaluation of the value
of aswitchexpression.
The syntax of switch statement is presented below:
```
```
8.12. return Statements 161
```

```
switchStatement:
(identifier ':')? 'switch' '('expression ')' switchBlock
;
```
```
switchBlock
:'{'caseClause*defaultClause? caseClause*'}'
;
```
```
caseClause
:'case'expression ':'statement*
;
```
```
defaultClause
:'default' ':' statement*
;
```
```
Theswitchexpression type must be of typechar,byte,short,int,long,string, orenum.
If available, an optional identifier allows thebreakstatement to transfer control out of a nestedswitchorloop
statement (see break Statements ).
A compile-time error occurs if not all of the following is true:
```
- Every case expression type is assignable (see _Assignability_ ) to the type of theswitchstatement expression.
- In aswitchstatement expression of typeenum, every case expression associated with theswitchstatement is
    of typeenum.
- No two case constant expressions (see _Constant Expressions_ ) have identical values.
- No case expression isnull.

1 letarg = prompt("Enter a value?");
2 switch (arg) {
3 case' 0 ':
4 case' 1 ':
5 console.log('One or zero')
6 break
7 case' 2 ':
8 console.log('Two')
9 break
10 default:
11 console.log('An unknown value')
12 }

```
The execution of aswitchstatement starts from the evaluation of theswitchexpression.
The value of theswitchexpression is compared repeatedly to the value of case expressions starting from the top till
the first match. The match means that particular case expression value equals the value of theswitchexpression in
terms of the operator ‘==’. However, if the expression value is of typestring, then the equality for strings determines
the equality.
So, in case of match execution is transferred to the set of statements of the caseClause where match occurred. If this
set of statements executes break statement then the wholeswitchstatement terminates. If no break statement was
executed then execution continues through all remaining caseClause*s as well as *defaultClause at last if it is present.
If no match occurred and defaultClause is present then it is executed.
```
```
162 Chapter 8. Statements
```

### 8.14 throwStatements

Athrowstatement causes an _error_ object to be created and raised (see _Error Handling_ ). It immediately transfers
control, and can exit multiple statements, constructors, functions, and method calls until atrystatement (see _try
Statements_ ) is found that catches the value thrown. If notrystatement is found, thenUncaughtExceptionErroris
thrown.

The syntax of _throw statement_ is presented below:

throwStatement:
'throw' expression
;

The expression type must be assignable (see _Assignability_ ) to typeError. Otherwise, a compile-time error occurs.

This implies that the object thrown is nevernull.

Errors can be thrown at any place in the code.

### 8.15 tryStatements

Atrystatement runs block of code, and provides optionalcatchclause to handle errors (see _Error Handling_ ) which
may occur during block of code execution.

The syntax of _try statement_ is presented below:

tryStatement:
'try' block catchClause?finallyClause?
;

catchClause:
'catch' '('identifier ')' block
;

finallyClause:
'finally' block
;

Atrystatement must contain either afinallyclause, or acatchclause. Otherwise, a compile-time error occurs.

If thetryblock completes normally, then no action is taken, and nocatchclause block is executed.

If an error is thrown in thetryblock directly or indirectly, then the control is transferred to thecatchclause.

**8.14.** throw **Statements 163**


#### 8.15.1 catchClause

```
Acatchclause consists of two parts:
```
- A _catch identifier_ that provides access to an object associated with the _error_ thrown; and
- A block of code that handles the error.
The type of _catch identifier_ inside the block isError(see _Error Handling_ ).

1 class ZeroDivisorextendsError {}
2
3 functiondivide(a: number, b:number):number {
4 if (b == 0)
5 throw newZeroDivisor()
6 returna / b
7 }
8
9 functionprocess(a: number, b:number):number {
10 try{
11 letres = divide(a, b)
12
13 // further processing ...
14 return res
15 }
16 catch(e) {
17 return einstanceof ZeroDivisor? -1 : 0
18 }
19 }

```
Acatchclause handles all errors at runtime. It returns ‘ -1 ’ for theZeroDivisor, and ‘ 0 ’ for all other errors.
```
#### 8.15.2 finallyClause

```
Afinallyclause defines the set of actions in the form of a block to be executed without regard to whether atry-catch
completes normally or abruptly.
The syntax of finally clause is presented below:
```
```
finallyClause:
'finally'block
;
```
```
Afinallyblock is executed without regard to how (by reachingreturnortry-catchend or raising new error ) the
program control is transferred out. Thefinallyblock is particularly useful to ensure proper resource management.
Any required actions (e.g., flush buffers and close file descriptors) can be performed while leaving thetry-catch:
```
```
classSomeResource {
// some API
// ...
close() {}
}
(continues on next page)
```
```
164 Chapter 8. Statements
```

```
(continued from previous page)
```
functionProcessFile(name:string) {
letr =newSomeResource()
try{
// some processing
}
finally{
// finally clause will be executed after try-catch is
executed normally or abruptly
r.close()
}
}

#### 8.15.3 tryStatement Execution

1. Atryblock and the entiretrystatement complete normally if nocatchblock is executed. The execution of a
    tryblock completes abruptly if an error is thrown inside thetryblock.
2. The the execution of atryblock completes abruptly if error _x_ is thrown inside thetryblock. If thecatchclause
    is present, and the execution of the body of thecatchclause completes normally, then the entiretrystatement
    completes normally. Otherwise, thetrystatement completes abruptly.
3. If nocatchclause is in place, then the error is propagated to the surrounding and caller scopes until reaching
    the scope with thecatchclause to handle the error. If there is no such scope, then the whole coroutine stack
    (see _Coroutines (Experimental)_ ) is discarded. Subsequent steps are then defined by the execution environment.
4. Iffinallyclause is in place, and its execution completes abruptly, then thetrystatement also completes
    abruptly.

**8.15.** try **Statements 165**


**166 Chapter 8. Statements**


##### CHAPTER

### NINE

### CLASSES

Class declarations introduce new reference types and describe the manner of their implementation.

A class body contains declarations and initializer blocks.

Declarations can introduce class members (see _Class Members_ ) or class constructors (see _Constructor Declaration_ ).

The body of the declaration of a member comprises the scope of a declaration (see _Scopes_ ).

Class members include:

- Fields,
- Methods, and
- Accessors.

Class members can be _declared_ or _inherited_.

Every member is associated with the class declaration it is declared in.

Field, method, accessor and constructor declarations can have the following access modifiers (see _Access Modifiers_ ):

- Public,
- Protected,
- Private.

Every class defines two class-level scopes (see _Scopes_ ): one for instance members, and the other for static members. It
means that two members of a class can have the same name if one is static while the other is not.

### 9.1 Class Declarations

Every class declaration defines a _class type_ , i.e., a new named reference type.

The class name is specified by an _identifier_ inside a class declaration.

IftypeParametersare defined in a class declaration, then that class is a _generic class_ (see _Generics_ ).

The syntax of _class declaration_ is presented below:

classDeclaration:
classModifier?'class'identifier typeParameters?
classExtendsClause? implementsClause?
(continues on next page)

##### 167


```
(continued from previous page)
classMembers
;
```
```
classModifier:
'abstract'| 'final'
;
```
```
Classes with thefinalmodifier are an experimental feature discussed in Final Classes.
The scope of a class declaration is specified in Scopes.
An example of a class is presented below:
```
1 class Point {
2 publicx: number
3 publicy: number
4 public constructor(x :number, y : number) {
5 this.x = x
6 this.y = y
7 }
8 publicdistanceBetween(other: Point):number {
9 return Math.sqrt(
10 (this.x - other.x) * (this.x - other.x) +
11 (this.y - other.y) * (this.y - other.y)
12 )
13 }
14 staticorigin =newPoint(0, 0)
15 }

#### 9.1.1 Abstract Classes

```
A class with the modifierabstractis known as abstract class. An abstract class is a class that cannot be instantiated,
i.e., no objects of this type can be created. It serves as a blueprint for other classes by defining common fields and
methods that subclasses must implement. Abstract classes can contain both abstract and concrete methods.
A compile-time error occurs if an attempt is made to create an instance of an abstract class:
```
```
1 abstract classX {
2 field:number
3 constructor(p:number) {this.field = p }
4 }
5 letx =newX(42)
6 // Compile-time error: Cannot create an instance of an abstract class.
```
```
Subclasses of an abstract class can be abstract or non-abstract. A non-abstract subclass of an abstract superclass can be
instantiated. As a result, a constructor for the abstract class, and field initializers for non-static fields of that class are
executed:
```
```
1 abstract classBase {
2 field:number
(continues on next page)
```
```
168 Chapter 9. Classes
```

```
(continued from previous page)
```
3 constructor(p:number) {this.field = p }
4 }
5
6 classDerivedextendsBase {
7 constructor(p:number) {super(p) }
8 }

```
A method with the modifierabstractis considered an abstract method (see Abstract Methods ). Abstract methods
have no bodies, i.e., they can be declared but not implemented.
Only abstract classes can have abstract methods. A compile-time error occurs if a non-abstract class has an abstract
method:
```
1 classY {
2 abstract method (p:string)
3 /* Compile-time error: Abstract methods can only
4 be within an abstract class. */
5 }

```
A compile-time error occurs if an abstract method declaration contains the modifiersfinaloroverride.
```
1 abstract classY {
2 final abstract method (p:string)
3 // Compile-time error: Abstract methods cannot be final
4 }

### 9.2 Class Extension Clause

```
All classes except classObjectcan contain theextendsclause that specifies the base class , or the direct superclass
of the current class. In this situation, the current class is a derived class , or a direct subclass. Any class, except class
Objectthat has noextendsclause, is assumed to have theextends Objectclause.
The syntax of class extension clause is presented below:
```
```
classExtendsClause:
'extends'typeReference
;
```
```
A compile-time error occurs if:
```
- typeReferencerefers directly to, or is an alias of any non-class type, e.g., of interface, enumeration, union,
    function, or utility type.
- Class type named bytypeReferenceis not accessible (see _Accessible_ ).
- Anextendsclause appears in the declaration of the classObject.
- Theextendsgraph has a cycle.
_Class extension_ implies that a class inherits all members of the direct superclass.
**Note**. Private members are inherited from superclasses, but are not accessible (see _Accessible_ ) within subclasses:

```
9.2. Class Extension Clause 169
```

1 class Base {
2 /* All methods are accessible in the class where
3 they were declared */
4 publicpublicMethod () {
5 this.protectedMethod()
6 this.privateMethod()
7 }
8 protectedprotectedMethod () {
9 this.publicMethod()
10 this.privateMethod()
11 }
12 privateprivateMethod () {
13 this.publicMethod();
14 this.protectedMethod()
15 }
16 }
17 class DerivedextendsBase {
18 foo () {
19 this.publicMethod() // OK
20 this.protectedMethod()// OK
21 this.privateMethod() // compile-time error:
22 // the private method is inaccessible
23 }
24 }

```
The transitive closure of a direct subclass relationship is the subclass relationship. ClassAcan be a subclass of classC
if:
```
- ClassAis the direct subclass ofC; or
- ClassAis a subclass of some classB, which is in turn a subclass ofC(i.e., the definition applies recursively).
ClassCis a _superclass_ of classAifAis its subclass.

### 9.3 Class Implementation Clause

```
A class can implement one or more interfaces. Interfaces to be implemented by a class are listed in theimplements
clause. Interfaces listed in this clause are direct superinterfaces of the class.
The syntax of class implementation clause is presented below:
```
```
implementsClause:
'implements' interfaceTypeList
;
```
```
interfaceTypeList:
typeReference(','typeReference)*
;
```
```
A compile-time error occurs iftypeReferencefails to name an accessible interface type (see Accessible ).
```
```
170 Chapter 9. Classes
```

1 // File1
2 interface I { }// Not exported
3
4 // File2
5 import {I}from"File1"
6 class Cimplements I {}
7 // Compile-time error I is not accessible

```
If some interface is repeated as a direct superinterface in a singleimplementsclause (even if that interface is named
differently), then all repetitions are ignored.
For the class declarationC<F 1 ,..., Fn> (𝑛≥ 0 ,𝐶̸=𝑂𝑏𝑗𝑒𝑐𝑡):
```
- _Direct superinterfaces_ of class typeC<F 1 ,..., Fn> are the types specified in theimplementsclause of the
    declaration ofC(if there is animplementsclause).
For the generic class declarationC<F 1 ,..., Fn> ( _n_ > _0_ ):
- _Direct superinterfaces_ of the parameterized class typeC<T 1 ,..., Tn> are all typesI<U 1 𝜃,..., Uk𝜃> if:
**-** Ti( 1 ≤𝑖≤𝑛) is a type;
**-** I<U 1 ,..., Uk> is the direct superinterface ofC<F 1 ,..., Fn>; and
**-** 𝜃is the substitution [F 1 := T 1 ,..., Fn:= Tn].
Interface typeIis a superinterface of class typeCifIis one of the following:
- Direct superinterface ofC;
- Superinterface ofJwhich is in turn a direct superinterface ofC(see _Superinterfaces and Subinterfaces_ that
defines superinterface of an interface); or
- Superinterface of the direct superclass ofC.
A class _implements_ all its superinterfaces.
A compile-time error occurs if a class implements two interface types that represent different instantiations of the same
generic interface (see _Generics_ ).
If a class is not declared _abstract_ , then:
- Any abstract method of each direct superinterface is implemented (see _Inheritance_ ) by a declaration in that class.
- The declaration of an existing method is inherited from a direct superclass, or a direct superinterface.
A compile-time error occurs if a class field has the same name as a method from one of superinterfaces implemented
by the class, except when one is static and the other is not.

#### 9.3.1 Implementing Interface Methods

```
If superinterfaces have more then one default implementations (see Default Interface Method Declarations ) for some
methodm, then:
```
- The class that implements these interfaces has method that overridesm(see _Override-Compatible Signatures_ );
    or
- There is a single interface method with default implementation that overrides all other methods; or

```
9.3. Class Implementation Clause 171
```

- All interface methods refer to the same implementation, and this default implementation is the current class
    method.
Otherwise, a compile-time error occurs.

1 interface I1 { foo () {} }
2 interface I2 { foo () {} }
3 class C1implements I1, I2 {
4 foo () {}// foo() from C1 overrides both foo() from I1 and foo() from I2
5 }
6
7 class C2implements I1, I2 {
8 // Compile-time error as foo() from I1 and foo() from I2 have different␣
˓→implementations
9 }
10
11 interface I3extendsI1 {}
12 interface I4extendsI1 {}
13 class C3implements I3, I4 {
14 // OK, as foo() from I3 and foo() from I4 refer to the same implementation
15 }
16
17 interface I5extendsI1 { foo() {} } // override method from I1
18 class C4implements I1, I5 {
19 // Compile-time error as foo() from I1 and foo() from I5 have different␣
˓→implementations
20 }
21
22 class Base {}
23 class DerivedextendsBase {}
24
25 interface IBase {
26 foo(p: Base) {}
27 }
28 interface IDerived {
29 foo(p: Derived) {}
30 }
31 class Cimplements IBase, IDerived {}// foo() from IBase overrides foo() from IDerived
32 newC().foo(newBase)// foo() from IBase is called

```
A single method declaration in a class is allowed to implement methods of one or more superinterfaces.
```
#### 9.3.2 Implementing Required Interface Properties

```
A class must implement all required properties from all superinterfaces (see Interface Properties ) that can be defined
in a form of a field or as a getter, a setter, or both. In any case implementation may be provided in a form of field or
accessors.
The following table summarizes all valid variants of implementation, and a compile-time error occurs for any other
combinations:
```
```
172 Chapter 9. Classes
```

```
Form of Interface Property Implementation in a Class
readonly field readonly field, field, getter, or getter and setter
getter only readonly field, field, getter, or getter and setter
field field, or getter and setter
getter and setter field, or getter and setter
setter only field, setter, or setter and getter
```
```
Providing implementation for the property in the form of a field is not necessary:
```
1 interface Style {
2 get color():string
3 set color(s:string)
4 }
5
6 class StyleClassOneimplementsStyle {
7 color:string = ""
8 }
9
10 class StyleClassTwoimplementsStyle {
11 privatecolor_:string = ""
12
13 get color():string{
14 return this.color_
15 }
16
17 set color(s:string) {
18 this.color_ = s
19 }
20 }

```
If a property is implemented as a field, then any required accessors and a private hidden field are defined implicitly.
Entities forStyleClassOneare implicitly defined as follows:
```
```
1 class StyleClassOneimplementsStyle {
2 private$$_color:string= "" // the exact name of the field is implementation␣
˓→specific
3 get color():string {return this.$$_color }
4 set color(s:string) {this.$$_color = s }
5 }
```
```
If a property is defined in a form that requires a setter, then the implementation of the property in the form of areadonly
field causes a compile-time error:
```
```
1 interface Style {
2 set color(s:string)
3 writable:number
4 }
5
6 class StyleClassTwoimplementsStyle {
7 readonlycolor:string = ""// compile-time error
8 readonlywritable:number = 0 // compile-time error
9 }
(continues on next page)
```
```
9.3. Class Implementation Clause 173
```

```
(continued from previous page)
```
10
11 functionwrite_into_read_only (s:Style) {
12 s.color = "Black"
13 s.writable = 42
14 }
15
16 write_into_read_only (newStyleClassTwo)

```
If a property is defined in thereadonlyform, then the implementation of the property can either keep thereadonly
form or extend it to a writable form as follows:
```
1 interface Style {
2 get color():string
3 readonlyreadable:number
4 }
5
6 class StyleClassThreeimplements Style {
7 get color():string{ return"Black" }
8 set color(s:string) {}// OK!
9 readable:number = 0 // OK!
10 }
11
12 functionhow_to_write (s:Style) {
13 s.color = "Black"// compile-time error
14 s.readable = 42// compile-time error
15 if (sinstanceof StyleClassThree) {
16 lets1 = s asStyleClassThree
17 s1.color = "Black" // OK!
18 s1.readable = 42// OK!
19 }
20 }
21
22 how_to_write (newStyleClassThree)

#### 9.3.3 Implementing Optional Interface Properties

```
A class can implement Optional Interface Properties ) from superinterfaces or use implicitly defined accessors from an
interface.
The use of accessors implicitly defined in the interface is represented in the example below:
```
```
1 interface I {
2 n?:number
3 }
4 class Cimplements I {}
5
6 letc =newC()
7 console.log(c.n)// Output: undefined
8 c.n = 1// runtime error is thrown
```
```
174 Chapter 9. Classes
```

```
The implementation of an optional interface property as a field is represented in the example below:
```
```
1 interface I {
2 num?:number
3 }
4 class Cimplements I {
5 num?:number = 42
6 }
```
```
For the example above, the private hidden field and the required accessors are defined implicitly for the classCoverriding
accessors from the interface:
```
```
1 class Cimplements I {
2 private$$_num:number = 42// the exact name of the field is implementation specific
3 get num():number |undefined {return this.$$_num }
4 set num(n:number |undefined) {this.$$_num = n }
5 }
```
```
If a property is implemented by accessors (see Class Accessor Declarations ), then it is acceptable to implement only
one accessor for an optional field, and use default implementation for another accessor as represented in the following
example:
```
1 interface I {
2 num?:number
3 }
4
5 class C1implements I {// OK, both default implementations
6 }
7
8 class C2implements I {// OK, default implementation used for get
9 set num(n:number |undefined) {this.$$_num = n }
10 }
11
12 class C3implements I {// OK, both explicit implementations
13 get num():number |undefined {return this.$$_num }
14 set num(n:number |undefined) {this.$$_num = n }
15 }

```
A compile-time error occurs, if an optional property in an interface is implemented as non-optional field:
```
```
1 interface I {
2 num?:number
3 }
4 class Cimplements I {
5 num:number= 42 // compile-time error, must be optional
6 }
```
```
9.3. Class Implementation Clause 175
```

### 9.4 Class Members

A class can contain declarations of the following members:

- Fields,
- Methods,
- Accessors,
- Constructors,
- Method overloads (see _Class Method Overload Declarations_ ),
- Constructor overloads (see _Constructor Overload Declarations_ ), and
- Single static block for initialization (see _Static Initialization_ ).

The syntax is presented below:

classMembers:
'{'
classMember* staticBlock? classMember*
'}'
;

classMember:
annotationUsage?
accessModifier?
(constructorDeclaration
|overloadConstructorDeclaration
|classFieldDeclaration
|classMethodDeclaration
|overloadMethodDeclaration
|classAccessorDeclaration
)
;

staticBlock:'static'Block;

Declarations can be inherited or immediately declared in a class. Any declaration within a class has a class scope. The
class scope is fully defined in _Scopes_.

Members can be static or non-static as follows:

- Static members that are not part of class instances, and can be accessed by using a qualified name notation (see
    _Names_ ) anywhere the class name is accessible (see _Accessible_ ); and
- Non-static, or instance members that belong to any instance of the class.

Names of all static and non-static entities in a class declaration scope (see _Scopes_ ) must be unique, i.e., fields, methods,
and overloads with the same static or non-static status cannot have the same name.

The use of annotations is discussed in _Using Annotations_.

Class members are as follows:

**176 Chapter 9. Classes**


- Members inherited from their direct superclass (see _Inheritance_ ), except classObjectthat cannot have a direct
    superclass.
- Members declared in a direct superinterface (see _Superinterfaces and Subinterfaces_ ).
- Members declared in the class body (see _Class Members_ ).

Class members declaredprivateare not accessible (see _Accessible_ ) to all subclasses of the current class.

Class members declaredprotectedorpublicare inherited by all subclasses of the class and accessible (see _Acces-
sible_ ) for all subclasses.

Constructors and static block are not members, and are not inherited.

Members can be as follows:

- Class fields (see _Field Declarations_ ),
- Methods (see _Method Declarations_ ), and
- Accessors (see _Class Accessor Declarations_ ).

A _method_ is defined by the following:

1. _Type parameter_ , i.e., the declaration of any type parameter of the method member.
2. _Argument type_ , i.e., the list of types of arguments applicable to the method member.
3. _Return type_ , i.e., the return type of the method member.

### 9.5 Access Modifiers

Access modifiers define how a class member or a constructor can be accessed. Accessibility in ArkTS can be of the
following kinds:

- Private,
- Protected,
- Public.

The desired accessibility of class members and constructors can be explicitly specified by the corresponding _access
modifiers_.

The syntax of _class members or constructors modifiers_ is presented below:

accessModifier:
'private'
|'protected'
|'public'
;

If no explicit modifier is provided, then a class member or a constructor is implicitly consideredpublicby default.

**9.5. Access Modifiers 177**


#### 9.5.1 Private Access Modifier

```
The modifierprivateindicates that a class member or a constructor is accessible (see Accessible ) within its declaring
class, i.e., a private member or constructor m declared in some classCcan be accessed only within the class body ofC:
```
1 class C {
2 privatecount:number
3 getCount():number {
4 return this.count// ok
5 }
6 }
7
8 functionincrement(c:C) {
9 c.count++// compile-time error - 'count'is private
10 }

#### 9.5.2 Protected Access Modifier

```
The modifierprotectedindicates that a class member or a constructor is accessible (see Accessible ) only within its
declaring class and the classes derived from that declaring class. A protected memberMdeclared in some classCcan
be accessed only within the class body ofCor of a class derived fromC:
```
1 class C {
2 protectedcount: number
3 getCount():number{
4 return this.count // ok
5 }
6 }
7
8 class DextendsC {
9 increment() {
10 this.count++// ok, D is derived from C
11 }
12 }
13
14 functionincrement(c:C) {
15 c.count++// compile-time error - 'count'is not accessible
16 }

#### 9.5.3 Public Access Modifier

```
The modifierpublicindicates that a class member or a constructor can be accessed everywhere, provided that the
member or the constructor belongs to a type that is also accessible (see Accessible ).
```
```
178 Chapter 9. Classes
```

### 9.6 Field Declarations

_Field declarations_ represent data members in class instances or static data members (see _Static and Instance Fields_ ).
Class instance _field declarations_ are its _own fields_ in contrast to the inherited ones. Syntactically, a field declaration is
similar to a variable declaration.

classFieldDeclaration:
fieldModifier*
identifier
('?'? ':'type initializer?
|'?'? initializer
|'!' ':'type
)
;

fieldModifier:
'static'|'readonly'|'override'
;

A field with an identifier marked with ‘?’ is called _optional field_ (see _Optional Fields_ ). A field with an identifier marked
with ‘!’ is called _field with late initialization_ (see _Fields with Late Initialization_ ).

A compile-time error occurs if:

- Some field modifier is used more than once in a field declaration.
- Name of a field declared in the body of a class declaration is also used for a method of this class with the same
    static or non-static status.
- Name of a field declared in the body of a class declaration is also used for another field in the same declaration
    with the same static or non-static status.

Any static field can be accessed only with the qualification of a superclass name (see _Field Access Expression_ ).

A class can inherit more than one field or property with the same name from its superinterfaces, or from both its
superclass (see _Inheritance_ ) and superinterfaces (see _Interface Inheritance_. However, an attempt to refer to such a field
or property by its simple name within the class body causes a compile-time error.

The same field or property declaration can be inherited from an interface in more than one way. In that case, the field
or property is considered to be inherited only once.

#### 9.6.1 Static and Instance Fields

There are two categories of class fields as follows:

- Static fields
    Static fields are declared with the modifierstatic. A static field is not part of a class instance. There is one
    copy of a static field irrespective of how many instances of the class (even if zero) are eventually created.
    Static fields are always accessed by using a qualified name notation wherever the class name is accessible (see
    _Accessible_ ).
- Instance, or non-static fields

**9.6. Field Declarations 179**


```
Instance fields belong to each instance of the class. An instance field is created for, and associated with a newly-
created instance of a class, or of its superclass. An instance field is accessible (see Accessible ) via the instance
name.
```
#### 9.6.2 Readonly (Constant) Fields

```
A field with the modifierreadonlyis a readonly field. Changing the value of a readonly field after initialization is not
allowed. Both static and non-static fields can be declared readonly fields.
```
#### 9.6.3 Optional Fields

```
Optional field f?: T = expreffectively means that the type off``is ``T | undefined. If an initializer is absent
in a field declaration , then the default valueundefined(see Default Values for Types ) is used as the initial value of
the field.
For example, the following two fields are actually defined the same way:
```
1 class C {
2 f?:string
3 g: string| undefined= undefined
4 }

#### 9.6.4 Field Initialization

```
All fields except Fields with Late Initialization are initialized by using the default value (see Default Values for Types )
or a field initializer (see below). Otherwise, the field can be initialized in one of the following:
```
- Initializer block of a static field (see _Static Initialization_ ), or
- Class constructor of a non-static field (see _Constructor Declaration_ ).
_Field initializer_ is an expression that is evaluated at compile time or runtime. The result of successful evaluation is
assigned into the field. The semantics of field initializers is therefore similar to that of assignments (see _Assignment_ ).
Each initializer expression evaluation and the subsequent assignment are only performed once.
Readonlyfields initialization never uses default values (see _Default Values for Types_ ).
The initializer of a non-static field declaration is evaluated at runtime. The assignment is performed each time an
instance of the class is created.
The instance field initializer expression cannot use the following directly in any form:
- super; or
- this.

```
180 Chapter 9. Classes
```

```
If the initializer expression contains one of the above patterns, then a compile-time error occurs.
If allowed in the code, the above restrictions can break the consistency of class instances as shown in the following
examples:
```
1 class C {
2 a =this // Compile-time error
3
4 f1 =this.foo() // Compile-time error as'this'method is invoked
5
6 f2 = "a string field"
7
8 foo ():string {
9 // Type safety requires fields to be initialized before access
10 console.log (this.f1,this.f2)
11 return this.f2
12 }
13
14 }
15
16 class B {}
17 functionfoo (f: () => B) { returnf() }
18 class A {
19 field1 = foo(() => this.field2) // Compile-time error as this is used in the␣
˓→initializer code
20 field2 =newB
21 }

#### 9.6.5 Fields with Late Initialization

```
Field with late initialization must be an instance field. If it is defined asstatic, then a compile-time error occurs.
Field with late initialization cannot be of a nullish type (see Nullish Types ). Otherwise, a compile-time error occurs.
As all other fields, a field with late initialization must be initialized before it is used for the first time. However, this field
can be initialized later and not within a class declaration. Initialization of this field can be performed in a constructor
(see Constructor Declaration ), although it is not mandatory.
Field with late initialization cannot have field initializers or be an optional field (see Optional Fields ). Field with late
initialization must be initialized explicitly, even though its type has a default value.
The fact of initialization of field with late initialization is checked when the field value is read. The check is normally
performed at runtime. If the compiler identifies an error situation, then the error is reported at compile time:
```
```
1 class C {
2 f!:string
3 }
4
5 letx =newC()
6 x.f = "aa"
7 console.log(x.f)// ok
8
(continues on next page)
```
```
9.6. Field Declarations 181
```

(continued from previous page)
9 lety =newC()
10 console.log(y.f)// runtime or compile-time error

```
Note. Access to a field with late initialization in most cases is less performant then access to other fields.
TypeScript uses the term definite assignment assertion for the notion similar to late initialization. However, ArkTS
uses stricter rules.
```
#### 9.6.6 Overriding Fields

```
When extending a class or implementing interfaces, a field declared in a superclass or a superinterface can be overridden
by a field with the same name, and the samestaticor non-staticmodifier status. Using the keywordoverrideis
not required. The new declaration acts as redeclaration.
A compile-time error occurs if:
```
- Field marked with the modifieroverridedoes not override a field from a superclass.
- Field declaration contains the modifierstaticalong with the modifieroverride.
- Types of the overriding field and of the overridden field are different.

```
1 class C {
2 field: number= 1
3 }
4 class DextendsC {
5 field: string= "aa" // compile-time error: type is not the same
6 overrideno_field = 1224 // compile-time error: no overridden field in the base␣
˓→class
7 static override field:string = "aa" // compile-time error: static cannot override
8 }
```
```
Initializers of overridden fields are preserved for execution, and the initialization is normally performed in the context
of superclass constructors.
```
1 class C {
2 field: number= this.init()
3 privateinit() {
4 console.log ("Field initialization in C")
5 return 123
6 }
7 }
8 class DextendsC {
9 overridefield: number= 123 // field can be explicitly marked as overridden
10 }
11
12 class DerivedextendsD {
13 field =this.init_in_derived()
14 privateinit_in_derived() {
15 console.log ("Field initialization in Derived")
16 return 42
(continues on next page)

```
182 Chapter 9. Classes
```

```
(continued from previous page)
```
17 }
18 }
19 newDerived()
20 /* Output:
21 Field initialization in C
22 Field initialization in Derived
23 */

```
A compile-time error occurs if a field is not declared asreadonlyin a superclass, while an overriding field is marked
asreadonly:
```
```
1 class C {
2 field = 1
3 }
4 class DextendsC {
5 readonlyfield = 2 // compile-time error, wrong overriding
6 }
```
```
A compile-time error occurs if a field overrides getter or setter in a superclass:
```
```
1 class C {
2 get num(): number{ return42 }
3 set num(x: number) {}
4 }
5 class DextendsC {
6 num:number = 2// compile-time error, wrong overriding
7 }
```
```
The same compile-time error occurs in more complex case, where a field simultaneously overrides a field from a
superclass and implements a property from a superinterface:
```
```
1 class C {
2 num:number = 1
3 }
4 interface I {
5 num:number
6 }
7 class DextendsCimplements I {
8 num:number = 2// compile-time error, conflict in overriding
9 }
```
```
The overriding conflict occurs asnuminD, and must be both:
```
- Field to override a field inherited from the superclassC; and
- Two accessors (see _Class Accessor Declarations_ ) to implement a property from the superinterface ‘I’ (see _Im-_
    _plementing Required Interface Properties_ ).
Overriding a field by an accessor also causes a compile-time error as follows:

```
1 class C {
2 num:number = 1
3 }
4 class DextendsC {
(continues on next page)
```
```
9.6. Field Declarations 183
```

```
(continued from previous page)
```
5 get num(): number{ return42 } // compile-time error, wrong overriding
6 set num(x: number) {} // compile-time error, wrong overriding
7 }

### 9.7 Method Declarations

```
Methods declare executable code that can be called.
The syntax of class method declarations is presented below:
```
```
classMethodDeclaration:
methodModifier* identifier typeParameters? signature block?
;
```
```
methodModifier:
'abstract'
|'static'
|'final'
|'override'
|'native'
|'async'
;
```
```
The identifier in a class method declaration defines the method name that can be used to refer to a method (see Method
Call Expression ).
Methods with thefinalmodifier is an experimental feature discussed in detail in Final Methods.
A compile-time error occurs if:
```
- Method modifier appears more than once in a method declaration;
- Body of a class declaration declares a method but the name of that method is already used for a field in the same
    declaration.
A non-static method declared in a class can do the following:
- Implement a method inherited from a superinterface or superinterfaces (see _Implementing Interface Methods_ );
- Override a method inherited from a superclass (see _Overriding in Classes_ );
- Act as method declaration of a new method.
A static method declared in a class can do the following:
- Shadow a static method inherited from a superclass (see _Static Methods_ );
- Act as method declaration of a new static method.

```
184 Chapter 9. Classes
```

#### 9.7.1 Static Methods

```
A method declared in a class with the modifierstaticis a static method.
A compile-time error occurs if:
```
- The method declaration contains another modifier (abstract,final, oroverride) along with the modifier
    static.
- The header or body of a class method includes the name of a type parameter of the surrounding declaration.
Static methods are always called without reference to a particular object. As a result, a compile-time error occurs if the
keywordsthisorsuperare used inside a static method.
Static methods can be inherited from a superclass or shadowed by name regardless of the their signature:

1 class Base {
2 static foo() { console.log ("static foo() from Base") }
3 static bar() { console.log ("static foo() from Base") }
4 }
5
6 class DerivedextendsBase {
7 static foo(p:string) { console.log ("static foo() from Derived") }
8 }
9
10 Base.foo() // Output: static foo() from Base
11 Base.bar() // Output: static foo() from Base
12 Derived.bar() // Output: static foo() from Base, bar() is inherited
13 Derived.foo("a string") // Output: static foo() from Derived, foo() is shadowed
14 Derived.foo() // compile-time error as foo() in Derived has shadowed Base.
˓→foo()

```
Note: class static methods may access protected or private members of the same class type or derived one represented
as parameters or local variables:
```
1 class C {
2 protectedcount1:number
3 private count2:number
4 staticgetCount(c:C): number {
5 const local_c =newC
6 return c.count1 + c.count2 + local_c.count1 + local_c.count2// OK
7 }
8 statichandleDerived (b:B) {
9 b.count1 + b.count2// OK
10 }
11 }
12 class BextendsC {
13 staticdealWithProtected (b:B) {
14 b.count1// OK
15 b.count2// compile-time error
16 }
17 }
18
19 C.getCount (newC) // will return the sum of counts
20 C.handleDerived (newB) // will work with protected and private fields

```
9.7. Method Declarations 185
```

#### 9.7.2 Instance Methods

```
A method that is not declared static is called non-static method , or instance method.
An instance method is always called with respect to an object that becomes the current object which the keywordthis
refers to during the execution of the method body.
```
#### 9.7.3 Abstract Methods

```
An abstract method declaration introduces the method as a member along with its signature but without implementa-
tion. An abstract method is declared with the modifierabstractin the declaration.
Non-abstract methods can be referred to as concrete methods.
A compile-time error occurs if:
```
- An abstract method is declared private.
- The method declaration contains another modifier (static,final,native, orasync) along with the modifier
    abstract.
- The declaration of an abstract method _m_ does not appear directly within abstract classA.
- Any non-abstract subclass ofA(see _Abstract Classes_ ) does not provide implementation for _m_.
An abstract method declaration provided by an abstract subclass can override another abstract method. An abstract
method can also override non-abstract methods inherited from base classes or base interfaces as follows:

1 class C {
2 foo() {}
3 }
4 interface I {
5 foo() {}// default implementation
6 }
7 abstract class XextendsCimplements I {
8 abstractfoo(): void/* Here abstract foo() overrides both foo()
9 coming from class C and interface I */
10 }

#### 9.7.4 Async Methods

```
Async methods are discussed in Async Methods.
```
```
186 Chapter 9. Classes
```

#### 9.7.5 Overriding Methods

Theoverridemodifier indicates that an instance method in a superclass is overridden by the corresponding instance
method from a subclass (see _Overriding_ ).

The usage of the modifieroverrideis optional but strongly recommended as it makes the overriding explicit.

A compile-time error occurs if:

- Method marked with the modifieroverrideoverrides no method from a superclass.
- Method declaration contains modifierstaticalong with the modifieroverride.

If the signature of an overridden method contains parameters with default values (see _Optional Parameters_ ), then
the overriding method must always use the same default parameter values for the overridden method. Otherwise, a
compile-time error occurs.

More details on overriding are provided in _Overriding in Classes_ and _Overriding and Overloading in Interfaces_.

#### 9.7.6 Native Methods

Native methods are discussed in _Native Methods_.

#### 9.7.7 Method Body

_Method body_ is a block of code that implements a method. A semicolon or an empty body (i.e., no body at all) indicate
the absence of implementation.

An abstract or native method must have an empty body.

In particular, a compile-time error occurs if:

- The body of an abstract or native method declaration is a block.
- The method declaration is neither abstract nor native, but its body is either empty or a semicolon.

The rules that apply to return statements in a method body are discussed in _return Statements_.

A compile-time error occurs if a method is declared to have a return type, but its body can complete normally (see
_Normal and Abrupt Statement Execution_ ).

#### 9.7.8 Methods Returningthis

A return type of an instance method can bethis. It means that the return type is the class type to which the method
belongs. It is the only place where the keywordthiscan be used as type annotation (see _Signatures_ and _Return Type_ ).

The only result that is allowed to be returned from such a method isthis:

**9.7. Method Declarations 187**


```
1 class C {
2 foo(): this{
3 return this
4 }
5 }
```
```
The return type of an overridden method in a subclass must also bethis:
```
1 class C {
2 foo(): this{
3 return this
4 }
5 }
6
7 class DextendsC {
8 foo(): this{
9 return this
10 }
11 }
12
13 letx =newC().foo()// type of 'x'is 'C'
14 lety =newD().foo()// type of 'y'is 'D'

```
Otherwise, a compile-time error occurs.
```
### 9.8 Class Accessor Declarations

```
Class accessors are often used instead of fields to add additional control for operations of getting or setting a field value.
An accessor can be either a getter or a setter.
The syntax of class accessor declarations is presented below:
```
```
classAccessorDeclaration:
classAccessorModifier*
('get' identifier'(' ')'returnType? block?
|'set' identifier'('parameter ')'block?
)
;
```
```
classAccessorModifier:
'abstract'
|'static'
|'final'
|'override'
|'native'
;
```
```
Accessor modifiers are a subset of method modifiers. The allowed accessor modifiers have exactly the same meaning as
the corresponding method modifiers (see Abstract Methods for the modifierabstract, Static Methods for the modifier
```
```
188 Chapter 9. Classes
```

```
static, Final Methods for the modifierfinal, Overriding Methods for the modifieroverride, and Native Methods
for the modifiernative).
```
```
1 class Person {
2 private_age:number = 0
3 get age():number {return this._age }
4 set age(a:number) {
5 if (a < 0) {throw new Error("wrong age") }
6 this._age = a
7 }
8 }
```
```
A get-accessor ( getter ) must have an explicit return type and no parameters, or no return type at all on condition it can
be inferred from the getter body. A set-accessor ( setter ) must have a single parameter and no return type. The use of
getters and setters looks the same as the use of fields. A compile-time error occurs if:
```
- Getters or setters are used as methods;
- Getter return type cannot be inferred from the getter body;
- _Set-accessor_ ( _setter_ ) has a single parameter that is optional (see _Optional Parameters_ ):

1 class Person {
2 private_age:number = 0
3 get age():number {return this._age }
4 set age(a:number) {
5 if (a < 0) {throw new Error("wrong age") }
6 this._age = a
7 }
8 }
9
10 letp =newPerson()
11 p.age = 25 // setter is called
12 if (p.age > 30) { // getter is called
13 // do something
14 }
15 p.age(17) // Compile-time error: setter is used as a method
16 letx = p.age()// Compile-time error: getter is used as a method
17
18 class X {
19 set x (p?: Object) {}// Compile-time error: setter has optional parameter
20 }

```
If a getter has no return type specified, then the type is inferred as in Return Type Inference.
```
```
1 class Person {
2 private_age:number = 0
3 get age() {return this._age }// return type is inferred as number
4 }
```
```
A class can define a getter, a setter, or both with the same name. If both a getter and a setter with a particular name are
defined, then both must have the same accessor modifiers. Otherwise, a compile-time error occurs.
Accessors can be implemented by using a private field or fields to store the data as in the example above.
```
```
9.8. Class Accessor Declarations 189
```

```
1 class Person {
2 name:string = ""
3 surname:string= ""
4 get fullName():string {
5 return this.surname + " " +this.name
6 }
7 }
8 console.log (newPerson().fullName)
```
```
A name of an accessor cannot be the same as that of a non-static field, or of a method of class or interface. Otherwise,
a compile-time error occurs:
```
```
1 class Person {
2 name:string = ""
3 get name():string { // Compile-time error: getter name clashes with the field name
4 return this.name
5 }
6 set name(a_name:string) {// Compile-time error: setter name clashes with the field␣
˓→name
7 this.name = a_name
8 }
9 }
```
```
In the process of inheriting and overriding (see Overriding ), accessors behave as methods. The getter parameter type fol-
lows the covariance pattern, and the setter parameter type follows the contravariance pattern (see Override-Compatible
Signatures ):
```
1 class Base {
2 get field(): Base {return new Base }
3 set field(a_field:Derived) {}
4 }
5 class DerivedextendsBase {
6 overrideget field(): Derived {return new Derived }
7 overrideset field(a_field:Base) {}
8 }
9 functionfoo (base: Base) {
10 base.field =newDerived// setter is called
11 letb: Base= base.field// getter is called
12 }
13 foo (newDerived)

### 9.9 Constructor Declaration

```
Constructors are used to initialize objects that are instances of a class. A constructor declaration starts with the keyword
constructor, and has optional name. In any other syntactical aspect, a constructor declaration is similar to a method
declaration with no return type:
```
```
190 Chapter 9. Classes
```

```
constructorDeclaration:
'native'?'constructor'identifier?parameters constructorBody?
;
```
```
An optional identifier in constructor declaration is an experimental feature discussed in Constructor Names. Construc-
tors are called by the following:
```
- Class instance creation expressions (see _New Expressions_ ); and
- Explicit constructor calls from other constructors (see _Constructor Body_ ).
Access to constructors is governed by access modifiers (see _Access Modifiers_ and _Scopes_ ). Declaring a constructor
inaccessible prevents class instantiation from using this constructor. If the only constructor is declared inaccessible,
then no class instance can be created.
Anativeconstructor (an experimental feature described in _Native Constructors_ ) must have no _constructorBody_. Oth-
erwise, a compile-time error occurs.
A non-nativeconstructor must have _constructorBody_. Otherwise, a compile-time error occurs.
A compile-time error occurs if more then one non-nativeanonymous constructors are defined in a class:

1 class C {
2 constructor(s:string) {}
3 constructor() {}// compile-time error: multiple anonymous constructors
4 }

#### 9.9.1 Formal Parameters

```
The syntax and semantics of a constructor’s formal parameters are identical to those of a method.
```
#### 9.9.2 Constructor Body

```
Constructor body is a block of code that implements a constructor.
The syntax of constructor body is presented below:
```
```
constructorBody:
'{' statement*'}'
;
```
```
The constructor body must provide correct initialization of new class instances. Constructors have two variations:
```
- _Primary constructor_ that initializes instance own fields directly;
- _Secondary constructor_ that uses another same-class constructor to initialize its instance fields.
The high-level sequence of a _primary constructor_ body includes the following:
1. Mandatory call to a superconstructor (see _Explicit Constructor Call_ ) if a class has an extension clause (see _Class
Extension Clause_ ) on all execution paths of the constructor body.

```
9.9. Constructor Declaration 191
```

2. Mandatory execution of field initializers (if any) in the order they appear in a class body implicitly added by the
    compiler.
3. Optional arbitrary code that avoids usage of non-initialized fields.
4. Optional code that ensures all object fields to be initialized.
5. Optional arbitrary code.
As step 4 above cannot be guaranted at compile time in all possible cases, the following strategy is to be taken:
- If the compiler can detect that a non-initialized field is accessed during compilation, then a compile-time error
occurs;
- Otherwise, it is a responsibility of the runtime system to detect such cases and handle them with a runtime
exception.

1 class Base {
2 x:Object
3 constructor() {
4 this.x =newObject// Base object is fully initialized
5 crash_this (this)
6 }
7 }
8 class Derived {
9 y:Object
10 constructor() {
11 super()// mandatory call to base class constructor
12 this.y =newObject
13 }
14 }
15 functioncrash_this (b: Base) {
16 if (binstanceof Derived) { // If b is of type Derived, then
17 console.log ((b asDerived).y)// Access y field of Derived object
18 // Depending on the compilation context, either the compiler reports
19 // a compile-time error, or the runtime system is to detect the case
20 }
21 }

```
The example below represents primary constructors :
```
1 class Point {
2 x:number
3 y:number
4 constructor(x:number, y:number) {
5 this.x = x
6 this.y = y
7 }
8 }
9
10 class ColoredPointextendsPoint {
11 static readonlyWHITE = 0
12 static readonlyBLACK = 1
13 color:number
14 constructor(x:number, y:number, color:number) {
15 super(x, y) // calls base class constructor
16 this.color = color
(continues on next page)

```
192 Chapter 9. Classes
```

```
(continued from previous page)
```
17 }
18 }

```
The high-level sequence of a secondary constructor body includes the following:
```
1. Call to another same-class constructor that uses the keywordthis(see _Explicit Constructor Call_ ) on all execution
    paths of the constructor body.
2. Optional arbitrary code.
The example below represents _primary_ and _secondary_ constructors:

1 class Point {
2 x:number
3 y:number
4 constructor(x:number, y:number) {
5 this.x = x
6 this.y = y
7 }
8 }
9
10 class ColoredPointextendsPoint {
11 static readonlyWHITE = 0
12 static readonlyBLACK = 1
13 color:number
14
15 // primary constructor:
16 constructor(x:number, y:number, color:number) {
17 super(x, y) // calls base class constructor as class has'extends'
18 this.color = color
19 }
20 // secondary constructor:
21 constructorzero(color:number) {
22 this(0, 0, color)
23 }
24 }

```
A compile-time error occurs if a constructor calls itself, directly or indirectly through a series of one or more explicit
constructor calls usingthis.
A constructor body looks like a method body (see Method Body ), except for the semantics as described above. Explicit
return of a value (see return Statements ) is prohibited. On the opposite, a constructor body can use a return statement
without an expression.
A constructor body can have no more than one call to the current class or direct superclass constructor. Otherwise, a
compile-time error occurs.
```
```
9.9. Constructor Declaration 193
```

#### 9.9.3 Explicit Constructor Call

```
There are two kinds of explicit constructor calls :
```
- _Superclass constructor calls_ (used to call a constructor from the direct superclass) that begin with the keyword
    super.
- _Other constructor calls_ that begin with the keywordthis(used to call another same-class constructor).
To call a named constructor ( _Constructor Names_ ), the name of the constructor must be provided while calling a super-
class or another same-class constructor.
A compile-time error occurs if arguments of an explicit constructor call refer to one of the following:
- Any non-static field or instance method; or
- thisorsuper.

1 // Class declarations without constructors
2 classBase {
3 constructor() {}
4 constructorbase() {}
5 }
6 classDerived1extendsBase {
7 constructor() {
8 super() // Call Base class constructor
9 }
10 }
11 classDerived2extendsBase {
12 constructor() {
13 super.base() // Call Base class named constructor
14 }
15 }
16 classDerived3extendsBase {
17 constructor() {
18 this.derived()// Call same class named constructor
19 }
20 constructorderived() {}
21 }

#### 9.9.4 Default Constructor

```
If a class contains no constructor declaration, then a default constructor is implicitly declared. This guarantees that
every class effectively has at least one constructor. The form of a default constructor is as follows:
```
- Default constructor has modifierpublic(see _Access Modifiers_ ).
- The default constructor body contains:
    **-** Call to a superclass constructor with no arguments except the primordial classObject. The default con-
       structor body for the primordial classObjectis empty.
    **-** Mandatory execution of field initializers (if any) in the order they appear in a class body.
A compile-time error occurs if a default constructor is implicit, but the superclass has no accessible constructor without
parameters (see _Accessible_ ).

```
194 Chapter 9. Classes
```

1 // Class declarations without constructors
2 classObj_no_ctor {}
3 classBase_no_ctor {}
4 classDerived_no_ctor extendsBase_no_ctor {}
5
6 // Class declarations with default constructors declared implicitly
7 classObj {
8 constructor() {} // Empty body - as there is no superclass
9 }
10 // Default constructors added
11 classBase { constructor() {super() } }
12 classDerivedextendsBase { constructor() { super() } }
13
14 // Example of an error case
15 classA {
16 private constructor () {}
17 }
18 classB0 extendsA {}// OK. No constructor in B
19 // During compilation of B
20 classB1 extendsA {
21 constructor() {// Default constructor added
22 // that leads to compile-time error
23 // as default constructor calls super()
24 // which is private and inaccessible
25 super()
26 }
27 }

### 9.10 Inheritance

```
ClassCinherits all accessible members from its direct superclass and direct superinterfaces (see Accessible ), and
optionally overrides or shadows some of the inherited members.
IfCis not abstract, then it must implement all inherited abstract methods. The method of each inherited abstract method
must be defined with override-compatible signatures (see Override-Compatible Signatures ).
Semantic checks for inherited method and accessors are described in Overriding in Classes.
Constructors from the direct superclass ofCare not subject of overriding because such constructors are not accessible
(see Accessible ) inCdirectly, and can only be called from a constructor ofC(see Constructor Body ).
```
```
9.10. Inheritance 195
```

**196 Chapter 9. Classes**


##### CHAPTER

### TEN

### INTERFACES

An interface declaration declares an _interface type_ , i.e., a reference type that:

- Includes properties and methods as its members;
- Has no instance variables (fields);
- Usually declares one or more methods;
- Allows otherwise unrelated classes to provide implementations for the methods, and so implement the interface.

Creating an instance of interface type is not possible.

An interface can be declared _direct extension_ of one or more other interfaces. If so, the interface inherits all members
from the interfaces it extends. Inherited members can be optionally overridden or hidden.

A class can be declared to _directly implement_ one or more interfaces. Any instance of a class implements all methods
specified by its interface(s). A class implements all interfaces that its direct superclasses and direct superinterfaces
implement. Interface inheritance allows objects to support common behaviors without sharing a superclass.

The value of a variable declared _interface type_ can be a reference to any instance of a class that implements the spec-
ified interface. However, it is not enough for a class to implement all methods of an interface. A class or one of its
superclasses must be actually declared to implement an interface. Otherwise, the class is not considered to implement
the interface.

The rules of subtyping are discussed in detail in _Subtyping for Non-Generic Classes and Interfaces_ and _Subtyping for
Generic Classes and Interfaces_.

### 10.1 Interface Declarations

_Interface declaration_ specifies a new named reference type.

The syntax of _interface declarations_ is presented below:

interfaceDeclaration:
'interface' identifier typeParameters?
interfaceExtendsClause?'{'interfaceMember*'}'
;

interfaceExtendsClause:
'extends'interfaceTypeList
;
(continues on next page)

##### 197


```
(continued from previous page)
```
interfaceTypeList:
typeReference(','typeReference)*
;

The _identifier_ in an interface declaration specifies the interface name.

An interface declaration withtypeParametersintroduces a new generic interface (see _Generics_ ).

The scope of an interface declaration is defined in _Scopes_.

### 10.2 Superinterfaces and Subinterfaces

An interface declared with anextendsclause extends all other named interfaces, and thus inherits all their members.
Such other named interfaces are _direct superinterfaces_ of a declared interface. A class that _implements_ the declared
interface also implements all interfaces that the interface _extends_.

A compile-time error occurs if:

- _typeReference`_ in theextendsclause refers directly to, or is an alias of non-interface type.
- Interface type named bytypeReferenceis not _Accessible_.
- Type arguments (see _Type Arguments_ ) oftypeReferencedenote a parameterized type that is not well-formed
    (see _Generic Instantiations_ ).
- Theextendsgraph has a cycle.

If an interface declaration (possibly generic)I<F 1 ,..., Fn> (𝑛≥ 0 ) contains anextendsclause, then the _direct
superinterfaces_ of the interface typeI<F 1 ,..., Fn> are the types given in theextendsclause of the declaration of
I.

All _direct superinterfaces_ of the parameterized interface typeI<T 1 ,..., Tn> are typesJ<U 1 𝜃,..., Uk𝜃>, if:

- Ti( 1 ≤𝑖≤𝑛) is the type of a generic interface declarationI<F 1 ,..., Fn> (𝑛 > 0 );
- J<U 1 ,..., Uk> is a direct superinterface ofI<F 1 ,..., Fn>; and
- 𝜃is a substitution [F 1 := T 1 ,..., Fn:= Tn].

The transitive closure of the direct superinterface relationship results in the _superinterface_ relationship.

Interface _I_ is a _subinterface_ of _K_ wherever _K_ is a superinterface of _I_. Interface _K_ is a superinterface of _I_ if:

- _I_ is a direct subinterface of _K_ ; or
- _K_ is a superinterface of some interface _J_ of which _I_ is, in turn, a subinterface.

There is no single interface to which all interfaces are extensions (unlike classObjectto which every class is an
extension).

A compile-time error occurs if an interface depends on itself.

If superinterfaces have default implementations (see _Default Interface Method Declarations_ ) for some methodm, then
the following occurs:

- Methodmwith an override-compatible signature (see _Override-Compatible Signatures_ ) declared within the cur-
    rent interface overrides all othermmethods inherited from superinterfaces; or

**198 Chapter 10. Interfaces**


- All methods inherited from superinterfaces refer to the same implementation, and this default implementation is
    the current interface method; or
- One methodmin some superinterface overrides all other methods from other superinterfaces.
Otherwise, a compile-time error occurs.

1 interface I1 { foo () {} }
2 interface I2 { foo () {} }
3 interface II1extendsI1, I2 {
4 foo () {}// foo() from II1 overrides both foo() from I1 and foo() from I2
5 }
6 interface II2extendsI1, I2 {
7 // Compile-time error as foo() from I1 and foo() from I2 have different␣
˓→implementations
8 }
9 interface I3extendsI1 {}
10 interface I4extendsI1 {}
11 interface II3extendsI3, I4 {
12 // OK, as foo() from I3 and foo() from I4 refer to the same implementation
13 }
14
15 class Base {}
16 class DerivedextendsBase {}
17
18 interface II1 {
19 foo (p:Base) {}
20 }
21 interface II2 {
22 foo (p:Derived) {}
23 }
24 interface II3extendsII1, II2 {}
25 // foo() from II1 overrides foo() from II2

### 10.3 Interface Members

```
An interface declaration can contain interface members , i.e., its properties (see Interface Properties ) and methods (see
Interface Method Declarations ).
The syntax of interface member is presented below:
```
```
interfaceMember
:annotationUsage?
(interfaceProperty
|interfaceMethodDeclaration
|overloadInterfaceMethodDeclaration
)
;
```
```
The scope of declaration of a member m that the interface typeIdeclares or inherits is specified in Scopes.
```
```
10.3. Interface Members 199
```

```
The usage of annotations is discussed in Using Annotations.
Interface members include:
```
- Members declared explicitly in the interface declaration;
- Members inherited from a direct superinterface (see _Superinterfaces and Subinterfaces_ ).
A compile-time error occurs if the method explicitly declared by the interface has the same name as theObject’s
publicmethod.

1 interface I {
2 toString (p:number):void// Compile-time error
3 toString():string {return "some string" }// Compile-time error
4 }

```
An interface inherits all members of the interfaces it extends (see Interface Inheritance ).
A name in a declaration scope must be unique, i.e., the names of properties and methods of an interface type must not
be the same (see Interface Declarations ).
```
### 10.4 Interface Properties

```
Interface property can be defined in the form of a field or an accessor (a getter or a setter).
The syntax of interface property is presented below:
```
```
interfaceProperty:
'readonly'? identifier'?'?':'type
|'get' identifier'(' ')'returnType
|'set' identifier'('parameter ')'
;
```
```
An interface property is a required property (see Required Interface Properties ) if it is one of the following:
```
- Explicit _accessor_ , i.e., a getter or a setter; or
- Form of a field that has no ‘?’.
Otherwise, it is an _optional property_ (see _Optional Interface Properties_ ).
If ‘?’ is used after the name of the property, then the property type is semantically equivalent totype | undefined.

1 interface I {
2 property?: Type
3 }
4 // is the same as
5 interface I {
6 property:Type |undefined
7 }

```
200 Chapter 10. Interfaces
```

#### 10.4.1 Required Interface Properties

```
A required property defined in the form of a field implicitly defines the following:
```
- Getter, if the property is marked asreadonly;
- Otherwise, both a getter and a setter with the same name.
A type annotation for the field defines return type for the getter and type of parameter for the setter.
As a result, the following declarations have the same effect:

1 interface Style {
2 color: string
3 }
4 // is the same as
5 interface Style {
6 get color():string
7 set color(s:string)
8 }

```
Note. A required property defined in a form of accessors does not define any additional entities in the interface.
A class that implements an interface with properties can also use a field or an accessor notation (see Implementing
Required Interface Properties , Implementing Optional Interface Properties ).
```
#### 10.4.2 Optional Interface Properties

```
An optional property can be defined in two forms:
```
- Short formidentifier '?' ':' T; or
- Explicit formidentifier':' T | undefined.
In both cases,identifierhas effective typeT | undefined.
The _optional property_ implicitly defines the following:
- A getter (if the property is marked asreadonly);
- Otherwise, both a getter and a setter with the same name.
Accessors have implicitly defined bodies, in this aspect they are similar to _Default Interface Method Declarations_.
However, ArkTS does not support explicitly defined accessors with bodies.
The following declaration:

1 interface I {
2 num?: number
3 }

- implicitly declares two accessors:

1 interface I {
2 get num(): number| undefined{ return undefined}
3 set num(x: number| undefined) {throw newInvalidStoreAccessError }
4 }

```
10.4. Interface Properties 201
```

If the default setter is not overridden in a class that implements the interface,InvalidStoreAccessErroris thrown
at attempt to set value of an optional property. See also _Implementing Optional Interface Properties_.

### 10.5 Interface Method Declarations

An ordinary _interface method declaration_ specifies the method name and signature, and is called _abstract_. Its implicit
accessibility ispublic.

An interface method can have a body (see _Default Interface Method Declarations_ ) as an experimental feature.

The syntax of _interface method declaration_ is presented below:

interfaceMethodDeclaration:
identifier signature
|interfaceDefaultMethodDeclaration
;

### 10.6 Interface Inheritance

Interface _I_ inherits all properties and methods from its direct superinterfaces. Semantic checks are described in _Over-
riding and Overloading in Interfaces_.

**Note**. The semantic rules of methods apply to properties because any interface property implicitly defines a getter, a
setter, or both.

Private methods defined in superinterfaces are not accessible (see _Accessible_ ) in the interface body.

A compile-time error occurs if interface _I_ declares aprivatemethod _m_ with a signature compatible with the instance
method𝑚′(see _Override-Compatible Signatures_ ) that has any access modifier in the superinterface of _I_.

**202 Chapter 10. Interfaces**


##### CHAPTER

### ELEVEN

### ENUMERATIONS

```
Enumeration typeenumspecifies a distinct user-defined type with an associated set of named constants that define its
possible values.
The syntax of enumeration declaration is presented below:
```
```
enumDeclaration:
'const'?'enum' identifier(':' type)? '{'enumConstantList?'}'
;
```
```
enumConstantList:
enumConstant(','enumConstant)* ','?
;
```
```
enumConstant:
identifier('=' constantExpression)?
;
```
```
Typeconst enumis supported for source-level compatibility with TypeScript. The modifierconstis skipped as it
has no impact onenumsemantics in ArkTS.
Qualification by type is mandatory to access the enumeration constant, except enumeration constant initialization ex-
pressions:
```
1 enum Color { Red, Green, Blue }
2 letc: Color= Color.Red
3
4 enum Flags { Read, Write, ReadWrite = Read | Write }
5 // No need to use Flags.Read | Flags.Write in initialization

```
If enumeration type is exported, then all enumeration constants are exported along with the mandatory qualification.
For example, if Color is exported, then all constants likeColor.Redare exported along with the mandatory qualifica-
tionColor.
The value of an enum constant can be set as follows:
```
- Explicitly to a numeric constant expression (expression of typeintorlong) or to a constant expression of type
    string; or
- Implicitly by omitting the constant expression.
If constant expression is omitted, then the value of the enum constant is set implicitly to an integer value (see _Enumer-
ation Integer Values_ ).
A compile-time error occurs if integer orstringtype enumeration constants are combined in a single enumeration.

##### 203


```
A type to which all enumeration constant values belong is called enumeration base type. This type isint,longor
string.
Any enumeration constant is of typeenumeration. Implicit conversion (see Enumeration to Constants Type Conver-
sions ) of an enumeration constant to numeric types or typestringdepends on the type of constants.
In addition, all enumeration constant names must be unique. Otherwise, a compile-time error occurs.
```
1 enum E1 { A, B = "hello" } // compile-time error
2 enum E2 { A = 5, B = "hello" }// compile-time error
3 enum E3 { A = 5, A = 77 } // compile-time error
4 enum E4 { A = 5, B = 5 } // OK! values can be the same

```
Emptyenumis supported as a corner case for compatibility with TypeScript.
```
1 enum Empty {}// OK

### 11.1 Enumeration Integer Values

```
The integer value of anenumconstant is set implicitly if an enumeration constant specifies no value.
A constant expression of typeintorlongcan be used to set the value explicitly:
```
1 enum Background { White = 0xFF, Grey = 0x7F, Black = 0x00 }
2 enum LongEnum { A = 0x7FFF_FFFF_1, B, C }

```
Choosing which type to use—intorlong—is based on the same principle as for integer literals (see Integer Literals ).
If all constants have no value, then the first constant is assigned the value zero. The other constant is assigned the value
of the immediately preceding constant plus one.
If some but not all constants have their values set explicitly, then the values of the constants are set by the following
rules:
```
- The constant which is the first and has no explicit value gets zero value.
- Constant with an explicit value has that explicit value.
- Constant that is not the first and has no explicit value takes the value of the immediately preceding constant plus
    one.
In the example below, the value ofRedis 0, ofBlue, 5, and ofGreen, 6:

1 enum Color { Red, Blue = 5, Green }

### 11.2 Enumeration String Values

```
A string value for enumeration constants must be set explicitly:
```
```
204 Chapter 11. Enumerations
```

1 enum Commands { Open = "fopen", Close = "fclose" }

### 11.3 Enumeration Operations

```
The value of an enumeration constant can be converted to typestringby using the methodtoString:
```
1 enum Color { Red, Green = 10, Blue }
2 letc: Color= Color.Green
3 console.log(c.toString())// prints: 10

```
The name of enumeration type can be indexed by the value of this enumeration type to get the name of the constant:
```
1 enum Color { Red, Green = 10, Blue }
2 letc: Color= Color.Green
3 console.log(Color[c])// prints: Green

```
If several enumeration constants have the same value, then the textually last constant has the priority:
```
1 enum E { One = 1, one = 1, oNe = 1 }
2 console.log(E.fromValue (1)) // prints: oNe

```
Additional methods available for enumeration types and constants are discussed in Enumeration Methods in the chapter
Experimental Features.
```
```
11.3. Enumeration Operations 205
```

**206 Chapter 11. Enumerations**


##### CHAPTER

### TWELVE

### ERROR HANDLING

```
ArkTS is designed to provide first-class support in responding to, and recovering from different error situations in a
program. Normal program execution can be interrupted by the occurrence of situations of two kinds:
```
- Runtime errors (e.g., null pointer dereferencing, array bounds checking, or division by zero);
- Operation completion failures (e.g., the task of reading and processing data from a file on disk can fail if the file
    does not exist on a specified path, read permissions are not available, or else).
The term _error_ in this Specification denotes all kinds of error situations.

### 12.1 Errors

```
Error is the base class of all error situations. Defining a new error class is normally not required because essential
error classes for various cases (e.g.,RangeError) are defined in the standard library (see Standard Library ).
However, a developer can handle a new error situation by usingErrorclass itself, or by a subclass ofError. An
example of error handling is provided below:
```
1 classUnknownErrorextendsError {// User-defined error class
2 error:Error
3 constructor(error:Error) {
4 super()
5 this.error = error
6 }
7 }
8
9 functionget_array_element<T>(array: T[], index: number): T|null{
10 try{
11 returnarray[index]// access array
12 }
13 catch (error) {
14 if (errorinstanceof RangeError)// invalid index detected
15 return null
(continues on next page)

##### 207


```
(continued from previous page)
```
16 throw newUnknownError (error)// Unknown error occurred
17 }
18 }

```
In most cases, errors are raised by the ArkTS runtime system, or by the standard library (see Standard Library ) code.
New error situations can be created and raised bythrowstatements (see throw Statements ).
Errors are handled by usingtrystatements (see try Statements ).
Note. Not every error can be recovered.
```
1 functionhandleAll(
2 actions : () =>void,
3 handling_actions : () =>void)
4 {
5 try{
6 actions()
7 }
8 catch(x) {// Type of x is Error
9 handling_actions()
10 }
11 }

```
208 Chapter 12. Error Handling
```

##### CHAPTER

### THIRTEEN

### MODULES AND NAMESPACES

```
Programs in ArkTS are structured as sequences of elements ready for compilation called modules. Each module cre-
ates its own scope (see Scopes ). Variables, functions, classes, interfaces, or other declarations of a module are only
accessible (see Accessible ) within such a scope if not explicitly exported.
A variable, function, class, interface, or other declarations exported from a module must be imported first by the module
that is to use them.
All modules are stored in a file system or a database (see Modules in Host System ).
A module can optionally consist of the following four parts:
```
1. Import directives that enable referring imported declarations in a module;
2. Top-level declarations;
3. Top-level statements; and
4. Re-export directives.
The syntax of _module_ is presented below:

```
moduleDeclaration:
importDirective* (topDeclaration |topLevelStatements| exportDirective)*
;
```
```
Every module can directly use all exported entities from the standard library (see Standard Library Usage ).
```
1 // Hello, world! module
2 functionmain() {
3 console.log("Hello, world!")// console is defined in the standard library
4 }

```
If a module has at least one top-level ambient declaration (see Ambient Declarations ) then all other declarations must
be ambient as well and no top-level statements (see Top-Level Statements ). Otherwise, a compile-time error occurs.
```
1 declare letx: number
2 functionmain() {}
3 // compile-time error: ambient and non-ambient declarations are mixed

##### 209


### 13.1 Import Directives

_Import directives_ make entities exported from other modules (see _Modules and Namespaces_ ) available for use in the
current module by using different binding forms. These directives have no effect during the program execution.

An import declaration has the following two parts:

- Import path that determines from what module to import;
- Import bindings that define what entities, and in what form (either qualified or unqualified) the current module
    can use.

The syntax of _import directives_ is presented below:

importDirective:
'import' 'type'? bindings'from' importPath
;

bindings:
defaultBinding
| (defaultBinding',')?allBinding
| (defaultBinding',')?selectiveBindings
;

allBinding:
'*' bindingAlias
;

bindingAlias:
'as'identifier
;

defaultBinding:
identifier
;

selectiveBindings:
nameBinding(',' nameBinding)*
;

nameBinding:
identifier bindingAlias?
|'default' 'as' identifier
;

importPath:
StringLiteral
;

Each binding adds a declaration or declarations to the scope of a module (see _Scopes_ ). Any declaration added so must
be distinguishable in the declaration scope (see _Declarations_ ).

Import withtypemodifier is discussed in _Import Type Directive_.

A compile-time error occurs if:

- Declaration added to the scope of a module by a binding is not distinguishable;

**210 Chapter 13. Modules and Namespaces**


- Module imports itself directly:importPathrefers to the file in which the current module is stored; or

#### 13.1.1 Bind All with Qualified Access

```
Import binding* as Abinds the single named entity A to the declaration scope of the current module.
A qualified name consisting of A and the name of entityA.nameis used to access any entity exported from the module
as defined by the import path.
```
```
Import Usage
```
```
import *as Mathfrom"..." letx = Math.sin(1.0)
```
```
This form of import is recommended because it simplifies the reading and understanding of the source code when all
exported entities are prefixed with the name of the imported module.
```
#### 13.1.2 Default Import Binding

```
Default import binding allows importing a declaration exported from a module as default export. Knowing the actual
name of a declaration is not required as the new name is given at importing. A compile-time error occurs if another
form of import is used to import an entity initially exported as default.
There are two forms of default import binding :
```
- Single identifier;
- Special form of selective import with the keyworddefault.

1 import DefaultExportedItemBindedNamefrom".../someFile"
2 import {default as DefaultExportedItemNewName}from ".../someFile"
3 functionfoo () {
4 letv1 =newDefaultExportedItemBindedName()
5 // instance of class'SomeClass'to be created here
6 letv2 =newDefaultExportedItemNewName()
7 // instance of class'SomeClass'to be created here
8 }
9
10 // SomeFile
11 export default classSomeClass {}
12
13 // Or
14 class SomeClass {}
15 export default SomeClass

```
13.1. Import Directives 211
```

#### 13.1.3 Selective Binding

```
Selective binding allows to bind an entity exported as identifier , or an entity exported by default (see Default Import
Binding ).
Binding with identifier binds an exported entity with the name identifier to the declaration scope of the current module.
If no binding alias is present, then the entity is added to the declaration scope under the original name. Otherwise, the
identifier specified in binding alias is used. In the latter case, the bounded entity is no longer accessible (see Accessible )
under the original name.
If an identifier denotes an overload alias (see Function Overload Declarations ), then all its accessible overloaded
functions, either imported or not, are considered in the process of Overload Resolution for call validity.
```
1 // File1
2 export function f1(p:number) {}
3 export function f2(p:string) {}
4 export overload foo {f1, f2}
5
6 // File2
7 import {foo}from "File1" // Note: f1 and f2 are not mandatory imported
8 foo(5) // f1() is called
9 foo("a string") // f2() is called
10
11 // File3
12 import {foo, f1}from "File1" // Note: f1 is accessible as well
13 f1(5) // f1() is called
14 foo(6) // f1() is called
15 foo("a string") // f2() is called

```
Selective binding that uses exported entities is represented in the examples below:
```
```
1 export constPI = 3.14
2 export function sin(d: number): number{}
```
```
Note. The import path of the module is irrelevant and replaced for ‘...’ in the examples below:
```
```
Import Usage
```
```
import {sin}from"..." letx = sin(1.0)
letf: float= 1.0
```
```
import {sinasSine} from"
..."
```
```
letx = Sine(1.0)// OK
lety = sin(1.0)/* Error␣
˓→‘sin’
is not accessible */
```
```
A single import statement can list several names as follows:
```
```
212 Chapter 13. Modules and Namespaces
```

```
Import Usage
```
```
import {sin, PI}from"..." letx = sin(PI)
```
```
import {sinasSine, PI}␣
˓→from "
..."
```
```
letx = Sine(PI)
```
```
Complex cases with several bindings mixed on one import path are discussed below in Several Bindings for One Import
Path.
```
#### 13.1.4 Import Type Directive

```
An import directive can have atypemodifier exclusively for a better syntactic compatibility with TypeScript (also
see Export Type Directive ). ArkTS supports no additional semantic checks for entities imported by using import type
directives.
The semantic checks performed by the compiler in TypeScript but not in ArkTS are represented by the following code:
```
1 // File module.ets
2 console.log ("Module initialization code")
3
4 export classClass1 {/*body*/}
5
6 class Class2 {}
7 export type{Class2}
8
9 // MainProgram.ets
10
11 import {Class1}from"./module.ets"
12 import type{Class2}from"./module.ets"
13
14 letc1 =newClass1()// OK
15 letc2 =newClass2()// Compile-time error in Typescript, OK in ArkTS

#### 13.1.5 Import Path

```
Import path is a string literal that determines where and how an imported module is to be searched for.
Import path can include the following:
```
- Initial dot ‘.’ or two dots ‘..’ followed by the slash character ‘/’.

```
13.1. Import Directives 213
```

- One or more path components (the subset of characters and case sensitivity of path components must follow the
    path rules of a host filesystem).
- Slash characters separating components of the path.
The slash character ‘/’ is used in import paths irrespective of the host system. The backslash character is not used in
this context.
In most file systems, an import path looks like a file path. _Relative_ (see below) and _non-relative_ import paths have
different _resolutions_ that map the import path to a file path of the host system.
The compiler uses its own algorithm to locate a module source that processes the import path. If the import path
specifies no file extension, then the compiler can append some according to its own rules and priorities. If the import
path refers to a folder, then the way to handle the case is determined by the actual compiler. If the compiler cannot
locate a module source definitely, then a compile-time error occurs.
A _relative import path_ starts with ‘./’ or ‘../’. Examples of relative paths are presented below:

1 "./components/entry"
2 "../constants/http"

```
Resolving relative import is relative to the importing file. Relative import is used on modules to maintain their relative
location.
```
1 import *as Utilsfrom "./mytreeutils"

```
Other import paths are non-relative.
Resolving a non-relative path depends on the compilation environment. The definition of the compiler environment
can be particularly provided in a configuration file or environment variables.
The base URL setting is used to resolve a path that starts with ‘/’. Path mapping is used in all other cases. Resolution
details depend on the implementation. For example, the compilation configuration file can contain the following lines:
```
1 "baseUrl": "/home/project",
2 "paths": {
3 "std": "/arkts/stdlib"
4 }

```
In the example above,/net/httpis resolved to/home/project/net/http, andstd/components/treemapto
/arkts/stdlib/components/treemap.
File name, placement, and format are implementation-specific.
If the above configuration is in effect, the first path maps directly to filesystem after applyingbaseUrl, whilestdin
the second path is replaced for/arkts/stdlib. Examples of non-relative paths are presented below.
```
1 "/net/http"
2 "std/components/treemap"

```
214 Chapter 13. Modules and Namespaces
```

#### 13.1.6 Several Bindings for One Import Path

The same bound entities can use the following:

- Several import bindings,
- One import directive, or several import directives with the same import path:

```
In one import directive
import {sin, cos}from"..."
```
```
In several import directives
import {sin}from"..."
import {cos}from"..."
```
No conflict occurs in the above example, because the import bindings define disjoint sets of names.

The order of import bindings in an import declaration has no influence on the outcome of the import.

The rules below prescribe what names must be used to add bound entities to the declaration scope of the current module
if multiple bindings are applied to a single name:

**13.1. Import Directives 215**


```
Case Sample Rule
A name is explicitly used without an
alias in several bindings. import {sin, sin}
from"..."
```
```
OK. The compile-time warning is
recommended.
```
```
A name is used explicitly without
alias in one binding. import {sin}
from"..."
```
```
OK. No warning.
```
```
A name is explicitly used without
alias, and implicitly with alias. import {sin}
from"..."
```
```
import *as M
from"..."
```
```
OK. Both the name and qualified
name can be used:
sin and M.sin are accessible.
```
```
A name is explicitly used with alias.
import {sinasSine}
from "..."
```
```
OK. Only alias is accessible for the
name, but not the original name:
```
- Sine is accessible;
- sin is not accessible.

```
A name is explicitly used with alias,
and implicitly with alias. import {sinasSine}
from"..."
```
```
import *as M
from"..."
```
```
OK. Both options can be used:
```
- Sine is accessible;
- M.sin is accessible.

```
A name is explicitly used with alias
several times. import {sinasSine,
sinasSIN}
from"..."
```
```
OK. Both aliases are accessible. But
warning can be displayed.
```
### 13.2 Standard Library Usage

```
All entities exported from the standard library (see Standard Library ) are accessible as simple names (see Accessible )
in any module. Using these names as programmer-defined entities at module scope causes a compile-time error in
accordance to Declarations.
```
1 console.log("Hello, world!") // ok, 'console'is defined in the library
2
3 letconsole = 5// compile-time error

```
216 Chapter 13. Modules and Namespaces
```

### 13.3 Top-Level Declarations

```
Top-level declarations declare top-level types (class,interface, orenumsee Type Declarations ), top-level variables
(see Variable Declarations ), constants (see Constant Declarations ), functions (see Function Declarations , overloads
(see Overload Declarations ), namespaces (see Namespace Declarations ), or other declarations (see Ambient Declara-
tions , Annotations , Functions with Receiver , Accessors with Receiver ). Top-level declarations can be exported.
The syntax of top-level declarations is presented below:
```
```
topDeclaration:
('export' 'default'?)?
annotationUsage?
(typeDeclaration
|variableDeclarations
|constantDeclarations
|functionDeclaration
|overloadFunctionDeclaration
|namespaceDeclaration
|ambientDeclaration
|annotationDeclaration
|accessorDeclaration
|functionWithReceiverDeclaration
|accessorWithReceiverDeclaration
)
;
```
1 export let x:number[], y:number

```
The usage of annotations is discussed in Using Annotations.
```
#### 13.3.1 Exported Declarations

```
Top-level declarations can use export modifiers that make the declarations accessible (see Accessible ) in other modules
by using import (see Import Directives ). The same result may be achieved using export directive (see Export Directives )
for tne top-level declaration. The declarations which are not exported as mentioned above can be used only inside the
module they are declared in.
```
1 export classPoint {}
2 export let Origin =newPoint(0, 0)
3 export function Distance(p1:Point, p2:Point):number {
4 // ...
5 }

```
In addition, only one top-level declaration can be exported by using the default export directive. It allows specifying
no declared name when importing (see Default Import Binding for details). A compile-time error occurs if more than
one top-level declaration is marked asdefault.
```
1 export default let PI = 3.141592653589

```
Another supported form of export default is using an expression as export default target. This export directive effectively
means that an anonymous constant variable is created with a value equal to the value of the expression evaluation result.
```
```
13.3. Top-Level Declarations 217
```

```
The export can be imported only by providing a name for the constant variable that is exported by using this export
directive. Otherwise, a compile-time error occurs.
```
1 // File1
2 class A {
3 foo () {}
4 }
5 export default new A
6
7 // File2
8 import {default as a}from "File1"
9
10 a.foo() // Calling method foo() of class A where 'a'is an instance of type A
11 a =newA // Compile-time error as'a'is a constant variable
12
13 // File3
14 import *as afrom "File1"/* Compile-time error: such form of import
15 cannot be used for the default export */

```
If a function, a variable, a constant, or an accessor is exported, or an exported class field or method is public, then any
type declared in the current module and used in their declaration must be exported. Otherwise, a compile-time error
occurs.
```
```
1 // Module
2 export function foo (p:SomeType): SomeType { ... }// Type'SomeType'is not exported
3 export let v:SomeType // Type'SomeType'is not exported
4 export classSomeClass {
5 field: SomeType// Type'SomeType'is not exported
6 foo (p:SomeType): SomeType { ... }// Type'SomeType'is not exported
7 }
8 class SomeType {}
```
### 13.4 Namespace Declarations

```
Namespace declaration introduces the qualified name to be used as a qualifier for access to each exported entity of the
namespace.
The syntax of namespace declarations is presented below:
```
```
namespaceDeclaration:
'namespace' qualifiedName
'{' namespaceMember* staticBlock? namespaceMember* '}'
;
```
```
namespaceMember:
topDeclaration| exportDirective
;
```
```
Namespace can have an initializer block ( staticBlock in namespace declaration syntax above). The initializer block is
called only in case when at least one of exported namespace members is used in the program. It is guaranteed that its
```
```
218 Chapter 13. Modules and Namespaces
```

```
code is called before any use of namespace members (see Static Initialization for detail).
The usage of a namespace is represented in the example below:
```
1 namespace NS1 {
2 export function foo() { }
3 export let variable = 1234
4 export constconstant = 1234
5 export let someVar:string
6
7 // Will be called before any use of NS1 members
8 static {
9 someVar = "some string"
10 console.log("Init for NS1 done")
11 }
12 export function bar() {}
13 }
14
15 namespace NS2 {
16 export constconstant = 1
17 // Will never be called since NS2 members are never used
18 static {
19 console.log("Init for NS2 done")
20 }
21 export function bar() {}
22 }
23
24 export function bar() {} // That is a different bar()
25
26 if (NS1.variable == NS1.constant) {
27 NS1.variable = 4321
28 }
29 NS1.bar() // namespace bar() is called
30 bar() // top-level bar() is called

```
Note. An exported namespace entity can be used in the form of a qualifiedName outside a namespace in the same
module. Any namespace entity can be and typically is used inside a namespace without qualification, i.e., without a
namespace name. A qualifiedName inside a namespace can be used for a namespace entity only when the entity is
exported. Using a qualifiedName for non-exported entity both inside and outside a namespace causes a compile-time
error:
```
1 namespace NS {
2 export let a:number = 1
3 letb = 2
4
5 export function foo() {
6 letv: number
7 v = a// OK, no qualification
8 v = NS.a// OK, `a` exported
9 }
10
11 export function bar() {
12 letv: number
13 v = b // OK, no qualification
(continues on next page)

```
13.4. Namespace Declarations 219
```

```
(continued from previous page)
```
14 v = NS.b// CTE,`b`not exported
15 }
16 }
17
18 NS.a = 1// OK, `NS.a` exported
19 NS.b = 1// CTE,`NS.b` not exported

```
Note. A namespace must be exported to be used in another module:
```
1 // File1
2 namespace Space1 {
3 export function foo() { ... }
4 export let variable = 1234
5 export constconstant = 1234
6 }
7 export namespace Space2 {
8 export function foo(p:number) { ... }
9 export let variable = "1234"
10 }
11
12 // File2
13 import {Space2asSpace1}from "File1"
14
15 // compile-time error - there is no variable or constant called'constant'
16 if (Space1.variable == Space1.constant) {
17 // compile-time error - incorrect assignment as type 'number'
18 // is not compatible with type 'string'
19 Space1.variable = 4321
20 }
21 Space1.foo() // compile-time error - there is no function 'foo()'
22 Space1.foo(1234)// OK

```
Note. Embedded namespaces are allowed:
```
1 namespace ExternalSpace {
2 export function foo() { ... }
3 export let variable = 1234
4 export namespace EmbeddedSpace {
5 export constconstant = 1234
6 }
7 }
8
9 if (ExternalSpace.variable == ExternalSpace.EmbeddedSpace.constant) {
10 ExternalSpace.variable = 4321
11 }

```
Note. Namespaces with identical namespace names in a single module merge their exported declarations into a single
namespace. A duplication causes a compile-time error. Exported and non-exported declarations with the same name
are also considered a compile-time error. Only one of the merging namespaces can have an initializer. Otherwise, a
compile-time error occurs.
```
```
1 // One source file
2 namespace A {
(continues on next page)
```
```
220 Chapter 13. Modules and Namespaces
```

(continued from previous page)
3 export function foo() { console.log ("1st A.foo() exported") }
4 functionbar() { }
5 export namespace C {
6 export function too() { console.log ("1st A.C.too() exported") }
7 }
8 }
9
10 namespace B { }
11
12 namespace A {
13 export function goo() {
14 A.foo()// calls exported foo()
15 foo() /* calls exported foo() as well as all A namespace
16 declarations are merged into one */
17 A.C.moo()
18 }
19 //export function foo() { }
20 // Compile-time error as foo() was already defined
21
22 // function foo() { console.log ("2nd A.foo() non-exported") }
23 // Compile-time error as foo() was already defined as exported
24 }
25
26 namespace A.C {
27 export function moo() {
28 too()// too() accessible when namespace C and too() are both exported
29 A.C.too()
30
31 }
32 }
33
34 A.goo()
35
36 // File
37 namespace A {
38 export function foo() { ... }
39 export function bar() { ... }
40 }
41
42 namespace A {
43 functiongoo() { bar() } // exported bar() is accessible in the same namespace
44 export function foo() { ... } // Compile-time error as foo() was already defined
45 }
46
47 namespace X {
48 static {}
49 }
50 namespace X {
51 static {}// Compile-time error as only one initializer allowed
52 }

```
Note. A namespace name can be a qualified name. It is a shortcut notation of embedded namespaces as represented
below:
```
```
13.4. Namespace Declarations 221
```

```
1 namespace A.B {
2 /*some declarations*/
3 }
```
```
The code above is the shortcut to the following code:
```
```
1 namespace A {
2 export namespace B {
3 /*some declarations*/
4 }
5 }
```
```
This code illustrates the usage of declarations in the following case:
```
```
1 namespace A.B.C {
2 export function foo() { ... }
3 }
4
5 A.B.C.foo()// Valid function call, as'B'and'C'are implicitly exported
```
```
If an ambient namespace (see Ambient Namespace Declarations ) defined in a module (see Modules and Namespaces ),
then all ambient namespace declarations are accessible across all declarations and top-level statements of the module.
```
1 declarenamespace A {
2 functionfoo(): void
3 typeX = Array<number>
4 }
5
6 A.foo()// Valid function call, as'foo'is accessible for top-level statements
7 functionfoo () {
8 A.foo()// Valid function call, as'foo'is accessible here as well
9 }
10 class C {
11 method () {
12 A.foo()// Valid function call, as'foo'is accessible here too
13 letx: A.X= []// Type A.X can be used
14 }
15 }

### 13.5 Export Directives

```
Export directive allows the following:
```
- Specifying a selective list of exported declarations with optional renaming;
- Specifying a name of one declaration;
- Exporting a type; or
- Re-exporting declarations from other modules.
The syntax of an _export directive_ is presented below:

```
222 Chapter 13. Modules and Namespaces
```

```
exportDirective:
selectiveExportDirective
|singleExportDirective
|exportTypeDirective
|reExportDirective
;
```
#### 13.5.1 Selective Export Directive

```
Top-level declarations can be made exported by using a selective export directive. The selective export directive pro-
vides an explicit list of names of the declarations to be exported. Optional renaming allows having the declarations
exported with new names.
The syntax of selective export directive is presented below:
```
```
selectiveExportDirective:
'export'selectiveBindings
;
```
```
A selective export directive uses the same selective bindings as an import directive:
```
1 export { d1, d2as d3}

```
The above directive exports ‘d1’ by its name, and ‘d2’ as ‘d3’. The name ‘d2’ is not accessible (see Accessible ) in the
modules that import this module.
```
#### 13.5.2 Single Export Directive

```
Single export directive allows specifying the declaration to be exported from the current module by using the declara-
tion’s own name, or anonymously.
The syntax of single export directive is presented below:
```
```
singleExportDirective:
'export'
(identifier
|'default' (expression| identifier)
|'{'identifier 'as' 'default' '}'
)
;
```
```
Ifdefaultis present, then only one such export directive is possible in the current module. Otherwise, a compile-time
error occurs.
The directive in the example below exports variable ‘v’ by its name:
```
```
13.5. Export Directives 223
```

1 export v
2 letv = 1

```
The directive in the example below exports class ‘A’ by its name as default export:
```
1 class A {}
2 export default A
3 export {A as default}// such syntax is also acceptable

```
The directive in the example below exports a constant variable anonymously:
```
1 class A {}
2 export default new A

```
Single export directive acts as re-export when the declaration referred to by identifier is imported.
```
1 import {v}from"some location"
2 export v

#### 13.5.3 Export Type Directive

```
An export directive can have atypemodifier exclusively for a better syntactic compatibility with TypeScript (also see
Import Type Directive ).
The export type directive syntax is presented below:
```
```
exportTypeDirective:
'export' 'type' selectiveBindings
;
```
```
ArkTS supports no additional semantic checks for entities exported by using export type directives.
```
#### 13.5.4 Re-Export Directive

```
In addition to exporting what is declared in the module, it is possible to re-export declarations that are part of other
modules’ export. A particular declaration or all declarations can be re-exported from a module. When re-exporting,
new names can be given. This action is similar to importing but has the opposite direction.
The syntax of re-export directive is presented below:
```
```
reExportDirective:
'export'
('*'bindingAlias?
|selectiveBindings
|'{' 'default' bindingAlias? '}'
)
(continues on next page)
```
```
224 Chapter 13. Modules and Namespaces
```

```
(continued from previous page)
'from' importPath
;
```
```
AnimportPathcannot refer to the file the current module is stored in. Otherwise, a compile-time error occurs.
If re-exported declarations are not distinguishable (see Declarations ) within the scope of the current module, then a
compile-time error occurs.
The re-exporting practices are represented in the following examples:
```
1 export *from "path_to_the_module"// re-export all exported declarations
2 export *as qualifierfrom "path_to_the_module"
3 // re-export all exported declarations with qualification
4 export { d1, d2as d3}from"path_to_the_module"
5 // re-export particular declarations some under new name
6 export {default}from "path_to_the_module"
7 // re-export default declaration from the other module
8 export {default as name}from"path_to_the_module"
9 // re-export default declaration from the other module under'name'

### 13.6 Top-Level Statements

```
A module can contain sequences of statements that logically comprise one sequence of statements.
The syntax of top-level statements is presented below:
```
```
topLevelStatements:
statement*
;
```
```
A module can contain any number of top-level statements that logically merge into a single sequence in the textual
order:
```
1 statements_1
2 /* top-declarations except constant and variable declarations */
3 statements_2

```
The sequence above is equal to the following:
```
1 /* top-declarations except constant and variable declarations */
2 statements_1; statements_2

```
This situation is represented by the example below:
```
1 // The actual text combination of the statements and declarations
2 console.log ("Start of top-level statements")
3 typeA =number |string
4 leta: A= 56
5 functionfoo () {
6 console.log (a)
(continues on next page)

```
13.6. Top-Level Statements 225
```

(continued from previous page)
7 }
8 a = "a string"
9
10
11 // The logically ordered text - declarations then statements
12 typeA =number |string
13 functionfoo () {
14 console.log (a)
15 }
16 console.log ("Start of top-level statements")
17 leta: A= 56
18 a = "a string"

- If a module is imported by some other module, then the semantics of top-level statements is to initialize the
    imported module. It means that all top-level statements are executed only once before a call to any other function,
    or before the access to any top-level variable of the module.
- If a module is used as a program, then top-level statements are used as a program entry point (see _Program Entry_
    _Point_ ). The set of top-level statements being empty implies that the program entry point is also empty and does
    nothing. If a module has themainfunction, then it is executed after the execution of the top-level statements.

1 // Source file A
2 {// Block form
3 console.log ("A.top-level statements")
4 }
5
6 // Source file B
7 import* asA from"Source file A "
8 functionmain () {
9 console.log ("B.main")
10 }

```
The output is as follows:
A. Top-level statements,
B. Main.
```
```
1 // One source file
2 console.log ("A.Top-level statements")
3 functionmain () {
4 console.log ("B.main")
5 }
```
```
A compile-time error occurs if top-level statements contain a return statement ( Expression Statements ).
The execution of top-level statements means that all statements, except type declarations, are executed one after another
in the textual order of their appearance within the module until an error situation is thrown (see Errors ), or last statement
is executed.
```
```
226 Chapter 13. Modules and Namespaces
```

### 13.7 Program Entry Point

```
Modules can act as programs (applications). Program execution starts from the execution of a program entry point
which can be of the following two kinds:
```
- Top-level statements for modules (see _Top-Level Statements_ ); or
- Entry point function (see below).
A module can have the following forms of entry point:
- Sole entry point function (mainor other as described below);
- Sole top-level statement (the first statement in the top-level statements acts as the entry point);
- Both top-level statement and entry point function (same as above, plus the function called after the top-level
statement execution is completed).
Entry point functions have the following features:
- Any exported top-level function can be used as an entry point. An entry point is selected by the compiler, the
execution environment, or both;
- Entry point function must either have no parameters, or have one parameter of typestring[]that provides
access to the arguments of a program command line;
- Entry point function return type is eithervoid(see _Type void_ ) orint;
- Entry point function cannot have overloading;
- Entry point function is calledmainby default.
The example below represents different forms of valid and invalid entry points:

1 functionmain() {
2 // Option 1: a return type is inferred from the body of main().
3 // It will be'int'if the body has 'return'with the integer expression
4 // and'void'if no return at all in the body
5 }
6
7 functionmain():void{
8 // Option 2: explicit :void - no return in the function body required
9 }
10
11 functionmain():int{
12 // Option 3: explicit :int - return is required
13 return 0
14 }
15
16 functionmain():string {// compile-time error: incorrect main signature
17 return""
18 }
19
20 functionmain(p:number) {// compile-time error: incorrect main signature
21 }
22
23 // Option 4: top-level statement is the entry point
24 console.log ("Hello, world!")
25
26 // Option 5: top-level exported function
(continues on next page)

```
13.7. Program Entry Point 227
```

```
(continued from previous page)
```
27 export function entry() {}
28
29 // Option 5: top-level exported function with command-line arguments
30 export function entry(cmdLine:string[]) {}

```
228 Chapter 13. Modules and Namespaces
```

##### CHAPTER

### FOURTEEN

### AMBIENT DECLARATIONS

```
Ambient declaration specifies an entity that is declared elsewhere. Ambient declarations:
```
- Provide type information for entities included into a program from external sources.
- Introduce no new entities like regular declarations do.
- Cannot include executable code, and thus have no initializers.
Ambient functions, methods, and constructors have no bodies.
The syntax of _ambient declaration_ is presented below:

```
ambientDeclaration:
'declare'
(ambientConstantDeclaration
|ambientFunctionDeclaration
|overloadFunctionDeclaration
|ambientClassDeclaration
|ambientInterfaceDeclaration
|ambientNamespaceDeclaration
|ambientAnnotationDeclaration
|ambientAccessorDeclaration
|'const'?enumDeclaration
|typeAlias
)
;
```
```
An ambient enumeration type declaration can be prefixed by the keywordconstfor TypeScript compatibility. It has
no influence on the declared type.
A compile-time error occurs if the modifierdeclareis used in a context that is already ambient:
```
1 declarenamespace A{
2 declare functionfoo(): void// compile-time error
3 }

```
A compile-time warning occurs if an ambient declaration is marked withexportkeyword as all ambient declarations
are exported by default:
```
1 export declare namespace A{// compile-time warning
2 functionfoo(): void
3 }

##### 229


### 14.1 Ambient Constant Declarations

```
The syntax of ambient constant declaration is presented below:
```
```
ambientConstantDeclaration:
'const' ambientConstList';'
;
```
```
ambientConstList:
ambientConst(','ambientConst)*
;
```
```
ambientConst:
identifier((':' type) | ('='␣
˓→(IntegerLiteral|FloatLiteral|StringLiteral|MultilineStringLiteral)))
;
```
```
An initializer expression for an ambient constant must be a numeric or string literal. The meaning of the literal is to
define the type of the ambient constant, while the actual value must be provided when a non-ambient declaration is
available.
```
### 14.2 Ambient Function Declarations

```
The syntax of ambient function declaration is presented below:
```
```
ambientFunctionDeclaration:
'function'identifier
typeParameters? signature
;
```
```
A compile-time error occurs if explicit return type for an ambient function declaration is not specified.
```
1 declare functionfoo(x: number): void// ok
2 declare functionbar(x: number) // compile-time error

```
Ambient functions cannot have parameters with default values but can have optional parameters.
Ambient function declarations cannot specify function bodies.
```
1 declare functionfoo(x?:string): void// ok
2 declare functionbar(y: number= 1): void// compile-time error

```
Note. The modifierasynccannot be used in an ambient context.
```
```
230 Chapter 14. Ambient Declarations
```

### 14.3 Ambient Overload Function Declarations

```
The syntax of ambient overload function declaration is identical to that of Function Overload Declarations. The
semantics of such declarations is defined by the same rules.
```
1 // Top-level functions are overloaded
2 declare functionfoo1(p:string): void
3 declare functionfoo2(p:number): void
4 declareoverload foo {foo1, foo2}
5
6 // Namespace functions are overloaded
7 declarenamespace N {
8 functionfoo1(p:string): void
9 functionfoo2(p:number): void
10 overload foo {foo1, foo2}
11 }
12
13 // All calls are valid
14 foo("a string")
15 foo(5)
16 N.foo("a string")
17 N.foo(5)

### 14.4 Ambient Class Declarations

```
The syntax of ambient class declaration is presented below:
```
```
ambientClassDeclaration:
'class'|'struct' identifier typeParameters?
classExtendsClause? implementsClause?
'{' ambientClassMember*'}'
;
```
```
ambientClassMember:
ambientAccessModifier?
(ambientFieldDeclaration
|ambientConstructorDeclaration
|ambientMethodDeclaration
|overloadMethodDeclaration
|ambientClassAccessorDeclaration
|ambientIndexerDeclaration
|ambientCallSignatureDeclaration
|ambientIterableDeclaration
)
;
```
```
ambientAccessModifier:
'public'|'protected'
;
```
```
14.3. Ambient Overload Function Declarations 231
```

```
Ambient field declarations have no initializers.
The syntax of ambient field declaration is presented below:
```
```
ambientFieldDeclaration:
ambientFieldModifier*identifier ':'type
;
```
```
ambientFieldModifier:
'static'|'readonly'
;
```
```
Ambient constructor, method, and accessor declarations have no bodies.
Their syntax is presented below:
```
```
ambientConstructorDeclaration:
'constructor'parameters
;
```
```
ambientMethodDeclaration:
ambientMethodModifier*identifier signature
;
```
```
ambientMethodModifier:
'static'
;
```
```
ambientClassAccessorDeclaration:
ambientMethodModifier*
('get' identifier'(' ')'returnType
|'set' identifier'('parameter ')'
)
;
```
```
Ambient methods can be overloaded similarly to non-ambient methods with the same syntax and semantics (see Class
Method Overload Declarations ).
```
1 // Class methods are overloaded
2 declare classA {
3 foo1(p:string): void
4 foo2(p:number): void
5 overload foo {foo1, foo2}
6 }
7
8 // All methods calls are valid
9 functiondemo (a:A) {
10 a.foo("a string")
11 a.foo(5)
12 }

```
232 Chapter 14. Ambient Declarations
```

#### 14.4.1 Ambient Indexer

```
Ambient indexer declarations specify the indexing of a class instance in an ambient context. The feature is provided
for TypeScript compatibility:
The syntax of ambient indexer declaration is presented below:
```
```
ambientIndexerDeclaration:
'readonly'? '[' identifier':'indexType']'returnType
;
indexType: 'number';
```
```
The following restriction applies: Only one ambient indexer declaration is allowed in an ambient class declaration.
```
1 declare classC {
2 [index:number]: number
3 }

```
Note. Ambient indexer declaration is supported in ambient contexts only. If written in ArkTS, ambient class imple-
mentation must conform to Indexable Types.
```
#### 14.4.2 Ambient Call Signature

```
Ambient call signature declarations are used to specify callable types in an ambient context. The feature is provided
for TypeScript compatibility:
The syntax of ambient call signature declaration is presented below:
```
```
ambientCallSignatureDeclaration:
signature
;
```
1 declare classC {
2 (someArg:number): boolean
3 }

```
Note. Ambient class signature declaration is supported in ambient contexts only. If written in ArkTS, ambient class
implementation must conform to Callable Types with $_invoke Method.
```
#### 14.4.3 Ambient Iterable

```
Ambient iterable declaration indicates that a class instance is iterable in an ambient context. The feature is provided
for TypeScript compatibility:
The syntax of ambient iterable declaration is presented below:
```
```
ambientIterableDeclaration:
'[Symbol.iterator]' '(' ')'returnType
;
```
```
14.4. Ambient Class Declarations 233
```

```
The following restrictions apply:
```
- _returnType_ must be a type that implementsIteratorinterface defined in _Standard Library_.
- Only one _ambient iterable declaration_ is allowed in an ambient class declaration.

```
1 declare classC {
2 [Symbol.iterator] (): CIterator
3 }
```
```
Note. Ambient iterable declaration is supported in ambient contexts only. If written in ArkTS, ambient class imple-
mentation must conform to Iterable Types.
```
### 14.5 Ambient Interface Declarations

```
The syntax of ambient interface declaration is presented below:
```
```
ambientInterfaceDeclaration:
'interface' identifier typeParameters?
interfaceExtendsClause?
'{' ambientInterfaceMember*'}'
;
```
```
ambientInterfaceMember
:interfaceProperty
|ambientInterfaceMethodDeclaration
|ambientIndexerDeclaration
|ambientIterableDeclaration
;
```
```
ambientInterfaceMethodDeclaration:
'default'?identifier signature
;
```
```
Ambient interface can contain additional members in the same manner as an ambient class (see Ambient Indexer , and
Ambient Iterable ).
If an interface method declaration is marked with the keyworddefault, then a non-ambient interface must contain
the default implementation for the method as follows:
```
1 declare interfaceI1 {
2 defaultfoo ():void// method foo will have the default implementation
3 }
4 class C1implements I1 {}// Class C1 is valid as foo() has the default implementation
5
6 interface I1 {
7 // If such interface is used as I1 it will be runtime error as there is
8 // no default implementation for foo()
9 foo ():void
10 }
11
(continues on next page)

```
234 Chapter 14. Ambient Declarations
```

```
(continued from previous page)
```
12 declare interfaceI2 {
13 foo ():void// method foo has no default implementation
14 }
15 class C2implements I2 {}// Class C2 is invalid as foo() has no implementation
16 class C3implements I2 { foo() {} } // Class C3 is valid as foo() has implementation

### 14.6 Ambient Namespace Declarations

```
Namespaces are used to logically group multiple entities. ArkTS supports ambient namespaces for better TypeScript
compatibility. TypeScript often uses ambient namespaces to specify the platform API or a third-party library API.
The syntax of ambient namespace declaration is presented below:
```
```
ambientNamespaceDeclaration:
'namespace' identifier'{'ambientNamespaceElement* '}'
;
```
```
ambientNamespaceElement:
ambientNamespaceElementDeclaration|exportDirective
;
```
```
ambientNamespaceElementDeclaration:
'export'?
(ambientConstantDeclaration
|ambientFunctionDeclaration
|ambientClassDeclaration
|ambientInterfaceDeclaration
|ambientNamespaceDeclaration
|ambientAccessorDeclaration
|'const'?enumDeclaration
|typeAlias
)
;
```
```
An enumeration type declaration can be prefixed with the keywordconstfor TypeScript compatibility. The prefix has
no influence on the declared type. Only exported entities can be accessed outside a namespace.
Namespaces can be nested:
```
```
1 declarenamespace A {
2 export namespace B {
3 export function foo():void;
4 }
5 }
```
```
A namespace is not an object but merely a scope for entities that can be accessed by using qualified names only.
If an ambient namespace is imported from a module, then all ambient namespace declarations are accessible (see
Accessible ) across all declarations and top-level statements of the current module.
```
```
14.6. Ambient Namespace Declarations 235
```

1 // File1.d.ets
2 export declare namespace A {// namespace itself must be exported
3 functionfoo(): void
4 typeX = Array<number>
5 }
6
7 // File2.ets
8 import {A}from'File1.d.ets'
9
10 A.foo()// Valid function call, as'foo'is accessible for top-level statements
11 functionfoo () {
12 A.foo()// Valid function call, as'foo'is accessible here as well
13 }
14 class C {
15 method () {
16 A.foo()// Valid function call, as'foo'is accessible here too
17 letx: A.X= []// Type A.X can be used
18 }
19 }

```
A compile-time error occurs if an ambient namespace declaration contains an exportDirective that refers to a declaration
which is not a part of the namespace.
```
```
1 export declare namespace A {
2 export {foo}// compile-time error: no'foo'in namespace'A'
3 }
4 functionfoo() {}
```
#### 14.6.1 Implementing Ambient Namespace Declaration

```
If an ambient namespace is implemented in ArkTS, a namespace with the same name must be declared (see Namespace
Declarations ) as the top-level declaration of a module. All namespace names of a nested namespace (i.e. a namespace
embedded into another namespace) must be the same as in ambient context.
```
```
236 Chapter 14. Ambient Declarations
```

##### CHAPTER

### FIFTEEN

### SEMANTIC RULES

```
This Chapter contains semantic rules to be used throughout this Specification document. The description of the rules
is more or less informal. Some details are omitted to simplify the understanding.
```
### 15.1 Semantic Essentials

```
The section gives a brief introduction to the major semantic terms and their usage in several contexts.
```
#### 15.1.1 Type of Standalone Expression

```
Standalone expression (see Type of Expression ) is an expression for which there is no target type in the context where
the expression is used.
The type of a standalone expression is determined as follows:
```
- In case of _Numeric Literals_ , the type is the default type of a literal:
    **-** Type of _Integer Literals_ isintorlong;
    **-** Type of _Floating-Point Literals_ isdoubleorfloat.
- In case of _Constant Expressions_ , the type is inferred from operand types and operations.
- In case of an _Array Literal_ , the type is inferred from the elements (see _Array Type Inference from Types of_
    _Elements_ ).
- Otherwise, a compile-time error occurs. Specifically, a compile-time error occurs if an _object literal_ is used as a
    _standalone expression_.
The situation is represented in the example below:

1 functionfoo() {
2 1 // type is'int'
3 1.0 // type is'number'
4 [1.0, 2.0] // type is number[]
5 [1, "aa"]// type is (int | string)
6 }

##### 237


#### 15.1.2 Specifics of Assignment-like Contexts

```
Assignment-like context (see Assignment-like Contexts ) can be considered as an assignmentx = expr, wherexis a
left-hand-side expression, andexpris a right-hand-side expression. E.g., there is an implicit assignment ofexprto
the formal parameterfooin the callfoo(expr), and implicit assignments to elements or properties in Array Literal
and Object Literal.
Assignment-like context is specific in that the type of a left-hand-side expression is known, but the type of a right-hand-
side expression is not necessarily known in the context as follows:
```
- If the type of a right-hand-side expression is known from the expression itself, then the _Assignability_ check is
    performed as in the example below:

1 functionfoo(x: string, y:string) {
2 x = y // ok, assignability is checked
3 }

- Otherwise, an attempt is made to apply the type of the left-hand-side expression to the right-hand-side expression.
    A compile-time error occurs if the attempt fails as in the example below:

1 functionfoo(x: int, y:double[]) {
2 x = 1 // ok, type of' 1 'is inferred from type of'x'
3 y = [1, 2] // ok, array literal is evaluated as [1.0, 2.0]
4 }

#### 15.1.3 Specifics of Variable Initialization Context

```
If the variable or a constant declaration (see Variable and Constant Declarations ) has an explicit type annotation, then
the same rules as for assignment-like contexts apply. Otherwise, there are two cases forlet x = expr(see Type
Inference from Initializer ) as follows:
```
- The type of the right-hand-side expression is known from the expression itself, then this type becomes the type
    of the variable as in the example below:

1 functionfoo(x: int) {
2 lety = x// type of 'y'is 'int'
3 }

- Otherwise, the type ofexpris evaluated as type of a standalone expression as in the example below:

1 functionfoo() {
2 letx = 1// x is of type 'int'(default type of' 1 ')
3 lety = [1, 2] // x is of type'number[]'
4 }

```
238 Chapter 15. Semantic Rules
```

#### 15.1.4 Specifics of Numeric Operator Contexts

```
Thepostfixandprefix incrementanddecrementoperators evaluatebyteandshortoperands without widening.
It is also true for anassignmentoperator (consideringassignmentas a binary operator).
For other numeric operators, the operands of unary and binary numeric expressions are widened to a larger numeric
type. The minimum type isint. None of those operators evaluates values of typesbyteandshortwithout widening.
Details of specific operators are discussed in corresponding sections of the Specification.
```
#### 15.1.5 Specifics of String Operator Contexts

```
If one operand of the binary operator ‘ + ’ is of typestring, then the string conversion applies to another non-string
operand to convert it to string (see String Concatenation and String Operator Contexts ).
```
#### 15.1.6 Other Contexts

```
The only semantic rule for all other contexts, and specifically for Overriding , is to use Subtyping.
```
#### 15.1.7 Specifics of Type Parameters

```
If the type of a left-hand-side expression in assignment-like context is a type parameter, then it provides no additional
information for type inference even where a type parameter constraint is set.
If the target type of an expression is a type parameter , then the type of the expression is inferred as the type of a
standalone expression.
The semantics is represented in the example below:
```
1 class C<Textends number> {
2 constructor(x:T) {}
3 }
4
5 newC(1)// compile-time error

```
The type of ‘ 1 ’ in the example above is inferred asint(default type of an integer literal). The expression is considered
new C<int>(1)and causes a compile-time error becauseintis not a subtype ofnumber(type parameter constraint).
Explicit type argumentnew C<number>(1)must be used to fix the code.
```
```
15.1. Semantic Essentials 239
```

#### 15.1.8 Semantic Essentials Summary

```
Major semantic terms are listed below:
```
- _Type of Expression_ ;
- _Assignment-like Contexts_ ;
- _Type Inference from Initializer_ ;
- _Numeric Operator Contexts_ ;
- _String Operator Contexts_ ;
- _Subtyping_ ;
- _Assignability_ ;
- _Overriding_ ;
- _Overloading_ ;
- _Type Inference_.

### 15.2 Subtyping

```
Subtype relationship between typesSandT, whereSis a subtype ofT(recorded asS<:T), means that any object of
typeScan be safely used in any context to replace an object of typeT. The opposite relation (recorded asT:>S) is
called supertype relationship. Each type is its own subtype and supertype (S<:SandS:>S).
By the definition ofS<:T, typeTbelongs to the set of supertypes of typeS. The set of supertypes includes all direct
supertypes (discussed in subsections), and all their respective supertypes. More formally speaking, the set is obtained
by reflexive and transitive closure over the direct supertype relation.
The terms subclass , subinterface , superclass , and superinterface are used in the following sections as synonyms for
subtype and supertype when considering non-generic classes, generic classes, or interface types.
If a relationship of two types is not described in one of the following sections, then the types are not related to each
other. Specifically, two Resizable Array Types and two Tuple Types are not related to each other, except where they are
identical (see Type Identity ).
```
1 class Base {}
2 class DerivedextendsBase {}
3
4 functionnot_a_subtype (
5 ab:Array<Base>, ad:Array<Derived>,
6 tb: [Base, Base], td: [Derived, Derived],
7 ) {
8 ab = ad// Compile-time error
9 tb = td// Compile-time error
10 }

```
240 Chapter 15. Semantic Rules
```

#### 15.2.1 Subtyping for Non-Generic Classes and Interfaces

Sfor non-generic classes and interfaces is a direct _subclass_ or _subinterface_ ofT(or ofObjecttype) when one of the
following conditions is true:

- ClassSis a _direct subtype_ of classT(S<:T) ifTis mentioned in theextendsclause ofS(see _Class Extension_
    _Clause_ ):

```
1 // Illustrating S<:T
2 classT {}
3 classS extendsT {}
4 functionfoo(t: T) {}
5
6 // Using T
7 foo(newT)
8
9 // Using S (S<:T)
10 foo(newS)
```
- ClassSis a _direct subtype_ of classObject(S<:Object) ifShas no _Class Extension Clause_ :

```
1 // Illustrating S<:Object
2 classS {}
3 functionfoo(o: Object) {}
4
5 // Using Object
6 foo(newObject)
7
8 // Using S (S<:Object)
9 foo(newS)
```
- ClassSis a _direct subtype_ of interfaceT(S<:T) ifTis mentioned in theimplementsclause ofS(see _Class_
    _Implementation Clause_ ):

```
1 // Illustrating S<:T
2 // S is class, T is interface
3 interfaceT {}
4 classS implementsT {}
5 functionfoo(t: T) {}
6 lets: S=newS
7
8 // Using T
9 lett: T= s
10 foo(t)
11
12 // Using S (S<:T)
13 foo(s)
```
- InterfaceSis a _direct subtype_ of interfaceT(S<:T) ifTis mentioned in theextendsclause ofS(see _Superin-_
    _terfaces and Subinterfaces_ ):

```
1 // Illustrating S<:T
2 // S is interface, T is interface
3 interfaceT {}
4 interfaceS extendsT {}
(continues on next page)
```
**15.2. Subtyping 241**


```
(continued from previous page)
5 functionfoo(t: T) {}
6 lett: T
7 lets: S
8
9 // Using T
10 classA implementsT {}
11 t =newA
12 foo(t)
13
14 // Using S (S<:T)
15 classB implementsS {}
16 s =newB
17 foo(s)
```
- InterfaceSis a _direct subtype_ of classObject(S<:Object) ifShas noextendsclause (see _Superinterfaces_
    _and Subinterfaces_ ).

```
1 // Illustrating subinterface of Object
2 interfaceS {}
3 functionfoo(o: Object) {}
4
5 // Using Object
6 foo(newObject)
7
8 // Using subinterface of Object
9 classA implementsS {}
10 lets: S=newA;
11 foo(s)
```
#### 15.2.2 Subtyping for Generic Classes and Interfaces

A _generic class_ or _generic interface_ is declared asC<F 1 ,..., Fn>, where _n_ >0 is a _direct subtype_ of another generic
class or interfaceT, if one of the following conditions is true:

- Tis a _direct superclass_ ofC<F 1 ,..., Fn> mentioned in theextendsclause ofC:

```
1 // T<U, V> is direct superclass of C<U,V>
2 // T<U, V> >: C<U, V>
3
4 classT<U, V> {
5 foo(p:U|V): U|V {return p }
6 }
7
8 classC<U, V>extendsT<U, V> {
9 bar(u:U): U {return u }
10 }
11
12
13 // OK, exact match
(continues on next page)
```
**242 Chapter 15. Semantic Rules**


```
(continued from previous page)
14 lett: T<int,boolean> =newT<int,boolean>
15 letc: C<int,boolean> =newC<int,boolean>
16
17
18 // OK, assigning to direct superclass
19 t = newC<int, boolean>
20
21 // CTE, cannot assign to subclass
22 c = newT<int, boolean>
```
- Tis one of direct superinterfaces ofC<F 1 ,..., Fn> (see _Superinterfaces and Subinterfaces_ ):

```
1 // Interface I<U, V> is direct superinterface
2 // of J<U,V>, X<U, V>
3
4 interfaceI<U, V> {
5 foo(u:U): U;
6 bar(v:V): V;
7 }
8
9 // J<U, V> <: I<U, V>
10 // since J extebds I
11 interfaceJ<U, V>extendsI<U, V>
12 {
13 foo(u:U): U
14 bar(v:V): V
15
16 foo1(p:U|V): U|V
17 }
18
19 // X<U, V> <: I<U, V>
20 // since X implements I
21 classX<U, V>implements I<U,V> {
22 foo(u: U): U { return u }
23 bar(v: V): V { returnv }
24 }
25
26 // Y<U,V> <: J<U, V> (directly)
27 // Also Y<U, V> <: I<U, V> (transitively)
28 classY<U, V>implements J<U,V> {
29 foo(u: U): U { return u }
30 bar(v: V): V { returnv }
31
32 foo1(p:U|V): U|V { return p }
33 }
34
35 leti: I<int,boolean>
36 letj: J<int,boolean>
37 letx =newX<int,boolean>
38 lety =newY<int,boolean>
39
40 // OK, assigning to direct supertypes
(continues on next page)
```
**15.2. Subtyping 243**


```
(continued from previous page)
41 i = x
42 j = y
43
44 // OK, assigning subinterface (J<:I)
45 i = j
46
47 // CTE, cannot assign superinterface (I>:JJ
48 j = i
```
- Tis typeObject(C<:Object) ifC<F 1 ,..., Fn> is either a generic class type with no _direct superclasses_ , or
    a generic interface type with no direct superinterfaces:

```
1 // Object is direct superclass and for C<U,V>
2 // and direct superinrerface for I<U,V>
3 //
4 classC<U, V> {
5 foo(u:U): U {return u }
6 bar(v:V): V { return v }
7 }
8 interfaceI<U, V> {
9 foo(u:U): U {return u }
10 bar(v:V): V { return v }
11 }
12
13 leto: Object= newObject
14 letc: C<int,boolean> =newC<int,boolean>
15 leti: I<int,boolean>
16
17 // // example1 - C<U,V> <: Object
18 functionexample1(o:Object) {}
19
20 // OK, example(Object)
21 example1(o)
22 // OK, C<int, boolean> <: Object
23 example1(c)
24
25 // // example2 - I<U,V> <: Object
26 functionexample2(o:Object) {}
27 classD<U, V>implements I<U, V> {}
28 i =newD<int,boolean>
29
30 // OK, example2(Object)
31 example2(o)
32 // OK, I<int, boolean> <: Object
33 example2(i)
```
The direct supertype of a type parameter is the type specified as the constraint of that type parameter.

If type parameters of a generic class or an interface have a variance specified (see _Type Parameter Variance_ ), then the
subtyping for instantiations of the class or interface is determined in accordance with the variance of the appropriate
type parameter. For example, with generic classG<in T1,out T2>theG<S,T> <: G<U, V>whenS>:UandT<:V

The following code illustrates this:

**244 Chapter 15. Semantic Rules**


1 // Subtyping illustration for generic with parameter variance
2
3 // U1 <: U0
4 classU0 {}
5 classU1 extendsU0 {}
6
7 // Generic with contravariant parameter
8 classE<in T> {}
9
10 lete0:E<U0> = newE<U1>// CTE, E<U0> is subtype of E<U1>
11 lete1:E<U1> = newE<U0>// OK, E<U1> is supertype for E<U0>
12
13 // Generic with covariant parameter
14 classF<out T> {}
15
16 letf0:F<U0> = newF<U1>// OK, F<U0> is supertype for F<U1>
17 letf1:F<U1> = newF<U0>// CTE, F<U1> is subtype of F<U0>

#### 15.2.3 Subtyping for Literal Types

```
Anystringliteral type (see String Literal Types ) is subtype of typestring. It affects overriding as shown in the
example below:
```
1 class Base {
2 foo(p: "1"):string {return "42" }
3 }
4 class DerivedextendsBase {
5 overridefoo(p: string): "1" {return "1" }
6 }
7 // Type "1" <: string
8
9 letbase: Base=newDerived
10 letresult:string = base.foo("1")
11 /* Argument "1" (value) is compatible to type "1" and to type string in
12 the overridden method
13 Function result of type string accepts "1" (value) of literal type "1"
14 */

```
Literal typenull(see Literal Types ) is a subtype and a supertype to itself. Similarly, literal typeundefinedis a
subtype and a supertype to itself.
```
```
15.2. Subtyping 245
```

#### 15.2.4 Subtyping for Union Types

```
A union typeUparticipates in a subtyping relationship (see Subtyping ) in the following cases:
```
1. Union typeU(U 1 | ... | Un) is a subtype of typeTif eachUiis a subtype ofT.

1 lets1: "1" | "2" = "1"
2 lets2:string = s1// ok
3
4 leta: string| number |boolean= "abc"
5 letb: string| number = 42
6 a = b // OK
7 b = a // compile-time error, boolean is absent is'b'
8
9 class Base {}
10 class Derived1extendsBase {}
11 class Derived2extendsBase {}
12
13 letx: Base= ...
14 lety: Derived1| Derived2 = ...
15
16 x = y // OK, both Derived1 and Derived2 are subtypes of Base
17 y = x // compile-time error
18
19 letx: Base|string = ...
20 lety: Derived1|string ...
21 x = y // OK, Derived1 is subtype of Base
22 y = x // compile-time error

2. TypeTis a subtype of union typeU(U 1 | ... | Un) if for somei Tis a subtype ofUi.

```
1 letu: number| string = 1// ok
2 u = "aa"// ok
3 u = 1.0 // ok, 1.0 is of type'number'(double)
4 u = 1 // compile-time error, type 'int'is not a subtype of 'number'
5 u =true// compile-time error
```
```
Note. If union type normalization produces a single type, then this type is used instead of the initial set of union types.
This concept is represented in the example below:
```
```
1 letu: "abc" | "cde" | string// type of 'u'is string
```
#### 15.2.5 Subtyping for Function Types

```
Function typeFwith parametersFP 1 , ... , FPmand return typeFRis a subtype of function typeSwith parameters
SP 1 , ... , SPnand return typeSRif all of the following conditions are met:
```
- m≤n;
- Parameter type ofSPifor eachi≤mis a subtype of parameter type ofFPi(contravariance), andSPiis: - Rest
    parameter ifFPiis a rest parameter; - Optional parameter ifFPiis an optional parameter.
- TypeFRis a subtype ofSR(covariance).

```
246 Chapter 15. Semantic Rules
```

1 class Base {}
2 class DerivedextendsBase {}
3
4 functioncheck(
5 bb: (p:Base) => Base,
6 bd: (p:Base) => Derived,
7 db: (p:Derived) => Base,
8 dd: (p:Derived) => Derived
9 ) {
10 bb = bd
11 /* OK: identical parameter types, and covariant return type */
12 bb = dd
13 /* Compile-time error: parameter type are not contravariant */
14 db = bd
15 /* OK: contravariant parameter types, and covariant return type */
16
17 letf: (p:Base, n: number) => Base = bb
18 /* OK: subtype has less parameters */
19
20 letg: () => Base = bb
21 /* Compile-time error: less parameters than expected */
22 }
23
24 letfoo: (x?:number, y?:string) =>void= ():void=> {} // OK:``m <= n``
25 foo = (p?: number): void=> {} // OK: ``m <= n``
26 foo = (p1?:number, p2?: string): void=> {} // OK: Identical types
27 foo = (p: number): void=> {}
28 // Compile-time error: 1st parameter in type is optional but mandatory in lambda
29 foo = (p1: number, p2?: string): void=> {}
30 // Compile-time error: 1st parameter in type is optional but mandatory in lambda

#### 15.2.6 Subtyping for Fixed-Size Array Types

```
Subtyping for fixed-size array types is based on subtyping of their element types. It is formally defined as follows:
FixedSize<B> <: FixedSize<A>ifB <: A.
The situation is represented in the following example:
```
```
1 letx: FixedArray<number> = [1, 2, 3]
2 lety: FixedArray<Object> = x// ok, as number <: Object
3 x = y // compile-time error
```
```
Such subtyping allows array assignments that can lead toArrayStoreErrorat runtime if a value of a type which is
not a subtype of an element type of one array is put into that array by using the subtyping of another array element
type. Type safety is ensured by runtime checks performed by the runtime system as represented in the example below:
```
```
1 class C {}
2 class DextendsC {}
3
(continues on next page)
```
```
15.2. Subtyping 247
```

(continued from previous page)
4 functionfoo (ca:FixedArray<C>) {
5 ca[0] =newC()// ArrayStoreError if ca refers to FixedArray<D>
6 }
7
8 letda:FixedArray<D> = [newD()]
9
10 foo(da)// leads to runtime error in 'foo'

#### 15.2.7 Subtyping for Intersection Types

```
Intersection typeIdefined as (I 1 & ... | In) is a subtype of typeTifIiis a subtype ofTfor some i.
TypeTis a subtype of intersection type (I 1 & ... | In) ifTis a subtype of eachIi.
```
#### 15.2.8 Subtyping for Difference Types

```
Difference typeA - Bis a subtype ofTifAis a subtype ofT.
TypeTis a subtype of the difference typeA - BifTis a subtype ofA, and no value belongs both toTandB(i.e.,T &
B = never).
```
### 15.3 Type Identity

```
Identity relation between two types means that the types are indistinguishable. Identity relation is symmetric and
transitive. Identity relation for typesAandBis defined as follows:
```
- Array typesA=T1[]andB=Array<T2>are identical ifT1andT2are identical.
- Tuple typesA= [T 1 ,T 2 ,...,Tn] andB= [U 1 ,U 2 ,...,Um] are identical on condition that:
    **-** nis equal tom, i.e., the types have the same number of elements;
    **-** Every _T_ iis identical to _U_ ifor any _i_ in1 .. n.
- Union typesA=T 1 |T 2 |...|TnandB=U 1 |U 2 |...|Umare identical on condition that:
    **-** nis equal tom, i.e., the types have the same number of elements;
    **-** _U_ iinUundergoes a permutation after which every _T_ iis identical to _U_ ifor any _i_ in1 .. n.
- TypesAandBare identical ifAis a subtype ofB(A<:B), andBis at the same time a subtype ofA(A:>B).
**Note.** _Type Alias Declaration_ creates no new type but only a new name for the existing type. An alias is indistinguishable
from its base type.

```
248 Chapter 15. Semantic Rules
```

```
Note. If a generic class or an interface has a type parameterTwhile its method has its own type parameterT, then the
two types are different and unrelated.
```
1 classA<T> {
2 data:T
3 constructor(p:T) {this.data = p }// OK, as here 'T'is a class type parameter
4 method <T>(p:T) {
5 this.data = p// compile-time error as'T'of the class is different from'T'of the␣
˓→method
6 }
7 }

### 15.4 Assignability

```
TypeT 1 is assignable to typeT 2 if:
```
- T 1 is typenever;
- T 1 is identical toT 2 (see _Type Identity_ );
- T 1 is a subtype ofT 2 (see _Subtyping_ ); or
- _Implicit conversion_ (see _Implicit Conversions_ ) is present that allows converting a value of typeT 1 to typeT 2.
_Assignability_ relationship is asymmetric, i.e., thatT 1 is assignable toT 2 does not imply thatT 2 is assignable to typeT 1.

### 15.5 Invariance, Covariance and Contravariance

```
Variance is how subtyping between types relates to subtyping between derived types, including generic types (See
Generics ), member signatures of generic types (type of parameters, return type), and overriding entities (See Override-
Compatible Signatures ). Variance can be of three kinds:
```
- Covariance,
- Contravariance, and
- Invariance.
_Covariance_ means it is possible to use a type which is more specific than originally specified.
_Contravariance_ means it is possible to use a type which is more general than originally specified.
_Invariance_ means it is only possible to use the original type, i.e., there is no subtyping for derived types.
Valid and invalid usages of variance are represented in the examples below. If classBaseis defined as follows:

1 classBase {
2 method_one(p:Base): Base {}
3 method_two(p:Derived): Base {}
4 method_three(p:Derived): Derived {}
5 }

```
15.4. Assignability 249
```

```
—then the code below is valid:
```
1 classDerivedextendsBase {
2 // invariance: parameter type and return type are unchanged
3 overridemethod_one(p:Base): Base {}
4
5 // covariance for the return type: Derived is a subtype of Base
6 overridemethod_two(p:Derived): Derived {}
7
8 // contravariance for parameter types: Base is a supertype for Derived
9 overridemethod_three(p:Base): Derived {}
10 }

```
On the contrary, the following code causes compile-time errors:
```
```
1 classDerivedextendsBase {
2
3 // covariance for parameter types is prohibited
4 overridemethod_one(p:Derived): Base {}
5
6 // contravariance for the return type is prohibited
7 overridemethod_tree(p:Derived): Base {}
8 }
```
### 15.6 Compatibility of Call Arguments

```
The following semantic checks must be performed to arguments from the left to the right when checking the validity
of any function, method, constructor, or lambda call:
Step 1 : All arguments in the form of spread expression (see Spread Expression ) are to be linearized recursively to
ensure that no spread expression is left at the call site.
Step 2 : The following checks are performed on all arguments from left to right, starting fromarg_pos= 1 andpar_pos
= 1:
if parameter at positionpar_posis of non-rest form, then
if T arg_pos<: T par_pos, then incrementarg_posandpar_poselse a compile-time error occurs,
exit Step 2
else // parameter is of rest form (see Rest Parameter )
if parameter is of rest_array_form, then
if T arg_pos<: T rest_array_type, then incrementarg_poselse incrementpar_pos
else // parameter is of rest_tuple_form
for rest_tuple_pos in 1 .. rest_tuple_types.count do
if T arg_pos<: T rest_tuple_pos, then incrementarg_posand rest_tuple_pos else
if rest_tuple_pos < rest_tuple_types.count, then incrementrest_tuple_pos
else a compile-time error occurs, exit Step 2
end incrementpar_pos
```
```
250 Chapter 15. Semantic Rules
```

```
end
end
Checks are represented in the examples below:
```
1 call (...[1, "str", true], ...[ ...123]) // Initial call form
2
3 call (1, "str", true, 123)// To be unfolded into the form with no spread expressions
4
5
6
7 functionfoo1 (p:Object) {}
8 foo1 (1) // Type of' 1 'must be assignable to'Object'
9 // p becomes 1
10
11 functionfoo2 (...p:Object[]) {}
12 foo2 (1, "111") // Types of' 1 'and "111" must be assignable to'Object'
13 // p becomes array [1, "111"]
14
15 functionfoo31 (...p: (number|string)[]) {}
16 foo31 (...[1, "111"]) // Type of array literal [1, "111"] must be assignable to␣
˓→(number|string)[]
17 // p becomes array [1, "111"]
18
19 functionfoo32 (...p: [number,string]) {}
20 foo32 (...[1, "111"]) // Types of' 1 'and "111" must be assignable to 'number'and'string
˓→'accordingly
21 // p becomes tuple [1, "111"]
22
23 functionfoo4 (...p:number[]) {}
24 foo4 (1, ...[2, 3]) //
25 // p becomes array [1, 2, 3]
26
27 functionfoo5 (p1: number, ...p2:number[]) {}
28 foo5 (...[1, 2, 3]) //
29 // p1 becomes 1, p2 becomes array [2, 3]

### 15.7 Type Inference

```
ArkTS supports strong typing but allows not to burden a programmer with the task of specifying type annotations
everywhere. A smart compiler can infer types of some entities and expressions from the surrounding context. This
technique called type inference allows keeping type safety and program code readability, doing less typing, and focusing
on business logic. Type inference is applied by the compiler in the following contexts:
```
- _Type Inference for Numeric Literals_ ;
- Variable and constant declarations (see _Type Inference from Initializer_ );
- Implicit generic instantiations (see _Implicit Generic Instantiations_ );
- Function, method or lambda return type (see _Return Type Inference_ );

```
15.7. Type Inference 251
```

- Lambda expression parameter type (see _Lambda Signature_ );
- Array literal type inference (see _Array Literal Type Inference from Context_ , and _Array Type Inference from Types_
    _of Elements_ );
- Object literal type inference (see _Object Literal_ );
- Smart types (see _Smart Types_ ).

#### 15.7.1 Type Inference for Numeric Literals

```
The type of expression of a numeric type for Constant Expressions is first evaluated from the expression as follows:
```
- Type of an integer literal is the default type of the literal:intorlong(see _Integer Literals_ );
- Type of a floating-point literal is the default type of the literal:doubleorfloat(see _Floating-Point Literals_ );
- Type of a named constant is specified in the constant declaration;
- Result type of an operator is evaluated according to the rules of the operator;
- Type of a _Cast Expression_ is specified in the expression target type.
The evaluated numeric result type can be inferred to a numeric _target type_ from the context on condition that:
1. Last executed operator in the expression is not a cast operatoras;
2. _Target type_ is a numeric type larger then the evaluated result type; or
3. The evaluated result type is an integer type, the _target type_ is a smaller integer type with the value of the expression
fitting into its range; or
4. The _target type_ isfloat, the evaluated result type isdoubleand the value of the expression fits into the range
of typefloat.
A compile-time error occurs if the context is a union type, and the evaluated value can be treated as value of several of
union component types.
Valid and invalid narrowing is represented in the examples below:

1 letb: byte= 127// ok, int -> byte narrowing
2 b = 64 + 63// ok, int -> byte narrowing
3 b = 128// compile-time-error, value is out of range
4 b = 1.0// compile-time-error, floating-point value cannot be narrowed
5 b = 1 as short// // compile-time-error, cast expression fixes'short'type
6
7 lets: short= 32768// compile-time-error, value is out of range
8
9 letu: byte|int= 1// compile-time error, ambiguity

```
252 Chapter 15. Semantic Rules
```

#### 15.7.2 Smart Types

```
Data entities like local variables (see Variable and Constant Declarations ) and parameters (see Parameter List ), if not
captured in a lambda body and modified by the lambda code, are subjected to smart typing.
Every data entity has a static type, which is specified explicitly or inferred at the point of declaration. This type defines
the set of operations that can be applied to the entity (namely, what methods can be called, and what other entities can
be accessed if the entity acts as a receiver of the operation):
```
```
1 leta =newObject
2 a.toString()// entity 'a'has method toString()
```
```
If an entity is class type (see Classes ), interface type (see Interfaces ), or union type (see Union Types ), then the compiler
can narrow (smart cast) a static type to a more precise type (smart type), and allow operations that are specific to the
type so narrowed:
```
1 functionboo() {
2 leta: number| string= 42
3 a++/* Smart type of'a'is number and number-specific
4 operations are type-safe */
5 }
6
7 class Base {}
8 class DerivedextendsBase { method () {} }
9 functiongoo() {
10 letb: Base=newDerived
11 b.method ()/* Smart type of'b'is Derived and Derived-specific
12 operations can be applied in type-safe way */
13 }

```
Other examples are explicit calls toinstanceof(see InstanceOf Expression ) or checks againstnull(see Equality
Expressions ) as parts ofifstatements (see if Statements ) or ternary conditional expressions (see Ternary Conditional
Expressions ):
```
```
1 functionfoo (b:Base, d:Derived|null) {
2 if (b instanceof Derived) {
3 b.method()
4 }
5 if (d != null) {
6 d.method()
7 }
8 }
```
```
In like cases, a smart compiler requires no additional checks or casts (see Cast Expression ) to deduce a smart type of
an entity.
Overloading (see Overload Declarations ) can cause tricky situations when a smart type results in calling an entity that
suits the smart type rather than a declared type of an argument (see Overload Resolution ):
```
```
1 class Base {b = 1}
2 class DerivedextendsBase{d = 2}
3
4 functionfooBase (p:Base) {}
5 functionfooDerived (p: Derived) {}
6
(continues on next page)
```
```
15.7. Type Inference 253
```

(continued from previous page)
7 overload foo { fooDerived, fooBase }
8
9 functiontoo() {
10 leta: Base=newBase
11 foo (a)// fooBase will be called
12 letb: Base=newDerived
13 foo (b)// as smart type of 'b'is Derived, fooDerived will be called
14 }

```
Particular cases supported by the compiler are determined by the compiler implementation.
```
### 15.8 Overriding

```
Method overriding is the language feature closely connected with inheritance. It allows a subclass or a subinterface to
offer a specific implementation of a method already defined in its supertype optionally with modified signature.
The actual method to be called is determined at runtime based on object type. Thus, overriding is related to runtime
polymorphism.
ArkTS uses the override-compatibility rule to check the correctness of overriding. The overriding is correct if method
signature in a subtype (subclass or subinterface) is override-compatible with the method defined in a supertype (see
Override-Compatible Signatures ).
An implementation is forced to Make a Bridge Method for Overriding Method in some cases of method overriding.
```
#### 15.8.1 Overriding in Classes

```
Note. Only accessible (see Accessible ) methods are subjected to overriding. The same rule applies to accessors in case
of overriding.
An overriding member can keep or extend an access modifier (see Access Modifiers ) of a member that is inherited or
implemented. Otherwise, a compile-time error occurs.
A compile-time error occurs if an attempt is made to do the following:
```
- Override a private method of a superclass; or
- Declare a method with the same name as that of a private method with default implementation from any super-
    interface.

```
1 classBase {
2 publicpublic_member() {}
3 protectedprotected_member() {}
4 privateprivate_member() {}
5 }
6
7 interfaceInterface {
(continues on next page)
```
```
254 Chapter 15. Semantic Rules
```

(continued from previous page)
8 public_member() // All members are public in interfaces
9 privateprivate_member() {}// Except private methods with default implementation
10 }
11
12 classDerivedextendsBaseimplements Interface {
13 public overridepublic_member() {}
14 // Public member can be overridden and/or implemented by the public one
15 public overrideprotected_member() {}
16 // Protected member can be overridden by the protected or public one
17 overrideprivate_member() {}
18 // A compile-time error occurs if an attempt is made to override private member
19 // or implement the private methods with default implementation
20 }

```
The table below represents semantic rules that apply in various contexts:
```
```
Context Semantic Check
An instance method is defined in a subclass with the
same name as the instance method in a superclass.
```
```
If signatures are override-compatible (see Override-
Compatible Signatures ), then overriding is used. Oth-
erwise, a compile-time error occurs.
```
```
1 classBase {
2 method_1() {}
3 method_2(p:number) {}
4 }
5 classDerivedextendsBase {
6 overridemethod_1() {}// overriding
7 method_2(p:string) {}// compile-time error
8 }
```
```
A constructor is defined in a subclass. All base class constructors are available for call in all
derived class constructors viasupercall (see Explicit
Constructor Call ).
```
```
1 classBase {
2 constructor(p:number) {}
3 }
4 classDerivedextendsBase {
5 constructor(p:string) {
6 super(5)
7 }
8 }
```
```
15.8. Overriding 255
```

#### 15.8.2 Overriding and Overloading in Interfaces

```
Context Semantic Check
A method is defined in a subinterface with the same
name as the method in the superinterface.
```
```
If signatures are override-compatible (see Override-
Compatible Signatures ), then overriding is used. Oth-
erwise, a compile-time error occurs.
A method is defined in a subinterface with the same
name as the private method in the superinterface.
```
```
A compile-time error occurs.
```
1 interfaceBase {
2 method_1()
3 method_2(p:number)
4 privatefoo() {}// private method with implementation body
5 }
6 interfaceDerivedextendsBase {
7 method_1()// overriding
8 method_2(p:string)// compile-time error: non-compatible signature
9 foo(p:number):void // compile-time error: the same name as private method
10 }

```
Two or more methods with the same name are defined in
the same interface.
```
```
TBD is used.
```
```
1 interfaceanInterface {
2 instance_method() // 1st signature
3 instance_method(p:number) // 2nd signature
4 }
```
#### 15.8.3 Override-Compatible Signatures

```
If there are two classesBaseandDerived, and classDerivedoverrides the methodfoo()ofBase, thenfoo()in
Basehas signatureS 1 <V 1 , ... Vk> (U 1 , ..., Un):Un+1, andfoo()inDerivedhas signatureS 2 <W 1 , ... Wl>
(T 1 , ..., Tm):Tm+1as in the example below:
```
```
1 class Base {
2 foo <V1, ... Vk> (p1:U1, ... pn: Un): Un+1
3 }
4 class DerivedextendsBase {
5 overridefoo <W1, ... Wl> (p1:T1, ... pm:Tm): Tm+1
6 }
```
```
The signatureS 2 is override-compatible withS 1 only if all of the following conditions are met:
```
1. Number of parameters of both methods is the same, i.e.,n = m.
2. Each parameter typeTiis a supertype ofUiforiin1..n(contravariance).

```
256 Chapter 15. Semantic Rules
```

3. If return typeTm+1isthis, thenUn+1isthis, or any of superinterfaces or superclass of the current type. Other-
    wise, return typeTm+1is a subtype ofUn+1(covariance).
4. Number of type parameters of either method is the same, i.e.,k = l.
5. Constraints ofW 1 , ...Wlare to be contravariant (see _Invariance, Covariance and Contravariance_ ) to the appro-
    priate constraints ofV 1 , ...Vk.
The following rule applies to generics:
- Derived class must have type parameter constraints to be subtype (see _Subtyping_ ) of the respective type parameter
constraint in the base type;
- Otherwise, a compile-time error occurs.

1 classBase {}
2 classDerivedextendsBase {}
3 classA1 <CovariantTypeParameter extendsBase> {}
4 classB1 <CovariantTypeParameter extendsDerived> extendsA1<CovariantTypeParameter> {}
5 // OK, derived class may have type compatible constraint of type parameters
6
7 classA2 <ContravariantTypeParameter extendsDerived> {}
8 classB2 <ContravariantTypeParameter extendsBase> extendsA2<ContravariantTypeParameter>
˓→{}
9 // Compile-time error, derived class cannot have non-compatible constraints of type␣
˓→parameters

```
The semantics is represented in the examples below:
```
1. **Class/Interface Types**

1 interfaceBase {
2 param(p:Derived):void
3 ret(): Base
4 }
5
6 interfaceDerivedextendsBase {
7 param(p:Base): void // Contravariant parameter
8 ret(): Derived // Covariant return type
9 }

2. **Function Types**

1 interfaceBase {
2 param(p: (q:Base)=>Derived):void
3 ret(): (q:Derived)=> Base
4 }
5
6 interfaceDerivedextendsBase {
7 param(p: (q:Derived)=>Base):void // Covariant parameter type, contravariant␣
˓→return type
8 ret(): (q:Base)=> Derived // Contravariant parameter type, covariant␣
˓→return type
9 }

3. **Union Types**

```
15.8. Overriding 257
```

1 interface BaseSuperType {}
2 interface BaseextendsBaseSuperType {
3 // Overriding for parameters
4 param<TextendsDerived, UextendsBase>(p:T | U):void
5
6 // Overriding for return type
7 ret<TextendsDerived, U extendsBase>(): T | U
8 }
9
10 interface DerivedextendsBase {
11 // Overriding kinds for parameters, Derived <: Base
12 param<TextendsBase, UextendsObject>(
13 p:Base| BaseSuperType // contravariant parameter type: Derived | Base <: Base␣
˓→| BaseSuperType
14 ): void
15 // Overriding kinds for return type
16 ret<TextendsBase, UextendsBaseSuperType>(): T | U
17 }

4. **Type Parameter Constraint**

```
1 interfaceBase {
2 param<TextendsDerived>(p:T):void
3 ret<TextendsDerived>(): T
4 }
5
6 interfaceDerivedextendsBase {
7 param<TextendsBase>(p:T): void // Contravariance for constraints of type␣
˓→parameters
8 ret<TextendsBase>(): T // Contravariance for constraints of the␣
˓→return type
9 }
```
```
Override compatibility withObjectis represented in the example below:
```
1 interface Base {
2 kinds_of_parameters<TextendsDerived, UextendsBase>( // It represents all␣
˓→possible kinds of parameter type
3 p01:Derived,
4 p02: (q:Base)=>Derived,
5 p03:number,
6 p04:T| U,
7 p05:E1,
8 p06:Base[],
9 p07: [Base, Base]
10 ): void
11 kinds_of_return_type(): Object// It can be overridden by all subtypes of Object
12 }
13 interface DerivedextendsBase {
14 kinds_of_parameters(// Object is a supertype for all class types
15 p1:Object,
16 p2:Object,
17 p3:Object,
(continues on next page)

```
258 Chapter 15. Semantic Rules
```

```
(continued from previous page)
```
18 p4:Object,
19 p5:Object,
20 p6:Object,
21 p7:Object
22 ): void
23 }
24
25 interface Derived1extendsBase {
26 kinds_of_return_type(): Base // Valid overriding
27 }
28 interface Derived2extendsBase {
29 kinds_of_return_type(): (q:Derived)=> Base// Valid overriding
30 }
31 interface Derived3extendsBase {
32 kinds_of_return_type():number // Valid overriding
33 }
34 interface Derived4extendsBase {
35 kinds_of_return_type():number | string// Valid overriding
36 }
37 interface Derived5extendsBase {
38 kinds_of_return_type(): E1// Valid overriding
39 }
40 interface Derived6extendsBase {
41 kinds_of_return_type(): Base[]// Valid overriding
42 }
43 interface Derived7extendsBase {
44 kinds_of_return_type(): [Base, Base]// Valid overriding
45 }

### 15.9 Overloading

```
Overloading is the language feature that allows to use the same name to call several functions, or methods, or construc-
tors with different signatures.
The actual function, method, or constructor to be called is determined at compile time. Thus, overloading is compile-
time polymorphism by name.
ArkTS supports the following two overloading mechanisms:
```
- Conventional overloading TBD; and
- Innovative _managed overloading_ (see _Overload Declarations_ ).
_Overload resolution_ is used to select one entity to call from a set of candidates if the name to call refers to an _overload
declaration_ (see _Overload Resolution_ ).
Both mechanisms of resolution use the first-match textual order to streamline the resolution process.
TBD: A compile-time warning is issued if the order of entities in an _overload declaration_ implies that some overloaded
entities can never be selected for a call.

```
15.9. Overloading 259
```

```
1 functionf1 (p: number) {}
2 functionf2 (p: string) {}
3 functionf3 (p: number|string) {}
4 overload foo {f1, f2, f3} // f3 will never be called as foo()
5
6 foo (5) // f1() is called
7 foo ("5") // f2() is called
```
#### 15.9.1 Overload Resolution

```
Overload declaration defines an ordered set of entities, and the first entity from this set that is accessible and has an
appropriate signature is used to call at the call site. This approach is called managed overloading because the first-
match algorithm provides full control for a developer to select a specific entity to call. This developer control over calls
is represented in the following example:
```
1 functionmax2i(a:int, b:int): int
2 return a > b? a :b
3 }
4 functionmax2d(a:double, b:double): double {
5 return a > b? a :b
6 }
7 functionmaxN(...a: double[]):double {
8 // returns max element in array 'a'
9 }
10 overload max {max2i, max2d, maxN}
11
12 leti = 1
13 letj = 2
14 letpi = 3.14
15
16 max(i, j) // max2i is used
17 max(i, pi) // max2d is used
18 max(i, pi, 4)// maxN is used
19 max(1) // maxN is used
20 max(false, true)// compile-time error, no appropriate signature

```
Overload resolution for an instance method overload (see Class Method Overload Declarations ) always uses the type
of the object reference known at compile time. It can be either the type used in a declaration, or a smart type (see Smart
Types ) as represented in the example below:
```
```
1 class A {
2 foo1(x:A) { console.log("A.foo") }
3 overload foo {foo1}
4 }
5 class BextendsA {
6 foo2(x:B) { console.log("B.foo") }
7 overload foo {foo2, foo1}
8 }
9
(continues on next page)
```
```
260 Chapter 15. Semantic Rules
```

```
(continued from previous page)
```
10 functiontest(a:A) {
11 a.foo(newB()) //'foo1'is called as overload from'A'is used
12 }
13
14 test(newB())// output: A.foo
15
16 letb =newB()
17 b.foo(b)// output: B.foo, as overload from'B'is used

### 15.10 Type Erasure

```
Type erasure is the compilation technique which provides a special handling of certain language types , primarily Gener-
ics , when applied to the semantics of the following expressions:
```
- _InstanceOf Expression_ ;
- _Cast Expression_.
As a result, special types must be used for the execution of such expressions. Certain _types_ in such expressions are
handled as their corresponding _effective types_ , while the _effective type_ is defined as type mapping. The _effective type_
of a specific typeTis always a supertype ofT. As a result, the relationship of an original type and an _effective type_ can
have the following two kinds:
- _Effective type_ ofTis identical toT, and _type erasure_ has no effect. So, typeTis _retained_.
- If _effective type_ ofTis not identical toT, then the typeTis considered affected by _type erasure_ , i.e., _erased_.
In addition, accessing a value of typeT, particularly by _Field Access Expression_ , _Method Call Expression_ , or _Function
Call Expression_ , can causeClassCastErrorthrown if typeTand thetargettype are both affected by _type erasure_ ,
and the value is produced by a _Cast Expression_.

1 class A<T> {
2 field?:T
3
4 test(value:Object) {
5 return valueinstanceof T // CTE, T is erased
6 }
7
8 cast(value:Object) {
9 return valueas T // OK, but check is done during execution
10 }
11 }
12
13 functioncastToA(p: Object) {
14 pinstanceof A<number>// CTE, A<number> is erased
15
16 returnp asA<number> // OK, but check is performed against type A, but not A<number>
17 }

```
Type mapping determines the effective types as follows:
```
```
15.10. Type Erasure 261
```

- _Type Parameter Constraint_ for _Type Parameters_.
- Instantiation of the same generic type (see _Explicit Generic Instantiations_ ) for _generic types_ (see _Generics_ ), with
    its type arguments selected in accordance with _Type Parameter Variance_ as outlined below:
       **-** _Covariant_ type parameters are instantiated with the constraint type;
       **-** _Contravariant_ type parameters are instantiated with the typenever;
       **-** _Invariant_ type parameters are instantiated with no type argument, i.e.,Array<T>is instantiated asArray<>.
- Union type constructed from the effective types of typesT1 | T2 ... Tnwithin the original union type for
    _Union Types_ in the formT1 | T2 ... Tn.
- Same for _Array Types_ in the formT[]as for generic typeArray<T>.
- Instantiation ofFixedArrayforFixedArray<T>instantiations, with the effective type of type argumentT
    preserved.
- Instantiation of an internal generic function type with respect to the number of parameter types _n_ for _Function_
    _Types_ in the form(P1, P2 ..., Pn) => R. Parameter typesP1, P2 ... Pnare instantiated withAny, and
    the return typeRis instantiated with typenever.
- Instantiation of an internal generic tuple type with respect to the number of element types _n_ for _Tuple Types_ in
    the form[T1, T2 ..., Tn].
- String for _String Literal Types_.
- Enumeration base type of the same const enum type for _const enum_ types (see _Enumerations_ ).
- Otherwise, the original type is _preserved_.

### 15.11 Static Initialization

_Static initialization_ is a routine performed once for each class (see _Classes_ ), namespace (see _Namespace Declarations_ ),
or module (see _Modules and Namespaces_ ).

_Static initialization_ execution involves the execution of the following:

- _Initializers_ of _variables_ or _static fields_ ;
- _Top-level statements_ ;
- Code inside a _static block_.

_Static initialization_ is performed before the first execution of one of the following operations:

- Invocation of a static method or function of an entity scope;
- Access to a static field or variable of an entity scope;
- Instantiation of an entity that is an interface or class;
- _Static initialization_ of a direct subclass of an entity that is a class.

**Note**. None of the operations above invokes a _static initialization_ recursively if the _static initialization_ of the same
entity is not complete.

**Note**. For namespaces, the code in a static block is executed only when namespace members are used in the program
(an example is provided in _Namespace Declarations_ ).

**262 Chapter 15. Semantic Rules**


If _static initialization_ routine execution is terminated due to an exception thrown, then the initialization is not complete.
Repeating an attempt to execute a _static initialization_ produces an exception again.

_Static initialization_ routine invocation of a concurrent execution (see _Coroutines (Experimental)_ ) involves synchroniza-
tion of all _coroutines_ that try to invoke it. The synchronization is to ensure that the initialization is performed only once,
and the operations that require the _static initialization_ to be performed are executed after the initialization completes.

If _static initialization_ routines of two concurrently initialized classes are circularly dependent, then a deadlock can
occur.

#### 15.11.1 Static Initialization Safety

A compile-time error occurs if a _named reference_ refers to a not yet initialized _entity_ , including one of the following:

- Variable (see _Variable and Constant Declarations_ ) of a module or namespace (see _Namespace Declarations_ );
- Static field of a class (see _Static and Instance Fields_ ).

If detecting an access to a not yet initialized _entity_ is not possible, then runtime evaluation is performed as follows:

- Default value is produced if the type of an entity has a default value;
- Otherwise,NullPointerErroris thrown.

### 15.12 Dispatch

As a result of assignment (see _Assignment_ ) to a variable or call (see _Method Call Expression_ or _Function Call Expres-
sion_ ), the actual runtime type of a parameter of class or interface can become different from the type explicitly specified
or inferred at the point of declaration.

In this situation method calls are dispatched during program execution based on their actual type.

This mechanism is called _dynamic dispatch_. Dynamic dispatch is used in OOP languages to provide greater flexibility
and the required level of abstraction. Unlike _static dispatch_ where the particular method to be called is known at
compile time, _dynamic dispatch_ requires additional action during program code execution. Compilation tools can
optimize dynamic to static dispatch.

### 15.13 Compatibility Features

Some features are added to ArkTS in order to support smooth TypeScript compatibility. Using these features while
doing the ArkTS programming is not recommended in most cases.

**15.12. Dispatch 263**


#### 15.13.1 Extended Conditional Expressions

```
ArkTS provides extended semantics for conditional expressions to ensure better TypeScript alignment. It affects the
semantics of the following:
```
- Ternary conditional expressions (see _Ternary Conditional Expressions_ , _Conditional-And Expression_ ,
    _Conditional-Or Expression_ , and _Logical Complement_ );
- whileanddostatements (see _while Statements and do Statements_ );
- forstatements (see _for Statements_ );
- ifstatements (see _if Statements_ ).
**Note**. The extended semantics is to be deprecated in one of the future versions of ArkTS.
The extended semantics approach is based on the concept of _truthiness_ that extends the boolean logic to operands of
non-boolean types.
Depending on the kind of a valid expression’s type, the value of the valid expression can be handled astrueorfalse
as described in the table below:

```
Value Type Kind Whenfalse Whentrue ArkTS Code Example to
Check
string empty string non-empty string s.length == 0
boolean false true x
enum enumconstant handled as
false
```
```
enumconstant handled as
true
```
```
x.valueOf()
```
```
number(double/float) 0 orNaN any other number n != 0 && !isNaN(n)
any integer type == 0 != 0 i != 0
bigint == 0n != 0n i != 0n
nullorundefined always never x != nullor
x != undefined
Union types When value isfalseac-
cording to this column
```
```
When value istrueac-
cording to this column
```
```
x != nullor
x != undefined for
union types with nullish
types
Any other nonNullish type never always new SomeType != null
```
```
Extended semantics of Conditional-And Expression and Conditional-Or Expression affects the resultant type of ex-
pressions as follows:
```
- Type of _conditional-and_ expressionA && Bequals the type ofBif the result ofAis handled astrue. Otherwise,
    the expression type equals the type ofA.
- Type of _conditional-or_ expressionA || Bequals the type ofBif the result ofAis handled asfalse. Otherwise,
    the expression type equals the type ofA.
The way this approach works in practice is represented in the example below. Anynonzeronumber is handled as
true. The loop continues until it becomeszerothat is handled asfalse:

1 for(leti = 10; i; i--) {
2 console.log (i)
3 }
4 /* And the output will be
5 10
6 9
(continues on next page)

```
264 Chapter 15. Semantic Rules
```

(continued from previous page)
7 8
8 7
9 6
10 5
11 4
12 3
13 2
14 1
15 */

```
15.13. Compatibility Features 265
```

**266 Chapter 15. Semantic Rules**


##### CHAPTER

### SIXTEEN

### CONCURRENCY

### 16.1 Introductory Note

Most modern hardware has multiple cores. To achieve maximum performance, the software must be capable of using
more than one core in some scenarios (e.g., multimedia processing, data analysis, simulation, modelling, databases
etc.).

Providing support to a number of asynchronous APIs at different levels is also crucial.

### 16.2 Concurrency Subsystem Overview

#### 16.2.1 Major Concurrency Features

ArkTS has APIs for asynchronous programming that enables tasks to be suspended and resumed later, and supports
coroutines that can run in parallel (implicitly or explicitly). Since the ArkTS coroutines share memory, a developer
must be aware about the possible associated issues, and use appropriate functionality to guarantee thread safety.

ArkTS enables both asynchronous programming and parallel-run coroutines, and provides machinery for trustworthy
concurrent programs by providing the following:

1. Asynchronous featuresasync/await/Promise;
2. Coroutines (experimental) in _Standard Library_ ;
3. Structured concurrency in _Standard Library_ (TaskPool API);
4. Synchronization primitives and “thread”-safe containers in _Standard Library_.

##### 267


### 16.3 Asynchronous API

#### 16.3.1 AsyncFunctions

Asyncfunctions are coroutines (i.e., functions which can be suspended and resumed later) that can be called as regular
functions. A compile-time error occurs if:

- Asyncfunction is called in a static initializer, including module scope;
- Asyncfunction has anabstractor anativemodifier;
- Return type of anasyncfunction is other thanPromise<T>.

TypePromise<T>is a library type discussed in detail in the ArkTS Concurrency Specification.

The returning values of both typePromise<T>and typeTare allowed inside theasyncfunction body (see _Return
Type Inference_ ).

Using return statement without an expression is allowed if the return type isPromise<void>. _No-argument_ return
statement can be added implicitly as the last statement of the function body if there is no explicit return statement in a
function with the returnPromise<void>.

**Note**. Using typePromise<void>is not recommended as this type is supported for the sake of backward TypeScript
compatibility only.

#### 16.3.2 AsyncLambdas

A lamdba with the modifierasync(see _Lambda Expressions_ ) is an implicit coroutine that can be called as a regular
lambda.

Asynclambdas follow the same rules as _Async Functions_.

#### 16.3.3 AsyncMethods

A class method with the modifierasync(see _Method Declarations_ ) is an implicit coroutine that can be called as a
regular method.

Asyncmethods follow the same rules as _Async Functions_.

#### 16.3.4 await.

The syntax of _await expression_ is presented below:

awaitExpression:
'await' expression
;

**268 Chapter 16. Concurrency**


The expression is a subtype of _Promise_. If expression is _Promise<T>_ , then type of _awaitExpression_ is _Awaited<T>_.

awaitis used to wait for _Promise_

IfPromisenot resolved, then the current coroutine is suspended until it is resolved.

If _Promise_ is rejected, then the reason of the rejection is thrown.

Usingawaitoutside of _async functions_ is forbidden.

#### 16.3.5 Promise

ThePromise objectis introduced to support asynchronous API. It is the object that represents a proxy for the result of
an asynchronous operation. The semantics ofPromiseis similar to the semantics ofPromisein JavaScript/TypeScript
if it is used in the context of a single coroutine.

Promise objectrepresents the values returned by the call of anasyncfunction.Promise objectcan be used
without any qualification as it is defined in the _Standard Library_.

ThePromiselifetime is not limited to the lifetime of the root coroutine as it is created.

Promiseis not in general designed to be used concurrently and simultaneously from multiple coroutines. However, it
is safe to do the following:

- PassPromisefrom one coroutine to another, and avoid using it again in the original coroutine.
- PassPromisefrom one coroutine to another, use it in both coroutines, and callthenonly in one coroutine.
- PassPromisefrom one coroutine to another, use it in both coroutines, and callthenin both coroutines. The
    user is to provide custom synchronization to guarantee thatthenis not called simultaneously for thisPromise.

The methods are used as follows:

- thentakes two arguments. The first argument is the callback used if the promise is fulfilled. The second argument
    is used if it is rejected, and returnsPromise<U>.
- Ifthenis called from the same parent coroutine several times, then the order ofthenis the same if called
    in JavaScript/TypeScript. The callback is called on the coroutine whenthencalled, and ifPromiseis passed
    from one coroutine to another and calledthenin both, then they are called in different coroutines (possibly
    concurrently). The developer must consider a possible data race, and take appropriate care.

Promise<U>::then<U, E = never>(onFulfilled: ((value:T) => U|PromiseLike<U>␣
˓→throws)|undefined, onRejected: ((error: Any) => E|PromiseLike<E>throws)|undefined):␣
˓→Promise<Awaited<U|E>>

- catchtakes one argument (the callback called after promise is rejected) and returnsPromise<Awaited<U|T>>

Promise<U>::catch<U = never>(onRejected?: (error: Any) => U|PromiseLike<U>throws):␣
˓→Promise<Awaited<T | U>>

- finallytakes one argument (the callback called afterpromiseis either fulfilled or rejected) and returns
    Promise<Awaited<T>>.

finally(onFinally?: () =>void throws): Promise<Awaited<T>>

**16.3. Asynchronous API 269**


#### 16.3.6 Unhandled Rejected Promises

In case of an unhandled rejection ofPromise, either the custom handler provided forPromiserejection is called, or
the defaultPromiserejection handler is called upon the entire program completion.

### 16.4 Coroutines (Experimental)

A function or lambda can be a _coroutine_. ArkTS supports _basic coroutines_ and _structured coroutines_. _Basic coroutines_
are used to create and launch a coroutine. The result is then to be awaited. Details are provided in _Standard Library_.

**270 Chapter 16. Concurrency**


##### CHAPTER

### SEVENTEEN

### EXPERIMENTAL FEATURES

This Chapter introduces the ArkTS features that are considered parts of the language, but have no counterpart in
TypeScript, and are therefore not recommended to those who seek a single source code for TypeScript and ArkTS.

Some features introduced in this Chapter are still under discussion. They can be removed from the final version of the
ArkTS specification. Once a feature introduced in this Chapter is approved and/or implemented, the corresponding
section is moved to the body of the specification as appropriate.

The _array creation_ feature introduced in _Resizable Array Creation Expressions_ enables users to dynamically create
objects of array type by using runtime expressions that provide the array size. This addition is useful to other array-
related features of the language, such as array literals. This feature can also be used to create arrays of arrays.

Overloading functions, methods, or constructors is a practical and convenient way to write program actions that are
similar in logic but different in implementation. ArkTS uses _Overload Declarations_ as an innovative form of _managed
overloading_.

Section _Native Functions and Methods_ introduces practically important and useful mechanisms for the inclusion of
components written in other languages into a program written in ArkTS.

Sections _Final Classes_ and _Final Methods_ discuss the well-known feature that in many OOP languages provides a way
to restrict class inheritance and method overriding. Making a class _final_ prohibits defining classes derived from it,
whereas making a method _final_ prevents it from overriding in derived classes.

Section _Adding Functionality to Existing Types_ discusses the way to add new functionality to an already defined type.

Section _Enumeration Methods_ adds methods to declarations of the enumeration types. Such methods can help in some
kinds of manipulations withenums.

The ArkTS language supports writing concurrent applications in the form of _coroutines_ (see _Coroutines (Experimen-
tal)_ ) that allow executing functions concurrently.

There is a basic set of language constructs that support concurrency. A function to be launched asynchronously is
marked by adding the modifierasyncto its declaration. In addition, any function or lambda expression can be launched
as a separate thread explicitly by using the launch function from the standard library.

### 17.1 Typechar

Values ofchartype are Unicode code points.

##### 271


```
Type Type’s Set of Values
char(32-bits) Symbols with codes from U+0000 to U+10FFFF (maximum valid Unicode code point) in-
clusive
```
```
Predefined constructors, methods, and constants forchartype are parts of the ArkTS Standard Library.
```
#### 17.1.1 Character Literals

```
Character literal represents the following:
```
- Value consisting of a single character; or
- Single escape sequence preceded by the characters _single quote_ (U+0027) and ‘ _c_ ’ (U+0063), and followed by a
    _single quote_ U+0027).
The syntax of _character literal_ is represented below:

```
CharLiteral:
'c\''SingleQuoteCharacter'\''
;
```
```
SingleQuoteCharacter:
~['\\\r\n]
|'\\' EscapeSequence
;
```
```
The examples are presented below:
```
1 c'a'
2 c'\n'
3 c'\x7F'
4 c'\u0000'

```
Character literals are of typechar.
```
#### 17.1.2 Character Equality Operators

```
Value equality is used for operands of typechar.
If both operands represent the same Unicode code point, then the result of ‘==’ or ‘===’ istrue. Otherwise, the
result isfalse.
```
```
272 Chapter 17. Experimental Features
```

### 17.2 Fixed-Size Array Types

```
Fixed-size array type , written asFixedArray<T>, is the built-in type characterized by the following:
```
- Any instance of array type contains elements. The number of elements is known as _array length_ , and can be
    accessed by using thelengthproperty.
- Array length is a non-negative integer number.
- Array length is set once at runtime and cannot be changed after that.
- Array element is accessed by its index. _Index_ is an integer number starting from _0_ to _array length minus 1_.
- Accessing an element by its index is a constant-time operation.
- If passed to a non-ArkTS environment, an array is represented as a contiguous memory location.
- Type of each array element is assignable to the element’s type specified in the array declaration (see _Assignabil-_
    _ity_ ).
_Fixed-size arrays_ differ from _resizable arrays_ as follows:
- Fixed-size array length is set once to achieve better performance;
- Fixed-size arrays have no methods defined;
- Fixed-size arrays have several constructors (see _Fixed-Size Array Creation_ );
- Fixed-size arrays are not compatible with _resizable arrays_.
Incompatibility between a resizable array and a fixed-size array is represented by the example below:

1 functionfoo(a: FixedArray<number>, b:Array<number>) {
2 a = b // compile-time error
3 b = a // compile-time error
4 }

#### 17.2.1 Fixed-Size Array Creation

```
Fixed-size array can be created by using Array Literal or constructors defined for typeFixedArray<T>, whereTmust
be a concrete type. A compile time error occurs ifTis a type parameter.
Using an array literal to create an array is represented in the example below:
```
1 leta :FixedArray<number> = [1, 2, 3]
2 /* create array with 3 elements of type number */
3 a[1] = 7/* put 7 as the 2nd element of the array, index of this element is 1 */
4 lety = a[2]/* get the last element of array 'a'*/
5 letcount = a.length// get the number of array elements
6 y = a[3]// Will lead to runtime error - attempt to access non-existing array element

```
Several constructors can be called to create aFixedArray<T>instance as follows:
```
- constructor(len: int), if typeThas either a default value (see _Default Values for Types_ ) or a constructor
    that can be called with no argument provided:

```
17.2. Fixed-Size Array Types 273
```

1 // type``number``has a default value:
2 leta =newFixedArray<number>(3)// creates array [0.0, 0.0, 0.0]
3
4 class C {
5 constructor(n?:number) {}
6 }
7 letb =newFixedArray<C>(2)// creates array [new C(), new C()]

- constructor(len: int, elem: T)for anyT. The constructor creates an array instance filled with a single
    valueelem:

1 leta =newFixedArray<string>(3, "a")// creates array ["a", "a", "a"]

- constructor(len: int, elems: (inx: int) => T)for anyT. The constructor creates an array in-
    stance where each _i_ element is evaluated as a result of theelemscall with argument _i_ :

1 leta =newFixedArray<int>(3, (inx: int) => 3 - inx )
2 // creates array [3, 2, 1]

```
New Expressions cannot use generic parameters to create a Fixed-size array. Attemptting to do so causes a compile-time
error as in the following example:
```
1 functionf<T>(): T {
2 letret =newFixedArray<T>(3) // compile-time error, generic parameter T
3 return ret
4 }

### 17.3 Resizable Array Creation Expressions

```
Array creation expression creates new objects that are instances of resizable arrays (see Resizable Array Types ). An
array instance can be created alternatively by using Array Literal.
The syntax of array creation expression is presented below:
```
```
newArrayInstance:
'new'arrayElementType dimensionExpression+ (arrayElement)?
;
```
```
arrayElementType:
typeReference
|'('type')'
;
```
```
dimensionExpression:
(continues on next page)
```
```
274 Chapter 17. Experimental Features
```

```
(continued from previous page)
'[' expression']'
;
```
```
arrayElement:
'(' expression')'
;
```
```
1 letx =new number[2][2]// create 2x2 matrix
```
```
Array creation expression creates an object that is a new array with the elements of the type specified by
arrayElelementType.
The type of each dimension expression must be assignable (see Assignability ) to aninttype. Otherwise, a compile-
time error occurs.
A compile-time error occurs if any dimension expression is a constant expression that is evaluated to a negative integer
value at compile time.
If the type of any dimension expression isnumberor other floating-point type, and its fractional part is other than ‘0’,
then errors occur as follows:
```
- Compile-time error, if the situation is identified during compilation; and
- Runtime error, if the situation is identified during program execution.
IfarrayElementis provided, then the type of theexpressioncan be as follows:
- Type of array element denoted byarrayElelementType, or
- Lambda function with the return type equal to the type of array element denoted byarrayElelementTypeand
the parameters of typeint, and the number of parameters equal to the number of array dimensions.
Otherwise, a compile-time error occurs.

```
1 letx =new number[-3]// compile-time error
2
3 lety =new number[3.141592653589] // compile-time error
4
5 foo (3.141592653589)
6 functionfoo (size:number) {
7 lety =new number[size] // runtime error
8 }
```
```
A compile-time error occurs ifarrayElelementTyperefers to a class that does not contain an accessible (see Ac-
cessible ) parameterless constructor, or constructor with all parameters of the second form of optional parameters (see
Optional Parameters ), or iftypehas no default value:
```
1 classC{
2 constructor(n:number) {}
3 }
4 letx =newC[3] // compile-time error: no parameterless constructor
5
6 classA {
7 constructor(p1?: number, p2?: string) {}
8 }
9 lety =newA[2] // OK, as all 3 elements of array will be filled with
10 // new A() objects

```
17.3. Resizable Array Creation Expressions 275
```

```
A compile-time error occurs ifarrayElelementTypeis a type parameter:
```
1 classA<T> {
2 foo() {
3 newT[2]// compile-time error: cannot create an array of type parameter␣
˓→elements
4 }
5 }

#### 17.3.1 Runtime Evaluation of Array Creation Expressions

```
The evaluation of an array creation expression at runtime is performed as follows:
```
1. The dimension expressions are evaluated. The evaluation is performed left-to-right. If any expression evaluation
    completes abruptly, then the expressions to the right of it are not evaluated.
2. The values of dimension expressions are checked. If the value of any dimension expression is less than zero,
    thenNegativeArraySizeErroris thrown.
3. Space for the new array is allocated. If the available space is not sufficient to allocate the array, then
    OutOfMemoryErroris thrown, and the evaluation of the array creation expression completes abruptly.
4. When an array with one dimension is created, each element of that array is initialized to its default value if type
    default value is defined ( _Default Values for Types_ ). If the default value for an element type is not defined, but the
    element type is a class type, then its _parameterless_ constructor is used to create the value of each element.
5. When array with several dimensions is created, the array creation effectively executes a set of nested loops of
    depth _n-1_.

### 17.4 Enumerations Experimental

```
Several experimental features described below are available for enumerations.
```
#### 17.4.1 Enumeration Methods

```
Several static methods are available to handle each enumeration type as follows:
```
- Methodstatic values()returns an array of enumeration constants in the order of declaration.
- Methodstatic getValueOf(name: string)returns an enumeration constant with the given name, or
    throws an error if no constant with such name exists.
- Methodstatic fromValue(value: T), whereTis the base type of the enumeration, returns an enumeration
    constant with a given value, or throws an error if no constant has such a value.

```
276 Chapter 17. Experimental Features
```

```
1 enumColor { Red, Green, Blue = 5 }
2 letcolors = Color.values()
3 //colors[0] is the same as Color.Red
4
5 letred = Color.getValueOf("Red")
6
7 Color.fromValue(5)// ok, returns Color.Blue
8 Color.fromValue(6)// throws runtime error
```
```
Additional methods for instances of an enumeration type are as follows:
```
- MethodvalueOf()returns a numeric orstringvalue of an enumeration constant depending on the type of the
    enumeration constant.
- MethodgetName()returns the name of an enumeration constant.

```
1 enumColor { Red, Green = 10, Blue }
2 letc:Color = Color.Green
3 console.log(c.valueOf())// prints 10
4 console.log(c.getName())// prints Green
```
```
Note. Methodsc.toString()andc.valueOf().toString()return the same value.
```
### 17.5 Indexable Types

```
If a class or an interface declares one or two functions with names$_getand$_set, and signatures (index: Type1):
Type2 and (index: Type1, value: Type2) respectively, then an indexing expression (see Indexing Expressions ) can be
applied to variables of such types:
```
```
1 class SomeClass {
2 $_get (index:number): SomeClass {return this}
3 $_set (index:number, value:SomeClass) { }
4 }
5 letx =newSomeClass
6 x = x[1]// This notation implies a call: x = x.$_get (1)
7 x[1] = x// This notation implies a call: x.$_set (1, x)
```
```
If only one function is present, then only the appropriate form of indexing expression (see Indexing Expressions ) is
available:
```
1 class ClassWithGet {
2 $_get (index:number): ClassWithGet {return this}
3 }
4 letgetClass = newClassWithGet
5 getClass = getClass[0]
6 getClass[0] = getClass // Error - no $_set function available
7
8 class ClassWithSet {
9 $_set (index:number, value:ClassWithSet) { }
10 }
(continues on next page)

```
17.5. Indexable Types 277
```

```
(continued from previous page)
```
11 letsetClass = newClassWithSet
12 setClass = setClass[0] // Error - no $_get function available
13 setClass[0] = setClass

```
Typestringcan be used as a type of the index parameter:
```
```
1 class SomeClass {
2 $_get (index:string): SomeClass {return this}
3 $_set (index:string, value:SomeClass) { }
4 }
5 letx =newSomeClass
6 x = x["index string"]
7 // This notation implies a call: x = x.$_get ("index string")
8 x["index string"] = x
9 // This notation implies a call: x.$_set ("index string", x)
```
```
Functions$_getand$_setare ordinary functions with compiler-known signatures. The functions can be used like
any other function. The functions can be abstract, or defined in an interface and implemented later. The functions
can be overridden and provide a dynamic dispatch for the indexing expression evaluation (see Indexing Expressions ).
The functions can be used in generic classes and interfaces for better flexibility. A compile-time error occurs if these
functions are marked asasync.
```
1 interface ReadonlyIndexable<K, V> {
2 $_get (index:K): V
3 }
4
5 interface Indexable<K, V>extendsReadonlyIndexable<K, V> {
6 $_set (index:K, value:V)
7 }
8
9 class IndexableByNumber<V>implements Indexable<number, V> {
10 privatedata:V[] = []
11 $_get (index:number): V {return this.data [index] }
12 $_set (index:number, value:V) {this.data[index] = value }
13 }
14
15 class IndexableByString<V>implements Indexable<string, V> {
16 privatedata =newMap<string, V>
17 $_get (index:string): V {return this.data [index] }
18 $_set (index:string, value:V) {this.data[index] = value }
19 }
20
21 class BadClassextendsIndexableByNumber<boolean> {
22 override$_set (index:number, value:boolean) { index / 0 }
23 }
24
25 letx: IndexableByNumber<boolean> =newBadClass
26 x[42] =true// This will be dispatched at runtime to the overridden
27 // version of the $_set method
28 x.$_get (15) // $_get and $_set can be called as ordinary
29 // methods

```
278 Chapter 17. Experimental Features
```

### 17.6 Iterable Types

```
A class or an interface is iterable if it implements the interfaceIterabledefined in the Standard Library , and thus has
an accessible parameterless method with the name$_iteratorand a return type that is a subtype (see Subtyping ) of
typeIteratoras defined in the Standard Library. It guarantees that an object returned by the$_iteratormethod
is of the type which implementsIterator, and thus allows traversing an object of the iterable type.
A union of iterable types is also iterable. It means that instances of such types can be used infor-ofstatements (see
for-of Statements ).
An iterable classCis represented in the example below:
```
1 classC implementsIterable<string> {
2 data: string[] = ['a','b', 'c']
3 $_iterator() { // Return type is inferred from the method body
4 return newCIterator(this)
5 }
6 }
7
8 classCIteratorimplements Iterator<string> {
9 index = 0
10 base: C
11 constructor(base: C) {
12 this.base = base
13 }
14 next(): IteratorResult<string> {
15 return{
16 done:this.index >= this.base.data.length,
17 value: this.index>= this.base.data.length? undefined :this.base.data[this.
˓→index++]
18 }
19 }
20 }
21
22 letc =newC()
23 for(letxof c) {
24 console.log(x)
25 }

```
In the example above, class C method $_iterator returns CIterator<string> that implements
Iterator<string>. If executed, this code prints out the following:
```
```
"a"
"b"
"c"
```
```
The method$_iteratoris an ordinary method with a compiler-known signature. This method can be used like any
other method. It can be abstract or defined in an interface to be implemented later. A compile-time error occurs if this
method is marked asasync.
Note. To support the code compatible with TypeScript, the name of the method$_iteratorcan be written as
[Symbol.iterator]. In this case, the classiterablelooks as follows:
```
```
17.6. Iterable Types 279
```

1 classC {
2 data: string[] = ['a','b', 'c'];
3 [Symbol.iterator]() {
4 return newCIterator(this)
5 }
6 }

```
The use of the name[Symbol.iterator]is considered deprecated. It can be removed in the future versions of the
language.
```
### 17.7 Callable Types

```
A type is callable if the name of the type can be used in a call expression. A call expression that uses the name of a
type is called a type call expression. Only class type can be callable. To make a type callable, a static method with the
name$_invokeor$_instantiatemust be defined or inherited:
```
1 class C {
2 static $_invoke() { console.log("invoked") }
3 }
4 C()// prints: invoked
5 C.$_invoke()// also prints: invoked

```
In the above example,C()is a type call expression. It is the short form of the normal method callC.$_invoke().
Using an explicit call is always valid for the methods$_invokeand$_instantiate.
Note. Only a constructor—not the methods$_invokeor$_instantiate—is called in a new expression :
```
1 class C {
2 static $_invoke() { console.log("invoked") }
3 constructor() { console.log("constructed") }
4 }
5 letx =newC()// constructor is called

```
The methods$_invokeand$_instantiateare similar but have differences as discussed below.
A compile-time error occurs if a callable type contains both methodsinvokeand$_instantiate.
```
#### 17.7.1 Callable Types with$_invokeMethod

```
The static method$_invokecan have an arbitrary signature. The method can be used in a type call expression in either
case above. If the signature has parameters, then the call must contain corresponding arguments.
```
1 class Add {
2 static $_invoke(a:number, b:number): number {
3 return a + b
4 }
(continues on next page)

```
280 Chapter 17. Experimental Features
```

```
(continued from previous page)
```
5 }
6 console.log(Add(2, 2)) // prints: 4

```
That a type contains the instance method$_invokedoes not make the type callable.
```
#### 17.7.2 Callable Types with$_instantiateMethod

```
The static method$_instantiatecan have an arbitrary signature by itself. If it is to be used in a type call expression ,
then its first parameter must be afactory(i.e., it must be a parameterless function type returning some class type ).
The method can have or not have other parameters, and those parameters can be arbitrary.
In a type call expression , the argument corresponding to thefactoryparameter is passed implicitly:
```
1 class C {
2 static $_instantiate(factory: () => C): C {
3 return factory()
4 }
5 }
6 letx = C()// factory is passed implicitly
7
8 // Explicit call of '$_instantiate'requires explicit'factory':
9 lety = C.$_instantiate(() => { return newC()})

```
If the method$_instantiatehas additional parameters, then the call must contain corresponding arguments:
```
1 class C {
2 name = ""
3 static $_instantiate(factory: () => C, name:string): C {
4 letx = factory()
5 x.name = name
6 return x
7 }
8 }
9 letx = C("Bob")// factory is passed implicitly

```
A compile-time error occurs in a type call expression with typeT, if:
```
- Thas neither method$_invokenor method$_instantiate; or
- Thas the method$_instantiatebut its first parameter is not afactory.

1 class C {
2 static $_instantiate(factory:string): C {
3 return factory()
4 }
5 }
6 letx = C()// compile-time error, wrong '$_instantiate'1st parameter

```
That a type contains the instance method$_instantiatedoes not make the type callable.
```
```
17.7. Callable Types 281
```

### 17.8 Statements

#### 17.8.1 For-of Explicit Type Annotation

```
An explicit type annotation is allowed for a ForVariable (see for-of Statements ):
```
```
1 // explicit type is used for a new variable,
2 letx:number[] = [1, 2, 3]
3 for(letn:number of x) {
4 console.log(n)
5 }
```
```
Type of elements in afor-ofexpression must be assignable (see Assignability ) to the type of the variable. Otherwise,
a compile-time error occurs.
```
### 17.9 Overload Declarations

```
ArkTS supports both the conventional overloading and an innovative form of managed overloading that allows a de-
veloper to fully control the order of selecting a specific entity to call from several overloaded entities Overloading.
The actual entity to be called is determined at compile time. Thus, overloading is related to the compile-time polymor-
phism by name. The semantic details are discussed in Overloading.
An overload declaration is used in managed overloading to define a set and an order of the overloaded entities (func-
tions, methods, or constructors).
An overload declaration can be used for:
```
- Functions (see _Function Declarations_ ), including functions in namespaces;
- Class or interface methods (see _Method Declarations_ and _Interface Method Declarations_ ); and
- _Ambient Declarations_.
An _overload declaration_ starts with the keywordoverloadand declares an _overload alias_ for a set of explicitly listed
entities as follows:

1 functionmax2(a:number, b:number): number{
2 return a > b? a :b
3 }
4 functionmaxN(...a: number[]):number {
5 // return max element
6 }
7
8 // declare 'max'as an ordered set of functions max2 and maxN
9 overload max { max2, maxN }
10
(continues on next page)

```
282 Chapter 17. Experimental Features
```

```
(continued from previous page)
```
11 max(1, 2) // max2 is called
12 max(3, 2, 4) // maxN is called
13 max("a", "b")// compile-time error, no function to call
14
15 maxN(1, 2) // maxN is explicitly called

```
The semantics of an entity included into an overload set does not change. Such entities follow the ordinary accessibility
rules, and can be used separately from an overload alias, e.g., called explicitly as follows:
```
```
1 maxN(1, 2) // maxN is explicitly called
2 max2(2, 3) // max2 is explicitly called
```
```
When calling an overload alias , entities from an overload set are checked in the listed order, and the first entity with
an appropriate signature is called (see Overload Resolution for detail). A compile-time error occurs if no entity with
an appropriate signature is available:
```
```
1 max(1) // maxN is called
2 max(1, 2) // max2 is called, as is the first in order
3
4 max("a", "b")// compile-time error, no function to call
```
```
It means that exactly one entity is selected for a call at the call site. Otherwise, a compile-time error occurs.
An overloaded entity in an overload declaration can be generic (see Generics ).
If during Overload Resolution type arguments are provided explicitly in a call of an overload alias (see Explicit Generic
Instantiations ), then consideration is given only to the entities that have an equal number of type parameters and type
arguments.
If type arguments are not provided explicitly (see Implicit Generic Instantiations ), then consideration is given to all
entities as represented in the example below:
```
```
1 functionfoo1(s:string) {}
2 functionfoo2<T>(x: T) {}
3
4 overload foo { foo1, foo2 }
5
6 foo("aa") // foo1 is called
7 foo(1) // foo2 is called, implicit generic instantiation
8 foo<string>("aa")// foo2 is called
```
```
An entity can be listed in several overload declarations :
```
1 functionmax2i(a:int, b:int): int{
2 return a > b? a :b
3 }
4 functionmaxNi(...a:int[]):int{
5 // return max element
6 }
7 functionmaxN(...a: number[]):number {
8 // return max element
9 }
10
11 overload maxi { max2i, maxNi }
12 overload max { max2i, maxNi, maxN }

```
17.9. Overload Declarations 283
```

#### 17.9.1 Function Overload Declarations

```
Function overload declaration allows declaring an overload alias for a set of functions (see Function Declarations ).
The syntax is presented below:
```
```
overloadFunctionDeclaration:
'overload'identifier '{' qualifiedName(','qualifiedName)* ','?'}'
;
```
```
A compile-time error occurs, if a qualified name does not refer to an accessible function.
A compile-time error occurs, if an overload alias is exported but an overloaded function is not:
```
```
1 export function foo1(p:string) {}
2 functionfoo2(p:number) {}
3 export overload foo { foo1, foo2 }// compile-time error, 'foo2'is not exported
4 overload bar { foo1, foo2 } // ok, as'bar'is not exported
```
```
All overloaded functions must be in the same module or namespace scope (see Scopes ). Otherwise, a compile-time
error occurs. The erroneous overload declarations are represented in the example below:
```
1 import {foo1}from "something"
2
3 functionfoo2() {}
4 overload foo {foo1, foo2}// compile-time error
5
6 namespace N {
7 export function fooN() {}
8 namespace M {
9 export function fooM() {}
10 }
11 overload goo {M.fooM, fooN} // compile-time error
12 }
13 overload bar {foo2, N.fooN} // compile-time error

#### 17.9.2 Class Method Overload Declarations

```
Method overload declaration allows declaring an overload alias as a class member (see Class Members ) for a set of
static or instance methods (see Method Declarations ). The syntax is presented below:
```
```
overloadMethodDeclaration:
overloadMethodModifier*
'overload'identifier '{' identifier(',' identifier)* ','? '}'
;
```
```
overloadMethodModifier: 'static' |'async';
```
```
Using method overload declaration and calling an overload alias are represented in the example below:
```
```
284 Chapter 17. Experimental Features
```

```
1 class Processor {
2 overload process { processNumber, processString }
3 processNumber(n:number) {/*body*/}
4 processString(s:string) {/*body*/}
5 }
6
7 letc =newC()
8 c.process(42)// calls processNumber
9 c.process("aa") // calls processString
```
```
Static overload alias is represented in the example below:
```
```
1 class C {
2 static one(n:number) {/*body*/}
3 static two(s:string) {/*body*/}
4 static overload foo { one, two }
5 }
```
```
A compile-time error occurs if:
```
- Method modifier is used more than once in an method overload declaration;
- _Identifier_ in the overloaded method list does not refer to an accessible method (either declared or inherited) of
    the current class;
- _Overload alias_ is:
    **-** _Static_ but the overloaded method is not;
    **-** _Non-static_ but the overloaded method is not;
    **-** Markedasyncbut the overloaded method is not; or
    **-** Notasyncbut the overloaded method is.
_Overload alias_ and overloaded methods can have different access modifiers. A compile-time error occurs if the _overload
alias_ is:
- publicbut at least one overloaded method is notpublic;
- protectedbut at least one overloaded method isprivate.
Valid and invalid overload declarations are represented in the example below:

1 class C {
2 privatefoo1(x:number) {/*body*/}
3 protectedfoo2(x:string) {/*body*/}
4 public foo3(x:boolean) {/*body*/}
5 foo4() {/*body*/}// implicitly public
6
7 public overload foo { foo3, foo4 }// ok
8 protectedoverload bar { foo2, foo3 } // ok
9 privateoverload goo { foo1, foo2, foo3 }// ok
10
11 public overload err1 {foo2, foo3}// compile-time error, foo2 is not public
12 protectedoverload err2 {foo2, foo1} // compile-time error, foo1 is private
13 }

```
Some or all overloaded functions can benativeas follows:
```
```
17.9. Overload Declarations 285
```

```
1 class C {
2 native foo1(x:number)
3 foo2(x:string) {/*body*/}
4 overload foo { foo1, foo2 }
5 }
```
```
If a superclass has an overload declaration , then this declaration can be overridden in a subclass. If a subclass does not
override an overload declaration , then the declaration from the superclass is inherited.
If a subclass overrides an overload declaration , then this declaration must list all methods of the overload declaration
in a superclass. Otherwise, a compile-time error occurs.
In addition, overriding an overload declaration in a subclass can include new methods and change the order of all
methods in the overload declaration.
An overload alias is used like an ordinary class method except that it is replaced in a call at compile time for one of
overloaded methods that use the type of object reference. The overload declaration in subtypes is represented in the
example below:
```
1 class Base {
2 overload process { processNumber, processString }
3 processNumber(n:number) {/*body*/}
4 processString(s:string) {/*body*/}
5 }
6
7 class D1extendsBase {
8 // method is overridden
9 overrideprocessNumber(n:number) {/*body*/}
10 // overload declaration is inherited
11 }
12
13 class D2extendsBase {
14 // method is added:
15 processInt(n:int) {/*body*/}
16 // new order for overloaded methods is specified:
17 overload process { processInt, processNumber, processString }
18 }
19
20 newD1().process(1) // calls processNumber from D1
21
22 newD2().process(1) // calls processInt from D2 (as it is listed earlier)
23 newD2().process(1.0)// calls processNumber from Base (first appropriate)

```
Methods with special names (see Indexable Types , Iterable Types , and Callable Types ) can be overloaded like ordinary
methods:
```
```
1 class C {
2 getByNumber(n: number): string{...}
3 getByString(s: string): string{...}
4 overload $_get { getByNumber, getByString }
5 }
6
7 letc =newC()
8
(continues on next page)
```
```
286 Chapter 17. Experimental Features
```

(continued from previous page)
9 c[1] // getByNumber is used
10 c["abc"]// getByString is used

```
If a class implements some interfaces with overload declarations for the same alias, then a new overload declaration
must include all overloaded methods. Otherwise, a compile-time error occurs.
```
1 interface I1 {
2 overload foo {f1, f2}
3 // f1 and f2 are declared in I1
4 }
5 interface I2 {
6 overload foo {f3, f4}
7 // f3 and f4 are declared in I2
8 }
9 class Cimplements I1, I2 {
10 // compile-time error as no new overload is defined
11 }
12 class Dimplements I1, I2 {
13 overload foo { f2, f3, f1, f4 } // OK, as new overload is defined
14 }
15 class Eimplements I1, I2 {
16 overload foo { f2, f4 } // compile-time error as not all methods are used
17 }
18
19 const i1:I1= newD
20 i1.foo(<arguments>) // call is valid if arguments fit first signature of {f1, f2} set
21
22 const i2:I2= newD
23 i2.foo(<arguments>) // call is valid if arguments fit first signature of {f3, f4} set
24
25 const d:D =newD
26 d.foo(<arguments>) // call is valid if arguments fit first signature of {f2, f3, f1, f4}
˓→set

#### 17.9.3 Interface Method Overload Declarations

```
Interface method overload declaration allows declaring an overload alias as an interface member (see Interface Mem-
bers ) for a set of interface methods (see Interface Method Declarations ).
The syntax is presented below:
```
```
overloadInterfaceMethodDeclaration:
'overload'identifier '{' identifier(',' identifier)* ','? '}'
;
```
```
The use of a method overload declaration is represented in the example below:
```
```
1 interface I {
2 foo(): void
(continues on next page)
```
```
17.9. Overload Declarations 287
```

(continued from previous page)
3 bar(n?:string): void
4 overload goo { foo, bar }
5 }
6
7 functionexample(i: I) {
8 i.goo() // calls i.foo()
9 i.goo("hello") // calls i.bar("hello")
10 i.bar() // explicit call: i.bar(undefined)
11 }

```
An overload alias is used like an ordinary interface method, except that in a call it is replaced at compile time by one
of overloaded methods by using the type of object reference.
A class that implements an interface with an overload alias usually implements all interface methods, except those
having a default body (see Default Interface Method Declarations ):
```
```
1 // Using interface overload declaration
2 classC implementsI {
3 foo(): void{/*body*/}
4 bar(n?:string): void{/*body*/}
5 }
6
7 letc =newC()
8 c.goo()// calls c.foo()
```
```
An interface overload alias can be overridden in a class. In this case, the overload declaration in the class must contain
all methods overloaded in the interface. Otherwise, a compile-time error occurs.
```
```
1 classD implementsI {
2 foo(): void{/*body*/}
3 bar(n?:string): void{/*body*/}
4 overload goo( bar, foo) // order is changes
5 }
6
7 letd =newD()
8 d.goo()// d.bar(undefined) is used, as it is the first appropriate method
```
```
An overload alias defined in a superinterface can be overridden in a subinterface. In this case, the overload declaration
of the subinterface must contain all methods overloaded in superinterface. Otherwise, a compile-time error occurs.
The overload alias defined in superinterfaces must be overridden in a subinterface if several overload declarations for
the same alias are inherited into the interface, otherwise a compile-time error occurs.
```
1 interface I1 {
2 overload foo {f1, f2}
3 // f1 and f2 are declared in I1
4 }
5 interface I2 {
6 overload foo {f3, f4}
7 // f3 and f4 are declared in I2
8 }
9 interface I3extendsI1, I2 {
10 // compile-time error as no new overload for'foo'is defined
11 }
(continues on next page)

```
288 Chapter 17. Experimental Features
```

```
(continued from previous page)
```
12 interface I4extendsI1, I2 {
13 overload foo { f4, f1, f3, f2 } // OK, as new overload is defined
14 }
15 interface I5extendsI1, I2 {
16 overload foo { f1, f3 } // compile-time error as not all methods are included
17 }

#### 17.9.4 Constructor Overload Declarations

```
Constructor overload declaration allows declaring an overload alias and setting an order of constructors for a call in a
new expression.
The syntax is presented below:
```
```
overloadConstructorDeclaration:
'overload' 'constructor' '{' identifier(','identifier)*','?'}'
;
```
```
This feature can be used if there are more then one constructors declared in the class, and maximum one of them is
anonymous (see Constructor Names ).
Only a single constructor overload declaration is allowed in a class. Otherwise, a compile-time error occurs.
Overload alias for constructors is used the same way as anonymous constructor (see New Expressions ).
The use of a constructor overload declaration is represented in the example below:
```
```
1 class BigFloat {
2 constructorfromNumber(n:number) {/*body1*/}
3 constructorfromString(s:string) {/*body2*/}
4
5 overloadconstructor{ fromNumber, fromString }
6 }
7
8 newBigFloat(1) // fromNumber is used
9 newBigFloat("3.14")// fromString is used
```
```
If a class has an anonymous constructor it is implicitly placed at first position in a list of overloaded constructors:
```
1 class C {
2 constructor() {/*body*/}
3 constructorfromString(s?:string) {/*body*/}
4
5 overloadconstructor{ fromString }
6 }
7
8 newC() // anonymous constructor is used
9 newC("abc") // fromString is used
10 newC.fromString("aa") // fromString is explicitly used

```
17.9. Overload Declarations 289
```

#### 17.9.5 Overload Alias Name Same As Function Name

```
A name of a top-level overload declaration can be the same as the name of an overloaded function. This situation is
represented in the following example:
```
1 functionfoo(n: number): number {/*body1*/}
2 functionfooString(s:number):string {/*body2*/}
3
4 overload foo {foo, fooString}
5
6 foo(1) // overload alias is used to call'foo'
7 foo("aa") // overload alias is used to call'fooString'

```
Using an overload alias causes no ambiguity for it is considered at the call site only, i.e., an overload alias is not
considered in the following situations:
```
- List of the overloaded entities (see _Function Overload Declarations_ );
- _Function Reference_.

1 functionfoo(n: number): number {/*body1*/}
2 functionfooString(s:number):string {/*body2*/}
3 overload foo {foo, fooString}
4
5 letfunc1 = foo// function'foo'is used, not overload alias

```
If the name of an overload alias is the same as the name of a function that is not listed as an overloaded function, then
a compile-time error occurs as follows:
```
1 functionfoo(n: number) {/*body1*/}
2 functionfooString(s:number) {/*body2*/}
3 functionfooBoolean(b: boolean) {/*body3*/}
4
5 overload foo { // compile-time error
6 fooBoolean, fooString
7 }

#### 17.9.6 Overload Alias Name Same As Method Name

```
A name of a class or interface overload declaration can be the same as the name of an overloaded method. As one
example, a method defined in a superclass can be used as one of overloaded methods in a same-name subclass overload
declaration. This important case is represented by the following example:
```
1 class C {
2 foo(n: number): number{/*body*/}
3 }
4 class Dimplements C {
5 fooString(s:number):string {/*body*/}
(continues on next page)

```
290 Chapter 17. Experimental Features
```

(continued from previous page)
6
7 overload foo {
8 foo,// method 'foo'from C
9 fooString
10 }
11 }
12
13 letd =newD()
14 letc: C= d
15
16 d.foo(1) // overload alias is used to call'foo'from C
17 d.foo("aa")// overload alias is used to call'fooString'from D
18 c.foo(1) // method'foo'from is called (no overload)

```
If names of a method and of an overload alias are the same, then the method can be overridden as usual:
```
```
1 class C {
2 foo(n: number): number{/*body*/}
3 }
4 class Dimplements C {
5 foo(n: number): number{/*body*/} // method is overridden
6 fooString(s:number):string {/*body*/}
7
8 overload foo { foo, fooString }
9 }
```
```
This feature is also valid in interfaces, or in an interface and a class that implements the interface:
```
1 interface I {
2 foo(n: number): number{/*body*/}
3 }
4 interface JextendsI {
5 fooString(s:number):string
6 overload foo { foo, fooString }
7 }
8
9 class Kimplements I {
10 foo(n: number): number{/*body*/}
11 fooString(s:number):string {/*body*/}
12
13 overload foo { foo, fooString }
14 }

```
Using an overload alias causes no ambiguity for it is considered at the call site only. An overload alias is not considered
in the following situations:
```
- _Overriding_ ;
- List of the overloaded entities (see _Class Method Overload Declarations_ and _Interface Method Overload Decla-_
    _rations_ );
- _Method Reference_.

```
17.9. Overload Declarations 291
```

1 class C {
2 foo(n: number): number{/*body*/}
3 }
4
5 class Dimplements C {
6 fooString(s:number):string {/*body*/}
7
8 overload foo { foo, fooString }
9 }
10
11 letd =newD()
12 letc: C= d
13
14 letfunc1 = c.foo// method 'foo'is used
15 letfunc2 = d.foo// method 'foo'is used, not overload alias

```
A compile-time error occurs if the name of an overload alias is the same as the name of a method (with the same static
or non-static modifier) that is not listed as an overloaded method as follows:
```
```
1 class C {
2 foo(n: number) {/*body*/}
3 fooString(s:number) {/*body*/}
4 fooBoolean(b:boolean) {/*body*/}
5
6 overload foo { // compile-time error
7 fooBoolean, fooString
8 }
9 }
```
### 17.10 Native Functions and Methods

#### 17.10.1 Native Functions

```
Native function is a function marked with the keywordnative(see Function Declarations ).
Native function implemented in a platform-dependent code is typically written in another programming language (e.g.,
C ). A compile-time error occurs if a native function has a body.
```
```
292 Chapter 17. Experimental Features
```

#### 17.10.2 Native Methods

_Native method_ is a method marked with the keywordnative(see _Method Declarations_ ).

_Native methods_ are the methods implemented in a platform-dependent code written in another programming language
(e.g., _C_ ).

A compile-time error occurs if:

- Method declaration contains the keywordabstractalong with the keywordnative.
- _Native method_ has a body (see _Method Body_ ) that is a block instead of a simple semicolon or empty body.

#### 17.10.3 Native Constructors

_Native constructor_ is a constructor marked with the keywordnative(see _Constructor Declaration_ ).

_Native constructors_ are the constructors implemented in a platform-dependent code written in another programming
language (e.g., _C_ ).

A compile-time error occurs if a _native constructor_ has a non-empty body (see _Constructor Body_ ).

### 17.11 Classes Experimental

#### 17.11.1 Final Classes

A class can be declaredfinalto prevent extension, i.e., a class declaredfinalcan have no subclasses. No method
of afinalclass can be overridden.

If a class typeFexpression is declared _final_ , then only a classFobject can be its value.

A compile-time error occurs if theextendsclause of a class declaration contains another class that isfinal.

#### 17.11.2 Final Methods

A method can be declaredfinalto prevent it from being overridden (see _Overriding Methods_ ) in subclasses.

A compile-time error occurs if:

- The method declaration contains the keywordabstractorstaticalong with the keywordfinal.
- A method declaredfinalis overridden.

**17.11. Classes Experimental 293**


#### 17.11.3 Constructor Names

```
A Constructor Declaration allows a developer to set a name used to explicitly specify constructor to call in New Ex-
pressions :
```
1 class Temperature{
2 // use specified scale:
3 constructorCelsius(n:double) {/*body1*/}
4 constructorFahrenheit(n:double) {/*body2*/}
5 }
6
7 newTemperature.Celsius(0)
8 newTemperature.Fahrenheit(32)

```
If a constructor has a name, then using the constructor directly in a new expression implies using the constructor name
explicitly:
```
1 class X{
2 constructorctor1(p:number) {/*body1*/}
3 constructorctor2(p:string) {/*body2*/}
4 }
5
6 newX(1) // compile-time error
7 newX("abs") // compile-time error
8 newX.ctor1(1) // OK
9 newX.ctor2("abs") // OK

```
A compile-time error occurs if a constructor name is used as a named reference (see Named Reference ) in any expres-
sion.
```
1 class X{
2 constructorfoo() {}
3 }
4 const func = X.foo// Compile-time error

```
The feature is also important for Constructor Overload Declarations.
```
### 17.12 Default Interface Method Declarations

```
The syntax of interface default method is presented below:
```
```
interfaceDefaultMethodDeclaration:
'private'?identifier signature block
;
```
```
A default method can be explicitly declaredprivatein an interface body.
A block of code that represents the body of a default method in an interface provides a default implementation for any
class if such a class does not override the method that implements the interface.
```
```
294 Chapter 17. Experimental Features
```

### 17.13 Adding Functionality to Existing Types

```
ArkTS supports adding functions and accessors to already defined types. The usage of functions so added looks the
same as if they are methods and accessors of these types. The mechanism is called Functions with Receiver and
Accessors with Receiver. This feature is often used to add new functionality to a class or an interface without having
to inherit from the class or to implement the interface. However, it can be used not only for classes and interfaces but
also for other types.
Moreover, Function Types with Receiver and Lambda Expressions with Receiver can be defined and used to make the
code more flexible.
```
#### 17.13.1 Functions with Receiver

```
Function with receiver declaration is a top-level declaration (see Top-Level Declarations ) that looks almost the same
as Function Declarations , except that the first mandatory parameter uses keywordthisas its name.
The syntax of function with receiver is presented below:
```
```
functionWithReceiverDeclaration:
'function'identifier typeParameters? signatureWithReceiver block
;
```
```
signatureWithReceiver:
'(' receiverParameter(', 'parameterList)? ')'returnType?
;
```
```
receiverParameter:
annotationUsage? 'this' ':'type
;
```
```
Function with receiver can be called in the following two ways by making:
```
- Ordinary function call (see _Function Call Expression_ ) when the first argument is the receiver object;
- Method call (see _Method Call Expression_ ) when the receiver is anobjectReferencebefore the function name
    passed as the first argument of the call.
All other arguments are handled in an ordinary manner.
**Note**. Derived classes or interfaces can be used as receivers.

1 classC {}
2
3 functionfoo(this:C) {}
4 functionbar(this:C, n:number): void{}
5
6 letc =newC()
7
8 // as a function call:
(continues on next page)

```
17.13. Adding Functionality to Existing Types 295
```

(continued from previous page)
9 foo(c)
10 bar(c, 1)
11
12 // as a method call:
13 c.foo()
14 c.bar(1)
15
16 interfaceD {}
17 functionfoo1(this:D) {}
18 functionbar1(this:D, n:number): void{}
19
20 functiondemo (d:D) {
21 // as a function call:
22 foo1(d)
23 bar1(d, 1)
24
25 // as a method call:
26 d.foo1()
27 d.bar1(1)
28 }
29
30 classE implementsD {}
31 conste =newE
32
33 // derived class is used as a receiver for a method call:
34 e.foo1()
35 e.bar1(1)
36
37 // the same as a function call:
38 foo1(e)
39 bar1(e, 1)

```
The keywordthiscan be used inside a function with receiver. It corresponds to the first parameter. Otherwise, a
compile-time error occurs. The type of parameterthisis called the receiver type (see Receiver Type ).
If the receiver type is a class or interface type, thenprivateorprotectedmembers are not accessible (see Accessible )
within the body of a function with receiver. Onlypublicmembers can be accessed:
```
1 classA {
2 foo () { ...this.bar() ... }
3 // function bar() is accessible here
4 protectedmember_1 ...
5 privatemember_2 ...
6 }
7 functionbar(this:A) { ...
8 this.foo() // Method foo() is accessible as it is public
9 this.member_1 // Compile-time error as member_1 is not accessible
10 this.member_2 // Compile-time error as member_2 is not accessible
11 ...
12 }
13 leta =newA()
14 a.foo()// Ordinary class method is called
15 a.bar()// Function with receiver is called

```
296 Chapter 17. Experimental Features
```

```
A compile-time error occurs if the name of a function with receiver is the same as the name of an accessible (see
Accessible ) instance method or field of the receiver type:
```
1 classA {
2 foo () { ... }
3 }
4 functionfoo(this:A) { ... } // Compile-time error to prevent ambiguity below
5 (newA).foo()

```
A compile-time error occurs if an attempt is made to call a function with receiver from a derived class variable:
```
1 classB extendsA {}
2 constb =newB
3 b.foo() // Compile-time error
4 foo (b) // OK

```
Function with receiver cannot have the same name as a global function. Otherwise, a compile-time error occurs.
```
1 functionfoo(this:A) { ... }
2 functionfoo() { ... }// Compile-time error

```
Function with receiver can be generic as in the following example:
```
1 function foo<T>(this:B<T>, p:T) {
2 console.log (p)
3 }
4 function demo (p1:B<SomeClass>, p2:B<BaseClass>) {
5 p1.foo(newSomeClass())
6 // Type inference should determine the instantiating type
7 p2.foo<BaseClass>(newDerivedClass())
8 // Explicit instantiation
9 }

```
Functions with receiver are dispatched statically. What function is being called is known at compile time based on the
receiver type specified in the declaration. A function with receiver can be applied to the receiver of any derived class
until it is overridden within the derived class:
```
1 classBase { ... }
2 classDerivedextendsBase { ... }
3
4 functionfoo(this:Base) { console.log ("Base.foo is called") }
5
6 letb:Base= newBase()
7 b.foo()//`Base.foo is called`to be printed
8 b =newDerived()
9 b.foo()//`Base.foo is called`to be printed

```
A function with receiver can be defined in a module other than the one that defines the receiver type. This is represented
in the following examples:
```
1 // file a.ets
2 classA {
3 foo() { ... }
4 }
(continues on next page)

```
17.13. Adding Functionality to Existing Types 297
```

(continued from previous page)
5
6 // file ext.ets
7 import{A}from "a.ets"// name'A'is imported
8 functionbar(this:A) () {
9 this.foo() // Method foo() is called
10 }

#### 17.13.2 Receiver Type

```
Receiver type is the type of the receiver parameter in a function, function type, and lambda with receiver. A receiver
type may be an interface type, a class type, an array type, or a type parameter. Otherwise, a compile-time error occurs.
The use of array type as receiver type is presented in the example below:
```
```
1 functionaddElements(this:number[], ...s: number[]) {
2 ...
3 }
4
5 letx:number[] = [1, 2]
6 x.addElements(3, 4)
```
#### 17.13.3 Accessors with Receiver

```
Note. Accessor declarations at the top level or in namespaces are of the following two kinds:
```
- _Accessors with Receiver_ (as described in this subsection) that can be used much like fields of a class; and
_Accessor with receiver_ declaration is either a top-level declaration (see _Top-Level Declarations_ ), or a declaration inside
a namespace (see _Namespace Declarations_ ) that can be used as class (see _Class Accessor Declarations_ ) or interface
accessor (see _Interface Properties_ ) for a specified receiver type:
The syntax of _accessor with receiver_ is presented below:

```
accessorWithReceiverDeclaration:
'get' identifier'('receiverParameter')'returnType block
|'set' identifier'('receiverParameter','parameter ')' block
;
```
```
The keywordthiscan be used inside a function with receiver. It corresponds to the first parameter. Otherwise, a
compile-time error occurs. The type of parameterthisis called the receiver type (see Receiver Type ).
If the receiver type is a class type or an interface type, thenprivateorprotectedmembers are not accessible (see
Accessible ) within the body of a function with receiver. Onlypublicmembers can be accessed:
A get-accessor (getter) must have the keywordthisas the only getter parameter ( receiverParameter ) and an explicit
return type.
```
```
298 Chapter 17. Experimental Features
```

```
A set-accessor (setter) must have a keywordthisas a first setter parameter ( receiver parameter ), one other parameter,
and no return type.
The keywordthishas the same meaninng and can be used in the same manner as described in Functions with Receiver :
```
- The keywordthiscan be used inside an _accessor with receiver_. It corresponds to the first parameter. Otherwise,
    a compile-time error occurs.
- The type of parameterthisis called the _receiver type_ (see _Receiver Type_ ).
- If the _receiver type_ is a class or interface type, thenprivateorprotectedmembers are not accessible (see
    _Accessible_ ) within the body of a _function with receiver_. Onlypublicmembers can be accessed.
**Note**. If the _accessor with receiver_ is an entity of a namespace, then the same rules apply to it when exporting and
using qualified names as the rules that apply to other namespace entities (see _Namespace Declarations_ ).
The use of getters and setters looks the same as the use of fields:

1 classPerson {
2 firstName:string
3 lastName:string
4 constructor(first:string, last:string) {
5 this.firstName = first
6 this.lastName = last
7 }
8 }
9
10 get fullName(this:Person):string {
11 return this.lastName +' '+ this.firstName
12 }
13
14 letc =newPerson("John", "Doe")
15
16 // Getter - ok, top=level getter with receiver used
17 console.log(c.fullName) // output:'Doe John'
18
19 // compile-time error, as setter is not defined
20 c.fullName = "new name"

```
A compile-time error occurs if an accessor is used in the form of a function or a method call.
```
#### 17.13.4 Function Types with Receiver

```
Function type with receiver specifies the signature of a function or lambda with receiver. It is almost the same as
function type (see Function Types ), except that the first parameter is mandatory, and the keywordthisis used as its
name:
The syntax of function type with receiver is presented below:
```
```
functionTypeWithReceiver:
'(' receiverParameter(','ftParameterList)?')'ftReturnType
;
```
```
The type of a receiver parameter is called the receiver type (see Receiver Type ).
```
```
17.13. Adding Functionality to Existing Types 299
```

```
1 classA {...}
2
3 typeFA = (this:A) =>boolean
4 typeFN = (this:number[], max:number) => number
```
```
Function type with receiver can be generic as in the following example:
```
```
1 classB<T> {...}
2
3 typeFB<T> = (this:B<T>, x:T):void
4 typeFBS = (this:B<string>, x:string): void
```
```
The usual rule of function type compatibility (see Subtyping for Function Types ) is applied to function type with receiver ,
and parameter names are ignored.
```
1 classA {...}
2
3 typeF1 = (this:A) =>boolean
4 typeF2 = (a:A) =>boolean
5
6 functionfoo(this:A): boolean{}
7 functiongoo(a:A):boolean{}
8
9 letf1:F1= foo // ok
10 f1 = goo// ok
11
12 letf2:F2= goo // ok
13 f2 = foo// ok
14 f1 = f2// ok

```
The sole difference is that only an entity of function type with receiver can be used in Method Call Expression. The
declarations from the previous example are reused in the example below:
```
```
1 leta =newA()
2 a.f1()// ok, function type with receiver
3 f1(a) // ok
4
5 a.f2()// compile-time error
6 f2(a)// ok
```
#### 17.13.5 Lambda Expressions with Receiver

```
Lambda expression with receiver defines an instance of a function type with receiver (see Function Types with Receiver ).
It looks almost the same as an ordinary lambda expression (see Lambda Expressions ), except that the first parameter is
mandatory, and the keywordthisis used as its name:
The syntax of lambda expression with receiver is presented below:
```
```
lambdaExpressionWithReceiver:
annotationUsage?
(continues on next page)
```
```
300 Chapter 17. Experimental Features
```

```
(continued from previous page)
'(' receiverParameter(','lambdaParameterList)?')'
returnType? '=>' lambdaBody
;
```
```
The usage of annotations is discussed in Using Annotations.
The keywordthiscan be used inside a lambda expression with receiver , It corresponds to the first parameter:
```
```
1 classA { name = "Bob" }
2
3 letshow = (this:A): void{
4 console.log(this.name)
5 }
```
```
Lambda can be called in two syntactical ways represented by the example below:
```
1 classA {
2 name: string
3 constructor(n:string) {
4 this.name = n
5 }
6 }
7
8 functionfoo(aa:A[], f: (this:A) =>void) {
9 for(leta of aa) {
10 a.f()// first way
11 f (a)// second way
12 }
13 }
14
15 letaa:A[] = [newA("aa"),newA("bb")]
16 foo(aa, (this:A) => { console.log(this.name)} )// output: "aa" "bb"

```
Note. If lambda expression with receiver is declared in a class or interface, thenthisuse in the lambda body refers to
the first lambda parameter and not to the surrounding class or interface. Any lambda call outside a class has to use the
ordinary syntax of arguments as represented by the example below:
```
1 classB {
2 foo() { console.log ("foo() from B is called") }
3 }
4 classA {
5 foo() { console.log ("foo() from A is called") }
6 bar() {
7 letlambda1 = (this:B): void=> {this.foo() }// local lambda
8 newB().lambda1()
9 }
10 lambda2 = (this:B): void=> {this.foo() }// class field lambda
11 }
12 newA().bar()// Output is'foo() from B is called'
13 newA().lambda2 (newB)// Argument is to be provided in its usual place
14
15 interfaceI {
16 lambda: (this: B) =>void// Property of the function type
(continues on next page)

```
17.13. Adding Functionality to Existing Types 301
```

```
(continued from previous page)
```
17 }
18 functionfoo (i:I) {
19 i.lambda(newB) // Argument is to be provided in its usual place
20 }

#### 17.13.6 Implicitthisin Lambda with Receiver Body

```
Implicitthiscan be used in the body of lambda expression with receiver when accessing the following:
```
- Instance methods, fields, and accessors of lambda receiver type (see _Receiver Type_ ); or
- Functions with receiver (see _Functions with Receiver_ ) of the same receiver type.
In other words, prefixthis.in such cases can be omitted. This feature is added to ArkTS to improve DSL support. It
is represented in the following examples:

1 class C {
2 name:string = ""
3 foo(): void{}
4 }
5
6 function process(context: (this:C) =>void) {}
7
8 process(
9 (this: C): void=> {
10 this.foo() // ok - normal call
11 foo() // ok - implicit'this'
12 name = "Bob"// ok - implicit'this'
13 }
14 )

```
The same applies if lambda expression with receiver is defined as trailing lambda (see Trailing Lambdas ). In this case,
lambda signature is inferred from the context:
```
```
1 process() {
2 this.foo() // ok - normal call
3 foo() // ok - implicit'this'
4 }
```
```
The example above represents the use of implicitthiswhen calling a function with receiver:
```
```
1 function bar(this:C) {}
2 function otherBar(this:OtherClass) {}
3
4 process() {
5 bar() // ok - implicit'this'
6 otherBar() // compile-time error, wrong type of implicit'this'
7 }
```
```
If a simple name used in a lambda body can be resolved as instance method, field, or accessor of the receiver type,
and as another entity in the current scope at the same time, then a compile-time error occurs to prevent ambiguity and
```
```
302 Chapter 17. Experimental Features
```

```
improve readability.
```
### 17.14 Trailing Lambdas

```
The trailing lambda is a special form of notation for function or method call when the last parameter of a function or
a method is of function type, and the argument is passed as a lambda using the Block notation. The trailing lambda
syntactically looks as follows:
```
```
1 classA {
2 foo (f: ()=>void) { ... }
3 }
4
5 leta =newA()
6 a.foo() { console.log ("method lambda argument is activated") }
7 // method foo receives last argument as the trailing lambda
```
```
The syntax of trailing lambda is presented below:
```
```
trailingLambdaCall:
(objectReference '.'identifier typeArguments?
|expression ('?.'| typeArguments)?
)
arguments block
;
```
```
Currently, no parameter can be specified for the trailing lambda, except a receiver parameter (see Lambda Expressions
with Receiver ). Otherwise, a compile-time error occurs.
A block immediately after a call is always handled as trailing lambda. A compile-time error occurs if the last parameter
of the called entity is not of a function type.
The semicolon ‘;’ separator can be used between a call and a block to indicate that the block does not define a trailing
lambda. When calling an entity with the last optional parameter (see Optional Parameters ), it means that the call must
use the default value of the parameter.
```
1 functionfoo (f: ()=>void) { ... }
2
3 foo() { console.log ("trailing lambda") }
4 //'foo'receives last argument as the trailing lambda
5
6 functionbar(f?: ()=>void) { ... }
7
8 bar() { console.log ("trailing lambda") }
9 // function'bar'receives last argument as the trailing lambda,
10 bar(); { console.log ("that is the block code") }
11 // function'bar'is called with parameter'f'set to'undefined'
12
13 functiongoo(n:number) { ... }
14
15 goo() { console.log("aa") }// compile-time error as goo() requires an argument
16 goo(); { console.log("aa") }// compile-time error as goo() requires an argument

```
17.14. Trailing Lambdas 303
```

```
If there are optional parameters in front of an optional function type parameter, then calling such a function or method
can skip optional arguments and keep the trailing lambda only. This implies that the value of all skipped arguments is
undefined.
```
1 functionfoo (p1?: number, p2?: string, f?: ()=>string) {
2 console.log (p1, p2, f?.())
3 }
4
5 foo() // undefined undefined undefined
6 foo() {return "lambda" } // undefined undefined lambda
7 foo(1) {return "lambda" } // 1 undefined lambda
8 foo(1, "a") {return "lambda" } // 1 a lambda

```
304 Chapter 17. Experimental Features
```

##### CHAPTER

### EIGHTEEN

### ANNOTATIONS

```
Annotation is a special language element that changes the semantics of the declaration to which it is applied by adding
metadata.
Declaring and using an annotation is represented in the example below:
```
1 // Annotation declaration:
2 @interface ClassAuthor {
3 authorName:string
4 }
5
6 // Annotation use:
7 @ClassAuthor({authorName: "Bob"})
8 class MyClass {/*body*/}

```
The annotation ClassAuthor in the example above adds metadata to the class declaration.
An annotation must be placed immediately before the declaration to which it is applied. An annotation can include
arguments as in the example above.
For an annotation to be used, the name of the annotation must be prefixed with the character ‘@’ (e.g.,@MyAnno). No
white space and line separator is allowed between the character ‘@’ and the name:
```
1 ClassAuthor({authorName: "Bob"}) // compile-time error, no '@'
2 @ ClassAuthor({authorName: "Bob"}) // compile-time error, spaceis forbidden

```
A compile-time error occurs if the annotation name is not accessible (see Accessible ) at the place of use. An annotation
declaration can be exported and used in other modules.
Multiple annotations can be applied to a single declaration:
```
1 @MyAnno()
2 @ClassAuthor({authorName: "John Smith"})
3 class MyClass {/*body*/}

### 18.1 Declaring Annotations

```
Declaring an annotation is similar to declaring an interface where the keywordinterfaceis prefixed with the character
‘@’.
```
##### 305


```
The syntax of annotation declaration is presented below:
```
```
annotationDeclaration:
'@interface' identifier'{'annotationField*'}'
;
annotationField:
identifier':'type constInitializer?
;
constInitializer:
'=' constantExpression
;
```
```
As any other declared entity, an annotation can be exported by using the keywordexport.
Type in the annotation field is restricted (see Types of Annotation Fields ).
The default value of an annotation field can be specified by using initializer as constant expression. A compile-time
error occurs if the value of this expression cannot be evaluated at compile time.
Annotation must be defined at the top level. Otherwise, a compile-time error occurs.
Annotation cannot be extended as inheritance is not supported.
The name of an annotation cannot coincide with the name of another entity:
```
1 @interface Position {/*properties*/}
2
3 class Position {/*body*/}// compile-time error: duplicate identifier

```
An annotation declaration defines no type, and no type alias can be applied to the annotation or used as an interface:
```
1 @interface Position {}
2 typePos = Position // compile-time error
3
4 class Aimplements Position {}// compile-time error

#### 18.1.1 Types of Annotation Fields

```
The choice of types for annotation fields is limited to the following:
```
- _Numeric Types_ ;
- Typeboolean(see _Type boolean_ );
- _Type string_ ;
- Enumeration types (see _Enumerations_ );
- Array of the above types (e.g.,string[]), including arrays of arrays (e.g.,string[][]).
A compile-time error occurs if any other type is used as the type of an _annotation field_.

```
306 Chapter 18. Annotations
```

### 18.2 Using Annotations

```
The following syntax is used to apply an annotation to a declaration, and to define the values of annotation fields:
```
```
annotationUsage:
'@' qualifiedName annotationValues?
;
annotationValues:
'(' (objectLiteral| constantExpression)? ')'
;
```
```
An annotation declaration is represented in the example below:
```
1 @interface ClassPreamble {
2 authorName:string
3 revision:number = 1
4 }
5 @interface MyAnno{}

```
In general, annotation field values are set by an object literal. In a special case, annotation field values are set by using
an expression (see Using Single Field Annotations ).
All values in an object literal must be constant expressions. Otherwise, a compile-time error occurs.
The use of annotation is presented in the example below. The annotations in this example are applied to class declara-
tions:
```
1 @ClassPreamble({authorName: "John", revision: 2 })
2 class C1 {/*body*/}
3
4 @ClassPreamble({authorName: "Bob"}) // default value for revision = 1
5 class C2 {/*body*/}
6
7 @MyAnno()
8 class C3 {/*body*/}

```
Annotations can be applied to the following:
```
- _Top-Level Declarations_ ;
- Class members (see _Class Members_ ) or interface members (see _Interface Members_ );
- Type usage (see _Using Types_ );
- Parameters (see _Parameter List_ and _Optional Parameters_ );
- Lambda expression (see _Lambda Expressions_ and _Lambda Expressions with Receiver_ );
- _Local Declarations_.
Otherwise, a compile-time error occurs:

1 functionfoo () @MyAnno() {}// wrong target for annotation

```
Repeatable annotations are not supported, i.e., an annotation can be applied to an entity no more than once:
```
1 @ClassPreamble({authorName: "John"})
2 @ClassPreamble({authorName: "Bob"}) // compile-time error
3 class C {/*body*/}

```
18.2. Using Annotations 307
```

```
When using an annotation, the order of values has no significance:
```
```
1 @ClassPreamble({authorName: "John", revision: 2 })
2 // the same as:
3 @ClassPreamble({revision: 2 , authorName: "John"})
```
```
When using an annotation, all fields without default values must be listed. Otherwise, a compile-time error occurs:
```
```
1 @ClassPreamble() // compile-time error, authorName is not defined
2 class C1 {/*body*/}
```
```
If a field of an array type for an annotation is defined, then its value is set by using the array literal syntax:
```
1 @interface ClassPreamble {
2 authorName:string
3 revision:number = 1
4 reviewers: string[]
5 }
6
7 @ClassPreamble(
8 {authorName: "Alice",
9 reviewers: ["Bob", "Clara"]}
10 )
11 class C3 {/*body*/}

```
If setting annotation properties is not required, then parentheses can be omitted after the annotation name:
```
```
1 @MyAnno
2 class C4 {/*body*/}
```
#### 18.2.1 Using Single Field Annotations

```
If annotation declaration defines only one field, then it can be used with a short notation to specify just one expression
instead of an object literal:
```
```
1 @interface deprecated{
2 fromVersion:string
3 }
4
5 @deprecated("5.18")
6 functionfoo() {}
7
8 @deprecated({fromVersion: "5.18"})
9 functiongoo() {}
```
```
A short notation and a notation with an object literal behave in exactly the same manner.
```
```
308 Chapter 18. Annotations
```

### 18.3 Exporting and Importing Annotations

```
An annotation can be exported and imported. However, a few forms of export and import directives are supported.
An annotation declaration to be exported must be marked with the keywordexportas follows:
```
1 // a.ets
2 export @interfaceMyAnno {}

```
If an annotation is imported as a part of an imported module, then the annotation is accessed by its qualified name:
```
1 // b.ets
2 import *as ns from"./a"
3
4 @ns.MyAnno
5 class C {/*body*/}

```
Unqualified import is also allowed:
```
1 // b.ets
2 import { MyAnno }from "./a"
3
4 @MyAnno
5 class C {/*body*/}

```
An annotation is not a type. Usingexport typeorimport typenotations to export or import annotations is for-
bidden:
```
1 import type{ MyAnno } from"./a"// compile-time error

```
Annotations are forbidden in the following cases:
```
- Export default,
- Import default,
- Rename in export, and
- Rename in import.

1 import {MyAnnoasAnno} from"./a" // compile-time error

### 18.4 Ambient Annotations

```
The syntax of ambient annotations is presented below:
```
```
ambientAnnotationDeclaration:
'declare'annotationDeclaration
;
```
```
Such a declaration does not introduce a new annotation but provides type information that is required to use the an-
notation. The annotation itself must be defined elsewhere. A runtime error occurs if no annotation corresponds to the
ambient annotation used in the program.
```
```
18.3. Exporting and Importing Annotations 309
```

```
An ambient annotation and the annotation that implements it must be exactly identical, including field initialization:
```
1 // a.d.ets
2 export declare @interfaceNameAnno{name: string= ""}
3
4 // a.ets
5 export @interfaceNameAnno{name: string= ""} // ok

```
The code in the example below is incorrect because the ambient declaration is not identical to the annotation declaration:
```
1 // a.d.ets
2 export declare @interfaceVersionAnno{version:number} // initialization is missing
3
4 // a.ets
5 export @interfaceVersionAnno{version:number = 1}

```
An ambient declaration can be imported and used in exactly the same manner as a regular annotation:
```
1 // a.d.ets
2 export declare @interfaceMyAnno {}
3
4 // b.ets
5 import { MyAnno }from "./a"
6
7 @MyAnno
8 class C {/*body*/}

```
If an annotation is applied to an ambient declaration in the .d.ets file (see the example below), then the annotation is
to be applied to the implementation declaration manually, because the annotation is not automatically applied to the
declaration that implements the ambient declaration:
```
1 // a.d.ets
2 export declare @interfaceMyAnno {}
3
4 @MyAnno
5 declare classC {}

### 18.5 Standard Annotations

```
Standard annotation is an annotation that is defined in Standard Library , or implicitly defined in the compiler ( built-in
annotation ). Standard annotation is usually known to the compiler. It modifies the semantics of the declaration it is
applied to.
An annotation that annotates a declaration of another annotation is called meta-annotation.
```
```
310 Chapter 18. Annotations
```

#### 18.5.1 Retention Annotation

```
@Retentionis a standard meta-annotation that is used to annotate a declaration of another annotation. A compile-time
error occurs if it is used in other places.
The annotation has a single fieldpolicyof typestring. It is typically used as follows:
```
1 @Retention({policy: "RUNTIME"})
2 @interface MyAnno {}// this annotation uses "RUNTIME" policy
3
4 @MyAnno//
5 class C {}

```
The value of this field determines at which point an annotation is used, and discarded after use. The retention policies
can be of three types:
```
- “SOURCE”
    Annotations that use “SOURCE” policy are processed at compile time, and are discarded after compilation;
- “BYTECODE”
    Metadata specified in annotations that use “BYTECODE” policy are saved into the bytecode file, but are dis-
    carded at runtime.
- “RUNTIME”
    Metadata specified in annotations that use “RUNTIME” policy are saved into the bytecode file, are retained and
    can be accessed at runtime.
The default retention policy is “BYTECODE”.
A compile-time error occurs if any other string literal is used as the value ofpolicyfield.
As@Retentionhas a single field, it can be used with a short notation (see _Using Single Field Annotations_ ) as follows:

1 @Retention("SOURCE")
2 @interface Author {name:string} // this annotation uses "SOURCE" policy

### 18.6 Runtime Access to Annotations

```
For an annotation with retention policy (see Retention Annotation )BYTECODEorRUNTIMEan abstract class with the
name of the annotation is implicitly declared. All fields of this class arereadonly. If a field is of an array type, the
array type is alsoreadonly.
For the following annotation:
```
1 @Retention("RUNTIME")
2 @interface MyAnno {
3 name: string
4 attrs: number[]
5 }

```
–the abstract class is declared:
```
```
18.6. Runtime Access to Annotations 311
```

1 abstract class MyAnno {
2 readonlyname: string
3 readonlyattrs: readonly number[]
4 }

```
The use of such a class is represented in following example:
```
1 @MyAnno({name: "someName", attr: [1, 2]})
2 class A {}
3
4 letmy:MyAnno =// call of reflection library to get instance of annotation for type A
5 console.log(my.name)// output: someName

```
312 Chapter 18. Annotations
```

##### CHAPTER

### NINETEEN

### STANDARD LIBRARY

The Standard Library of the ArkTS language defines the required set of types, variables, constants, functions, and
annotations that provide APIs for effective and convenient programming.

The Standard Library has two parts: the common part provides TypeScript compatibility, and the ArkTS-specific part
adds more advanced features.

A detailed description of all elements of the standard library is covered in a separate document that is a part of the
ArkTS distribution package.

##### 313


**314 Chapter 19. Standard Library**


##### CHAPTER

### TWENTY

### IMPLEMENTATION DETAILS

```
Important implementation details are discussed in this section.
```
### 20.1 Import Path Lookup

```
If an import path<some path>/nameis resolved to a path in the folder ‘ name ’, then the compiler executes the following
lookup sequence:
```
- If the folder contains the fileindex.ets, then this file is imported as a module written in ArkTS;
- If the folder contains the fileindex.ts, then this file is imported as a module written in TypeScript.

### 20.2 Modules in Host System

```
Modules are created and stored in a manner that is determined by the host system. The exact manner modules are stored
in a file system is determined by a particular implementation of the compiler and other tools.
A simple implementation stores every module in a single file.
```
### 20.3 Getting Type Via Reflection

```
The ArkTS standard library (see Standard Library ) provides a pseudogeneric static methodType.from<T>()to be
processed by the compiler in a specific way during compilation. A call to this method allows getting a value of type
Typethat represents typeTat runtime.
```
1 lettype_of_int:Type= Type.from<int>()
2 lettype_of_string: Type= Type.from<string>()
3 lettype_of_number: Type= Type.from<number>()
4 lettype_of_Object: Type= Type.from<Object>()
5
6 class UserClass {}
7 lettype_of_user_class: Type= Type.from<UserClass>()
8
(continues on next page)

##### 315


(continued from previous page)
9 interface SomeInterface {}
10 lettype_of_interface: Type= Type.from<SomeInterface>()

```
If typeTused as type argument is affected by Type Erasure , then the function returns value of typeTypefor effective
type ofTbut not forTitself:
```
```
1 lettype_of_array1: Type= Type.from<int[]>()// value of Type for Array<>
2 lettype_of_array2: Type= Type.from<Array<number>>()// the same Type value
```
### 20.4 Ensuring Module Initialization

```
The ArkTS standard library (see Standard Library ) provides a top-level functioninitModule()with one parameter
ofstringtype. A call to this function ensures that the module referred by the argument is available, and that its
initialization (see Static Initialization ) is performed. An argument must be a string literal. Otherwise, a compile-time
error occurs.
The current module has no access to the exported declarations of the module referred by the argument. If such module
is not available or any other runtime issue occurs then a proper exception is thrown.
```
```
1 initModule ("@ohos/library/src/main/ets/pages/Index")
```
### 20.5 Generic and Function Types Peculiarities

```
The current compiler and runtime implementations use type erasure. Type erasure affects the behavior of generics and
function types. It is expected to change in the future. A particular example is provided in the last bullet point in the list
of compile-time errors in InstanceOf Expression.
```
### 20.6 Keywordstructand ArkUI

```
The current compiler reserves the keywordstructbecause it is used in legacy ArkUI code. This keyword can be used
as a replacement for the keywordclassin Class Declarations. Class declarations marked with the keywordstruct
are processed by the ArkUI plugin and replaced with class declarations that use specific ArkUI types.
```
```
316 Chapter 20. Implementation Details
```

### 20.7 OutOfMemoryErrorfor Primitive Type Operations

```
The execution of some primitive type operations (e.g., increment, decrement, and assignment) can throw
OutOfMemoryError(see Error Handling ) if allocation of a new object is required but the available memory is not
sufficient to perform it.
```
### 20.8 Make a Bridge Method for Overriding Method

```
Situations are possible where the compiler must create an additional bridge method to provide a type-safe call for
the overriding method in a subclass of a generic class. Overriding is based on erased types (see Type Erasure ). The
situation is represented in the following example:
```
1 class B<TextendsObject> {
2 foo(p: T) {}
3 }
4 class DextendsB<string> {
5 foo(p: string) {}// original overriding method
6 }

```
In the example above, the compiler generates a bridge method with the namefooand signature(p: Object). The
bridge method acts as follows:
```
- Behaves as an ordinary method in most cases, but is not accessible from the source code, and does not participate
    in overloading;
- Applies narrowing to argument types inside its body to match the parameter types of the original method, and
    invokes the original method.
The use of the _bridge_ method is represented by the following code:

1 letd =newD()
2 d.foo("aa")// original method from 'D'is called
3 letb: B<string> = d
4 b.foo("aa")// bridge method with signature (p: Object) is called
5 // its body calls original method, using (p as string) to check the type of the argument

```
More formally, a bridge methodm(C 1 , ..., Cn)is created inD, in the following cases:
```
- ClassBcomprises type parametersB<T 1 extends C 1 , ..., Tnextends Cn>;
- SubclassDis defined asclass D extends B<X 1 , ..., Xn>;
- Methodmof classDoverridesmfromBwith type parameters in signature, e.g.,(T 1 , ..., Tn);
- Signature of the overridden methodmis not(C 1 , ..., Cn).

```
20.7. OutOfMemoryError for Primitive Type Operations 317
```

**318 Chapter 20. Implementation Details**


##### CHAPTER

### TWENTYONE

### GRAMMAR SUMMARY

literal:Literal;

identifier: Identifier;

indexType: 'number';

type:
annotationUsage?
(typeReference
|'readonly'?arrayType
|'readonly'?tupleType
|functionType
|functionTypeWithReceiver
|unionType
|keyofType
|StringLiteral
)
|'('type')'
;

typeReference:
typeReferencePart('.'typeReferencePart)*
;

typeReferencePart:
identifier typeArguments?
;

arrayType:
type'[' ']'
;

##### 319


**320 Chapter 21. Grammar Summary**


##### CHAPTER

### TWENTYTWO

### CONTRIBUTORS

Language design lead:

- Nedoria Aleksei

Contributors:

- Bronnikov Georgy
- Gavrin Evgeny
- Huo Qingyi
- Kanatov Alexey
- Nedoria Aleksei
- Olshevsky Vladimir
- Pavlyuk Alexander
- Pei Jiajun
- Polyakov Alexander
- Pukhov Vsevolod
- Qiu Yu
- Rubanov Vladimir
- Soldatov Anton
- Solomennikov Dmitry
- Trubenkov Dmitrii
- Velikanov Michael
- Xian Yuqiang
- Zouev Evgeniy

Technical writer:

- Baranov Dmitry

##### 321


**322 Chapter 22. Contributors**


## Index

### A

abrupt completion, 90–92, 95, 102, 106, 111, 113,
122, 123, 141–144, 148, 153, 164, 165, 276
abstract, 186
abstract class, 98, 168, 169, 172, 186, 312
abstract concept, 2
abstract data structure, 2
abstract declaration, **3** , 202
abstract function, 268, 278
abstract keyword, 293
abstract method, 98, 108, 169, 171, 172, 186, 187, 195
abstract method call, 108
abstract modifier, 168, 169, 185–187, 189
abstract notion, 2
abstract symbol, 3
abstraction, 2
access, 36, 38, 40, 47, 53, 54, 65, 78, 79, 91, 105, 106,
110, 111, 113, 141, 144, 164, 176–182, 191,
194, 195, 198, 202, 203, 211, 212, 216, 217,
219, 221–223, 226, 227, 235, 236, 253, 254,
260, 261, 263, 273, 275, 279, 284, 296, 297,
302, 305, 309, 311, 316, 317
access constructor, 88
access expression, 142
access modifier, 54, 167, 177, 178, 189, 191, 194,
202, 254, 285
accessibility, 51, 53, 54, 65, 74, 79, 99, 106, 148, 159,
169, 171, 176–178, 180, 191, 194, 195, 198,
202, 203, 212, 216–218, 221–223, 226, 236,
254, 275, 279, 283, 296, 297, 305
accessible, 260
accessible constructor, 178, 194
accessible declaration, 217, 222, 236
accessible entity, 54
accessible function, 284
accessible interface type, 171
accessible member, 195
accessible member field, 99, 106
accessible method, 285
accessible scope, 53
accessible type, 178
accessor, 46, 100, 101, 106, 167, 173, 175–177, 184,

##### 190, 195, 200–202, 254, 295, 299, 302, 303

```
accessor declaration, 100, 106, 177, 184, 217, 299
accessor modifier, 189, 190
accessor with receiver, 217, 295, 299
accessor with receiver declaration, 299
addition, 13, 130
additive expression, 29, 83, 130, 150
additive operator, 29, 30, 130, 131, 150
alias, 23, 24, 27, 38, 41, 45, 55, 56, 169, 198, 216, 287,
288
alignment, 137, 264
allocation, 148, 317
alpha-numeric character, 214
ambient, 229
ambient accessor declaration, 232
ambient annotation, 310
ambient call signature, 233
ambient call signature declaration, 233
ambient class, 233–235
ambient class declaration, 232, 233
ambient constant, 230
ambient constructor, 229
ambient constructor declaration, 232
ambient context, 230, 233, 234, 236
ambient declaration, 217, 229, 282, 310
ambient field declaration, 232
ambient function, 229, 230
ambient function declaration, 230
ambient indexer, 234
ambient indexer declaration, 233
ambient interface, 234
ambient interface declaration, 234, 235
ambient iterable, 234
ambient iterable declaration, 234, 235
ambient method, 229, 232
ambient method declaration, 232
ambient namespace, 222, 235, 236
ambient namespace declaration, 235, 236
ambient overload function, 231
ambient overload function declaration, 231
AND operator, 138
annotation, 26, 35, 57–59, 147, 155, 159, 176, 188, 200,
```
##### 323


##### 217, 238, 268, 282, 301, 305–312

annotation declaration, 305–310
annotation field, 306, 307
annotation name, 308
anonymous class, 100, 102
anonymous type, 27, 55, 56
any, 24
Any type, 45
any type, 68
API, 269
arbitrary large integer, 36
arbitrary signature, 281
argument, 42, 60–64, 88, 92, 103, 104, 109, 114, 194,
227, 239, 269, 274, 281, 287, 295, 302, 304,
305, 316
argument expression, 83
argument type, 70, 177, 251, 317
arithmetic operator, 83
ArkUI code, 316
ArkUI plugin, 316
ArkUI type, 316
array, 24, 27, 32, 38, 39, 55, 58, 61, 67, 68, 71, 73, 90,
92, 95–97, 104, 111, 114, 141, 143, 158, 240,
247, 251, 271, 274–276, 306, 316
array access, 111
array access expression, 90
array argument, 63
array bounds checking, 207
array creation, 271, 276
array creation expression, 114, 275, 276
array declaration, 38, 273
array dimension, 275
array element, 32, 38, 40, 56, 81, 90, 95–97, 110, 142–
144, 247, 273, 275
array element type, 90, 247
array indexing, 110
array indexing expression, 90, 110, 111, 142, 143
array initialization, 96
array initializer, 95
array instance, 37, 114, 274, 275
array length, **3** , 37–39, 90, 95, 111, 142, 143, 273
array literal, 45, 47, 81, 89, 95–97, 103, 116, 117,
237, 252, 271, 273, 275, 308
array literal expression, 95
array of arrays, 276
array operation, 38
array reference expression, 90, 110, 142
array reference subexpression, 143
array size, 271
array type, **3** , 23, 25, 26, 32, 37, 38, 61–63, 72, 95, 103,
110, 240, 249, 251, 271, 273, 298, 308, 311
array value, 37
assign, 238, 273
assignability, 28, 38, 40, 41, 57, 58, 62, 63, 65, 68,

##### 73, 76, 77, 82, 96, 99, 117, 141, 142, 163, 165,

##### 197, 238, 240, 249, 251, 273, 275, 279, 282

```
assignable type, 65, 73, 76, 77, 251
assignment, 33, 38–40, 42, 47, 49, 57, 61, 81, 84, 89–
92, 96, 103, 113, 141–144, 159, 163, 180–182,
204, 238, 249, 263, 275, 317
assignment context, 28, 81
assignment expression, 58, 141–144
assignment operator, 57, 91, 141, 143
assignment-like context, 81, 84, 238–240
assignment-like contexts, 58
associativity, 91, 126, 130, 138, 140
asymmetric relationship, 249
async function, 75, 268, 269, 278
async lambda, 268
async mark, 147
async method, 186, 268, 279
async modifier, 186, 230, 268, 271
async type, 66
asynchronous API, 269
asynchronous launch, 271
asynchronous operation, 269
asynchronous programming, 267
automatic transition, 2
available memory, 317
await expression, 269
await operator, 91
awaited, 75
```
### B

```
backslash, 19
backslash character, 214
backspace, 19
backtick, 20, 145
backward compatibility, 268
balanced braces, 154
base, 129, 250
base class, 169, 183, 194, 207, 249, 256
base type, 84, 249, 257
base URL, 214
basic coroutine, 270
BigInt, 28
bigint, 24, 28, 29, 131
bigint comparison, 133
bigint literal, 18, 36
bigint operand, 133
bigint type, 32, 36, 83, 125, 131, 133, 135, 139
binary, 14
binary expression, 121
binary numeric expression, 239
binary operation, 143, 144
binary operator, 30, 82, 91, 126–131, 239
bind all, 54
binding, 210–213, 215, 216, 224
```
**324 Index**


bitwise AND operand, 139
bitwise complement, 125
bitwise complement expression, 125, 126
bitwise complement operator, 29, 30, 125
bitwise exclusive OR operand, 139
bitwise expression, 83, 138, 139
bitwise inclusive OR operand, 139
bitwise logical AND operator, 132
bitwise operator, 91, 138, 150
block, 54, 60, 107, 108, 148, 154, 156, 161, 163, 164,
187, 188, 293, 294, 303
block notation, 303
block of code, 191
block scope, 54, 155, 156
block statement, 154
body, 54, 60, 185, 202
Boolean, 31
boolean, 29, 31, 49, 66, 82, 115, 246
boolean comparison, 134
Boolean literal, 18
boolean logic, 264
boolean logical expression, 139
boolean operand, 139, 140
boolean operator, 139
boolean relational operator, 134
boolean type, 28, 30, 31, 65, 125, 132, 134, 135, 138–
140, 145, 155, 157, 264, 306
boolean value, 145
bound, 245
bound entity, 212, 215
bounded instance, 137
bounded object, 137
brace, 95
bracket, 95, 110
break, 162
break statement, 156, 159, 160, 162
bridge method, 254, 317
built-in annotation, 310
built-in array, 27
built-in getter, 76
built-in setter, 76
built-in type, 36, 38, 273
byte, 28, 29, 49, 84, 86
bytecode, 311
bytecode file, 311

### C

call, 63, 88, 91, 103, 107–109, 184, 194, 212, 219, 226,
260, 263, 270, 274, 280, 281, 283, 288, 289,
296, 302, 303, 310, 316
call argument, 88, 109, 251
call context, 81
call expression, 148, 280
call method, 181

```
call parameter type, 96
call site, 104, 260, 283, 291
callable class type, 280
callable type, 233, 280, 281, 286
callback, 269
callee, 109
caller scope, 165
candidate, 260
captured by lambda, 148, 149
captured variable, 148, 149
carriage return character, 10
case sensitivity, 214
cast, 29, 31, 116
cast conversion, 118
cast expression, 48, 90, 96, 116, 117, 150, 261
cast operator, 29, 30, 91, 116
casting conversion, 4 , 86, 96, 150, 254
catch clause, 163–165
catch identifier, 164
chaining, 91
chaining operator, 88, 107–110, 113, 144
char, 14, 49
char literal, 272
char type, 272
character, 3, 9, 35, 36, 272
character literal, 272
character type, 28
check, 117, 182, 251
circular dependency, 181
circular reference, 43
class, 27, 32, 40, 47, 51, 53, 67–71, 73, 74, 76, 88, 93–
95, 97–99, 102, 104–108, 110, 114, 115, 135,
161, 168–173, 175–182, 184, 185, 187, 188,
190, 193–195, 197–199, 201, 207, 217, 245,
254, 256, 257, 262, 263, 271, 275, 277, 279,
280, 287–289, 291, 293–295, 297, 301, 302,
307
class accessor, 184, 188
class accessor declaration, 175, 188
class body, 167, 176–179, 192, 194
class constructor, 167, 180, 255
class declaration, 27, 55, 167, 168, 171, 176, 179,
181, 184, 195, 293, 305, 307, 316
class declaration body, 184
class declaration scope, 177
class extension, 169, 242, 293
class extension clause, 169
class field, 99, 171, 177
class fields, 180
class implementation clause, 171
class inheritance, 271
class instance, 54, 92, 97, 102, 148, 176, 177, 179–
181, 191, 233, 234
class instance creation expression, 114, 115
```
**Index 325**


class instance expression, 114
class instance method, 65
class instantiation, 191
class interface, 271
class iterator, 158
class keyword, 316
class level scope, **4** , 54
class member, 53, 54, 167, 176–178, 284
class method, 172, 176, 184, 190, 232, 261, 268, 282,
284
class method declaration, 184
class method overload declaration, 291
class name, 168, 198
class type, 23, 25, 32, 43, 46, 73, 76, 77, 98, 99, 104–
106, 116, 168, 169, 171, 253, 257, 276, 280,
281, 293, 298
class variable, 253
class-composite context, 99
class-level scope, 167
clause, 169
closure, 240
code readability, 252
comma, 13
comma-separated argument expression, 92
comma-separated list, 98
command-line argument, 228
comment, 2, **4** , 9, 10, 21
common subset, 1
commutative operation, 126, 130
commutative operator, 134, 138
comparison, 13, 31, 133–135
comparison operator, 29, 30
compatibility, 24, 28, 36, 38, 40, 48, 68, 78, 89, 97,
109, 162, 202, 204, 213, 220, 229, 233–235,
251, 259, 263, 268, 273, 280, 282, 300
compatible code, 280
compatible expression, 89
compilation, 110, 195, 214, 275, 316
compilation environment, 214
compilation tool, 263
compile time, 23, 89, 107, 118, 150, 180, 261, 263, 275,
282, 288, 297, 306, 311
compile time error, 273
compile type, 259
compile-time error, **4**
compile-time warning, **4**
compile-time error, 16, 17, 33, 38–41, 43, 44, 46, 47,
52, 56, 57, 59–62, 65, 66, 69–78, 82, 88, 89, 93,
94, 96–110, 112–118, 121–125, 127–134, 136,
138–141, 144, 145, 147, 148, 155–163, 168–
173, 175, 179, 181–194, 198–200, 202–204,
209–211, 214, 216–220, 223, 225, 226, 229,
230, 236–239, 250, 252, 254–257, 268, 274–
276, 278–290, 292–294, 296–300, 302, 303,

##### 305–308, 311, 316

```
compile-time feature, 2
compile-time polymorphism, 259
compile-time warning, 135, 229, 259
compile-time-error, 135
compiler, 23, 56, 66, 103, 112, 181, 182, 192, 213, 214,
227, 253, 254, 278, 279, 310, 315–317
compiler environment, 214
compiler-known signature, 278, 279
complement expression, 125
complement operator, 31
completion, 122, 207
completion failure, 207
compliance, 85, 86
component programming, 2
composite literal context, 81
compound assignment expression, 143
compound assignment operator, 143, 144
compound-assignment operator, 91
concatenation, 30, 31, 81, 145
concatenation operator, 36
concrete method, 186
concurrency, 267, 269, 271
concurrent execution, 263
conditional evaluation, 139
conditional expression, 125, 155, 253
conditional operator, 29, 30, 91, 150
conditional-and expression, 83, 139, 264, 265
conditional-and operator, 31, 121, 139, 150
conditional-or expression, 83, 140, 264, 265
conditional-or operator, 31, 121, 140, 150
configuration file, 214
console, 209
const, 159
const declaration, 154
const enum, 203
const enum type, 262
const keyword, 229, 235
const modifier, 203
constant, 4 , 29, 30, 51, 53, 58, 81, 84, 85, 103, 162,
203–205, 217, 220, 252, 276
constant declaration, 4 , 17, 58, 81, 217, 225, 230,
238, 252
constant expression, 40, 89, 116, 130, 150, 203, 204,
237, 252, 275, 306
constant field, 180
constant value, 134
constant variable, 218, 224
constant-time operation, 38, 273
constraint, 67, 68, 73, 89, 107, 239, 245, 257, 258, 262
construct, 1, 26, 271
constructed value, 77
constructor, 29, 30, 60, 62, 81, 102–104, 106, 114, 115,
120, 161, 163, 167, 168, 176–178, 181, 191,
```
**326 Index**


##### 193–195, 229, 251, 254, 255, 259, 271, 273–

##### 276, 280, 282, 289, 293, 294, 301

constructor body, 115, 161, 191–195, 229
constructor call, 81, 92, 115, 191–195, 251, 255
constructor call statement, 104
constructor declaration, 181, 191, 194, 293, 294
constructor keyword, 191
constructor name, 294
constructor overload, 176
constructor overload declaration, 289, 294
constructor overloading, 271
constructor parameter, 191
constructor type, 178
contained expression, 104
container, 78
context, 12, 41, 55, 74, 81, 82, 84, 89, 95–100, 104, 105,
113, 148, 155, 183, 214, 229, 237–240, 252,
253, 255, 259, 269, 302
context-free grammar, **4**
context-free grammar, 2
contiguous memory location, 38
continue statement, 156, 160
contravariance, 70, 246, 247, 249, 250, 258
contravariance pattern, 190
contravariant, 70, 257
contravariant parameter, 257
contravariant parameter type, 257
contravariant return type, 247, 257
contravariant type parameter, 262
control, 153, 188, 260
control transfer, 159–163
conversion, 31, 41, 44, 48, 61, 81–86, 96, 110, 122–
125, 128, 130, 131, 133, 136, 139, 142, 143,
145, 204, 205, 249, 252, 275
converted type, 133, 136
convertibility, 123, 124, 139
convertible expression, 122
convertible type, 122–125, 130, 133, 136
core, 267
coroutine, 87, 88, 263, 267–271
coroutine stack, 165
covariance, 70, 246, 247, 249, 250, 257
covariance pattern, 190
covariant, 70
covariant parameter type, 257
covariant return type, 247, 257
covariant type parameter, 262
creation, 197
creation expression, 92
cross-platform development, 2
curly brace, 98, 145
custom synchronization, 269
cyclic dependency, 57

### D

```
data analysis, 267
data entity, 253
data member, 179
data race, 269
data type, 49
database, 267
deadlock, 263
deallocation, 2
decimal, 14, 18, 82
decimal form, 30
decimal number, 17
declaration, 21, 43, 51, 53–55, 57, 59, 74, 93, 158, 159,
171, 176, 177, 182–186, 188, 197, 198, 202,
211, 216–218, 220, 222–226, 229, 230, 236,
238, 249, 253, 277, 285–287, 289, 295, 297,
305–307, 310–312, 316
declaration annotation, 311
declaration body, 154
declaration name, 223
declaration scope, 52, 167, 176, 200, 211, 212, 215
declare, 229
declared class member, 167
declared entity, 52, 306
declared function, 93
declared interface, 198
declared name, 218
declared type, 104, 229, 235
declaring class, 178
decrement, 113, 150, 317
decrement expression, 122, 123
decrement operator, 29–31, 89, 91, 122, 123, 150
decrementation, 122, 123
default, 69, 99, 227
default constructor, 194, 195
default export directive, 217, 218
default implementation, 235, 294
default keyword, 211, 235
default method, 294
default target, 218
default type, 67, 69, 239
default value, 48, 57, 61, 101, 180, 182, 187, 230, 263,
274–276, 303
definite assignment assertion, 182
delimiter, 2
denormalizaton, 31
denormalized number, 31
denormalized value, 31
derived class, 54, 169, 178, 207, 256, 257, 271, 295,
297
derived class constructor, 255
derived interface, 295
derived type, 47, 249
difference type, 248
```
**Index 327**


dimension expression, 275, 276
direct call expression, 104
direct extension, 197
direct implementation, 197
direct subclass, 169, 170, 263
direct subinterface, 198
direct superclass, 169, 177, 195
direct superinstance, 177
direct superinterface, 42, 171, 195, 198, 200, 202,
245
direct supertype, 240, 245
directive, 210, 223, 224
dispatch, 107, 263
distinct argument, 73
distinct generic declaration, 73
distinct type, 76, 77
distinguishable functions, 53
dividend, 127–129
division, 127, 128
division operator, 29, 127, 128
divisor, 127–129, 164
do statement, 157, 160, 264
dot operator, 54
dot-separated name, 92
double, 28, 30, 49
double infinity, 85
double NaN, 85
double quotes, 19
double type, 85
DSL support, 302
dual semantics, 35
dynamic dispatch, 107, 263, 278
dynamically created object, 2
dynamically dispatched overriding, 2

### E

effective type, 117, 261, 262
element type, 38
else-block, 156
embedded expression, 145, 146
embedded namespace, 220, 221, 236
embedded type, 74
empty body, 187, 293
empty string, 264
enclosing context, 155
enclosing statement, 160
end marker, 3
ensure-not-nullish expression, 48, 120
entity, 51, 53, 54, 63, 67, 92, 93, 173, 201, 209–212,
215, 216, 219, 224, 229, 235, 236, 252–254,
259, 260, 263, 282, 283, 290, 300, 303, 306,
308
entity declaration, 53
entry point, 227, 228

```
entry point function, 227, 228
enum, 204, 217, 271
enum constant, 203, 204
enum declaration, 55
enum member, 54
enum type, 32
enumeration, 27, 28, 169, 203, 205, 262
enumeration base type, 204, 262
enumeration constant, 84, 134, 203–205, 276, 277
enumeration constant value, 134, 204
enumeration declaration, 27, 203
enumeration integer value, 134, 204
enumeration method, 271, 276
enumeration relational operator, 134
enumeration string value, 134
enumeration type, 23, 25, 28, 48, 82, 84, 85, 134, 135,
150, 203–205, 276, 277, 306
enumeration type constant, 150
enumeration type declaration, 229, 235
environment, 165
environment variable, 214
equality, 35, 128, 136
equality expression, 83, 134, 135, 137, 150
equality operator, 29, 91, 134, 135, 137, 150, 272
erased type, 317
error, 29, 31, 33, 90–92, 127, 129, 131, 148, 153, 154,
163–165, 182, 195, 207, 208, 226, 275, 317
error handling, 207
error object, 90, 163
error situation, 208
escape character, 20
escape sequence, 19, 272
evaluation, 87–92, 95, 102, 103, 106, 109, 111, 113,
115, 116, 118, 120–123, 127, 134, 135, 139–
145, 147–150, 153, 156–158, 161, 162, 180,
181, 276, 278
evaluation result, 218
exception, 263, 316
exclusive OR operator, 138
executable code, 184, 229
execution, 90, 95, 102, 148, 153–155, 157, 160–162,
164, 165, 186, 192, 194, 207, 227, 261
execution path, 65, 192
execution transfer, 162
exit condition, 160
explicit call, 280
explicit constructor call, 194
explicit initialization, 48
explicit instantiation, 67
exponent, 129
exponentiation, 91, 129
export, 54, 210–212, 216–220, 223–225, 309
export annotation, 309
export default, 309
```
**328 Index**


export directive, 218, 223, 224, 309
export function, 221
export keyword, 306, 309
export modifier, 217
export namespace, 222
export target, 218
export type, 213, 224, 309
exported declaration, 217, 218, 220, 223
exported entity, 51, 54, 93
expression, 3, **4** , 23, 35, 54, 61, 66, 78, 81–84, 87–92,
95, 97–99, 101–104, 108–110, 113–116, 118,
120–126, 130, 132, 139–141, 143–145, 148,
150, 154–159, 161–163, 180, 184, 191, 193,
203, 204, 218, 226, 237–240, 251, 252, 264,
269, 275, 276, 280, 282, 289, 294, 295, 300,
306–308
expression evaluation, 90
expression statement, 154, 226
expression type, 66, 82, 85, 89, 90, 108, 114, 139,
162, 264
expression value, 81, 157, 162
extended conditional expression, 125, 145, 157,
264
extended equality, 137
extended exponent, 142
extended semantics, 140, 264
extends clause, 169, 198, 293
extends graph, 169, 198
extends keyword, 68
extends Object clause, 169
extension, 197–199, 214, 293
extension clause, 192, 242

### F

factory, 281
factory parameter, 281
field, 46, 51, 75, 81, 99, 100, 102, 105, 106, 110, 113,
141, 144, 167, 173, 175–177, 179–184, 188,
189, 197, 200, 201, 299, 302, 303, 308, 311
field access, 106, 113, 144, 180, 261
field access expression, 48, 105, 106, 113, 180
field annotation, 308
field declaration, 81, 177, 179, 180
field initialization, 180, 183, 310
field initializer, 168, 180–182, 192, 194
field modifier, 179
field name, 79
field overriding, 183
field type, 99, 141
field value, 182, 188
field with late initialization, 179, 181, 182,
192
file, 214, 315
file path, 214

```
file system, 315
filesystem, 214
final class, 168, 271, 293
final keyword, 293
final method, 187, 189, 271, 293
final modifier, 168, 169, 184–186
finally block, 164
finally clause, 163, 164
finite value, 127, 128, 131, 133
first-match algorithm, 260
fit into( v. ), 4
fixed array type, 23
fixed-size array type, 4
fixed-size array, 37, 90, 240, 247, 273
fixed-size array argument, 64
fixed-size array type, 32, 64, 97, 247, 273
flexibility, 278, 295
float, 28, 30, 49, 84
float infinity, 85
float NaN, 85
float type, 85
float zero, 85
floating-point addition, 130
floating-point arithmetic, 130
floating-point calculation, 91
floating-point comparison, 133
floating-point division, 127, 128
floating-point equality test, 136
floating-point expression, 31
floating-point infinity, 85, 86
floating-point literal, 17, 237
floating-point number, 30, 31, 124
floating-point operand, 86, 128, 131, 133, 137
floating-point operation, 30, 31, 127, 128, 131
floating-point operator, 31
floating-point remainder, 129
floating-point remainder operation, 128, 129
floating-point subtraction, 131
floating-point type, 28, 30, 31, 38, 82, 84, 86, 127,
128, 131, 275
floating-point value, 31, 124, 133, 137
floating-type multiplication, 126
flush to zero, 31
folder, 214, 315
for statement, 155, 158, 160, 264
for-of loop, 158
for-of statement, 155, 158, 160, 279
for-of type annotation, 159
for-of type statement, 282
for-variable, 282
form feed, 10, 19
formal parameter, 81, 147
fractional part, 275
```
**Index 329**


function, 27, 33, 34, 40, 51, 55, 57, 60, 62–69, 71, 73,
74, 81, 93, 96, 103, 104, 109, 136, 137, 155,
160–164, 169, 217, 220, 226, 227, 229, 230,
238, 251–254, 259, 260, 263, 270, 271, 277,
278, 282–284, 290, 292, 295, 298, 299, 301,
304, 307, 316
function body, 60, 65, 148, 161, 228–230, 268, 292
function body declaration, 54
function call, 39, 40, 48, 53, 60, 61, 65, 81, 89, 92,
108, 109, 113, 251, 261, 263, 283, 295, 297,
299, 303, 304
function call expression, 65, 108, 109
function declaration, **4** , 54, 60, 147, 217, 230, 284,
295
function increment, 178
function name, 54, 277, 295
function object, 42, 137
function overload, 284
function overload declaration, 231, 284
function overloading, 271
function parameter, 57, 65
function reference, 93, 290
function return type, 66, 252
function scope, **4** , 54
function signature, 40, 93
Function type, 42
function type, 23, 25, 26, 32, 40–42, 71, 72, 108, 135,
147, 148, 246, 257, 295, 298–300, 302–304,
316
function type equality operator, 137
function type parameter scope, **4** , 54
function type with receiver, 25, 299–301
function types conversion, **4**
function with receiver, 104, 217, 295–297, 299,
302
function with receiver declaration, 295
functional object, 109
functionality, 267, 271, 295

### G

general import, 213
generic, 2, **4** , 67, 69, 73, 74, 89, 198, 249, 257, 261, 283,
300, 316
generic class, 27, 68, 70, 71, 168, 208, 245, 278, 317
generic class declaration, 171
generic declaration, 67, 68, 73, 198
generic entity, 71
generic function, 60, 68, 74, 93, 297
generic instantiation, 67, 68, 71, 73, 93, 94, 115,
198, 252, 283
generic interface, 68, 70, 171, 198, 245, 278
generic method, 74, 93, 94
generic parameter, 67
generic tuple, 262

```
generic type, 4 , 27, 34, 35, 56, 69, 70, 73, 101, 115,
245, 249, 262
genericity, 2
get-accessor, 189, 299
getter, 76, 77, 173, 188–190, 200–202, 299
getter body, 189
getter parameter, 190
goal symbol, 2, 3, 4
gradual underflow, 31
grammar, 5
grammar production, 3
grammar rule, 3, 88
```
### H

```
hard keyword, 12
hardware, 267
header, 185
hexadecimal, 14, 19
hidden field, 173, 175
hidden member, 197
hiding, 197
high-level language, 2
high-level sequence, 192
horizontal tab, 19
horizontal tabulation, 10
host system, 315
```
### I

```
identifier, 2, 10–13, 26, 27, 51, 60, 61, 98, 99, 106,
113, 156, 160, 162, 168, 179, 184, 188, 191,
198, 201, 211, 212, 224, 284, 285
identity, 249
identity conversion, 84
IEEE 754, 30, 31, 84–86, 126–131, 133, 136
if statement, 155, 156, 253, 264
immutable variable, 155
implementation, 30, 31, 66, 136, 167, 169, 171–175,
183, 184, 186, 187, 190, 191, 197–199, 201,
202, 214, 233, 234, 236, 242, 254, 255, 271,
278, 279, 288, 291, 293–295, 310, 315
implementation clause, 242
implementation method, 187
implementing, 167
implements clause, 171
implicit conversion, 17, 145, 249
import, 54, 209, 211, 213–218, 224–226, 284, 309, 310
import annotation, 309
import binding, 210–215, 217
import declaration, 210, 215
import default, 309
import directive, 209–211, 213, 215, 217, 223, 309
import outcome, 215
import path, 210, 211, 213–215, 225, 315
import statement, 213
```
**330 Index**


import type, 211, 213, 309
import type directive, 213
imported declaration, 209
imported entity, 93
imported file, 214
imported function, 93
imported module, 226, 309
in keyword, 70
in-place type declaration, 25
in-position, 70
in-variance, 67
inclusive OR operator, 138
incompatibility, 273
increment, 113, 150, 251, 317
increment expression, 122, 123
increment operator, 29–31, 89, 91, 122, 123, 150
incrementation, 122
index, 78, 273
index expression, 40, 78, 110–113
index parameter, 278
index subexpression, 142–144
indexable type, 38, 110, 233, 277
indexing, 36, 92, 111, 233
indexing expression, 48, 78, 92, 110–113, 142–144,
277, 278
indexing expression evaluation, 278
indistinguishable type, 249
inference, 57, 66, 302
inference type, 101
inferred type, 58, 59, 65, 66, 74, 89, 95–98, 100, 116,
125, 148, 158, 159, 189, 190, 228, 237, 239,
253, 263, 279
infinite operand, 131
infinite value, 131
infinity, 85, 91, 124, 127–129, 131, 133
infinity double, 85
inheritance, 2, 32, 176, 177, 179, 184–186, 190, 195,
197, 198, 200, 202, 254, 271, 280, 285, 286,
288, 295, 306
inherited class member, 167
inherited field, 184
inherited member, 177, 195, 197
initial value, 39, 40, 81, 102
initialization, 48, 57, 76, 77, 95, 101, 102, 114, 115,
148, 155, 158, 176, 180–183, 191, 192, 226,
229, 238, 263, 276, 310, 316
initialization expression, 97
initializer, 57, 59, 99, 150, 161, 180, 181, 183, 220,
221, 229, 232, 238, 240, 262, 306
initializer block, 106, 161, 167, 176, 177, 180, 219
initializer declaration, 232
initializer expression, 57–59, 95–97, 181, 230
innermost declaration, 54
instance, 32–34, 54, 78, 92, 97, 102, 114, 115, 135, 137,

##### 147, 148, 158, 159, 180, 181, 186, 187, 191,

##### 192, 197, 273–275, 277, 279, 281, 301, 302

```
instance creation expression, 97, 148, 191
instance entity, 54
instance field, 106, 179–181, 183, 191, 192
instance field access, 106
instance field initializer, 181
instance member, 51, 53, 167
instance method, 94, 104, 105, 107–109, 186, 188,
194, 202, 255, 281, 284, 293, 297, 303
instance method call, 105
instance name, 180
instance own field, 191
instance variable, 197
instanceof, 253
instanceof expression, 115, 261, 316
instanceof operator, 115
instantiated generic type, 115
instantiation, 27, 34, 35, 67–74, 93, 114, 115, 168,
171, 180, 197, 245, 262, 280, 281
int, 16, 28, 29, 49, 84
int type, 86, 204, 227, 275
integer, 14, 16, 18, 30, 31, 36, 38, 126, 129, 130, 273,
275
integer addition, 91, 130
integer arithmetic, 130
integer bitwise operator, 29, 30, 139
integer conversion, 252
integer division, 90, 91, 127, 132
integer equality test, 136
integer expression, 228
integer literal, 16, 204, 237, 239
integer multiplication, 91, 126
integer number, 273
integer operand, 127, 128, 133, 137
integer operator, 29
integer overflow, 127
integer remainder, 90, 91, 128
integer subtraction, 131
integer type, 28, 29, 82, 84, 86, 110, 111, 125, 131,
139, 203, 204, 252, 264, 275
integer value, 18, 29, 124, 127, 131, 203, 204, 275
interface, 27, 32, 51, 54, 67, 68, 70, 71, 73, 74, 93,
94, 100–102, 104, 106, 107, 169, 171, 173, 175,
176, 182, 183, 186, 187, 190, 197–202, 217,
234, 245, 255, 256, 263, 271, 277–279, 287,
288, 291, 294, 295, 302, 306, 307
interface body, 202, 294
interface declaration, 27, 55, 197, 198, 200
interface field, 81
interface inheritance, 202
interface keyword, 306
interface level scope, 5 , 54
interface member, 53, 54, 200
```
**Index 331**


interface method, 172, 190, 202, 235, 282, 288
interface method declaration, 202, 294
interface name, 198
interface property, 101, 200–202
interface type, 23, 25, 27, 29, 32, 43, 46, 47, 51, 73,
76, 77, 98, 100, 101, 104, 116, 171, 197, 198,
200, 242, 253, 257, 296, 298
interface type declaration, 245
interface type variable, 51
intersection type, 248
invariance, 70, 249, 250, 257
invariant, 70
invariant type parameter, 262
invocation, 263, 280, 281
iterable class, 158, 279, 280
iterable class instance, 234
iterable interface, 158, 279
iterable type, 158, 159, 279, 286
iteration, 158–160
iterator, 159, 279, 280

### K

key, 78, 101, 112, 113, 142, 144
key type, 101, 102, 112
key-value pair, 142, 144
keyof keyword, 47
keyof type, 47
keyword, 2, **5** , 10, 12, 13, 18, 21
keyword null, 35
keyword super, 54
keyword this, 54
keyword undefined, 35

### L

label, 156, 160
label identifier, 160
lambda, 34, 42, 62, 65, 66, 92, 148, 149, 156, 247, 270,
299, 301–304
lambda body, 105, 147, 148, 161, 302, 303
lambda call, 81, 148
lambda code, 253
lambda expression, 65, 92, 104, 108, 147–149, 156,
252, 268, 271, 295, 301, 307
lambda expression call, 148
lambda expression type, 148
lambda expression with receiver, 87, 88, 295,
301–303, 307
lambda function, 275
lambda parameter, 148, 302
lambda receiver type, 302
lambda return type, 148
lambda signature, 147, 148, 302
lambda with receiver, 298, 299
lambda with receiver body, 302

```
language element, 305
late initialization, 182
launch function, 271
lazy operator, 121
left shift, 132
length property, 273
let, 159
let declaration, 154
lexical element, 2
lexical grammar, 2, 3
lexical input, 9, 10
lexical input element, 10
lexical notation, 2
lexical structure, 2
line separator, 2, 9, 10, 21, 305
line separator character, 10
linearization, 5 , 45
linkage, 90
literal, 5 , 10, 14, 16, 18, 20, 35, 36, 78, 89, 92, 96, 102,
112, 113, 150
literal expression, 97
literal type, 20, 23, 25, 32, 36, 37, 44, 45, 48, 59, 102,
112, 245, 262
literal value, 18, 44
local declaration, 154, 307
local variable, 84, 253
logical complement, 125, 264
logical complement expression, 125
logical complement operator, 125
logical expression, 83, 138, 139
logical operator, 31, 91, 138, 139, 150
long, 16, 28, 29, 49, 84
long type, 36, 86, 204
lookup sequence, 315
loop, 156, 158–160, 265
loop body, 156, 159
loop index variable, 158
loop iteration, 158, 160
loop label, 156
loop scope, 159
loop statement, 156, 159, 160, 162
loss of information, 30, 82, 127, 128, 131
loss of precision, 129
low-level representation, 2
low-order bit, 126, 130
lowest-order bit, 132
lvalue, 89
```
### M

```
magnitude, 128, 129, 131
maintainability, 2
managed overloading, 259, 260, 271, 282
mandatory call, 192
mandatory parameter, 41
```
**332 Index**


mantissa bit, 31
mapped value, 113
mask value, 132
match( _v._ ), **5**
member, 53, 176, 177, 186, 197
member access, 91
member field, 106
memory location, 38, 273
meta-annotation, 310, 311
metadata, 305, 311
metasymbol, 3, **5**
method, **5** , 29, 30, 32–34, 36, 38, 42, 46, 51, 54, 57, 60,
62, 64–68, 71, 73–77, 81, 88, 93, 94, 101, 103,
107, 108, 110, 115, 155, 160, 161, 163, 167,
171, 172, 176–180, 184–191, 197, 199, 200,
202, 205, 229, 235, 250–254, 256, 257, 259,
263, 269, 271, 277, 279–282, 284, 286–288,
291–293, 295, 297, 302–304, 307, 316, 317
method body, 65, 66, 104, 148, 161, 169, 186, 187, 193,
202, 229, 279, 293, 294
method body declaration, 54
method call, 39, 40, 48, 60, 61, 81, 82, 89, 92, 107,
108, 113, 163, 184, 232, 251, 261, 263, 280,
295, 299, 300, 303, 304, 316
method call expression, 65, 92, 107, 108, 115, 184,
263, 295
method declaration, 169, 172, 177, 184–187, 191,
200, 268, 282, 293, 294
method member, 177
method modifier, 108, 184, 189, 285
method name, 54, 184, 202, 291
method overload, 176, 261, 284
method overload declaration, 284, 285, 288
method overload signature, 256
method overloading, 271
method overriding, 254, 271
method parameter, 57, 65
method parameter name, 54
method reference, 93, 94, 137, 291
method return type, 66, 108, 252
method scope, **5** , 54
method signature, 94, 186, 188, 202, 254
migration, 2
mode of evaluation, 153
modelling, 267
modification, 159
modifier, 54, 159, 168, 169, 180, 182, 185, 187, 211
modifier async, 230, 271
modifier const, 159
modifier declare, 229
modifier export, 218
modifier let, 159
modifier static, 180
modularity, 2

```
module, 2, 53, 54, 74, 93, 150, 209–211, 213–215, 217,
220, 222–227, 229, 236, 262, 263, 284, 297,
315, 316
module initialization, 316
module level scope, 5 , 53
multiline comment, 21
multiline string, 18, 20, 145, 146
multiline string literal, 20
multimedia processing, 267
multiplication, 126, 127, 132
multiplication operator, 127
multiplicative expression, 30, 83, 125, 150
multiplicative operator, 29, 30, 125, 150
multitargeting, 2
mutable variable, 155
```
### N

```
name, 51–57, 60, 65, 93, 205, 211, 212, 215–217, 256,
259, 277, 280, 286, 289–291, 294, 297, 305
name binding, 213
name-value pair, 98, 99, 102
named class, 102
named constant, 203
named constructor, 93
named entity, 52
named function, 60
named reference, 92–94, 263, 294
named store location, 57
named type, 25, 27
named variable, 144
namespace, 53, 93, 217, 219–222, 235, 262, 263, 282,
284
namespace declaration, 53, 217, 219, 236, 262
namespace level scope, 53
namespace name, 220, 236
namespace scope, 284
namespace variable, 219
NaN, 31, 86, 124, 127–129, 131, 133, 135, 137, 265
Nan, 85
NaN value, 91
narrowed type, 253
narrowing, 252, 253
narrowing conversion, 5
native, 286
native constructor, 191, 293
native function, 60, 66, 268, 271, 292
native keyword, 292, 293
native method, 187, 271, 292, 293
native modifier, 186, 189
nearest value, 31
negation, 124, 131
negative infinity, 86, 133, 137
negative integer, 127, 128
negative integer value, 275
```
**Index 333**


negative zero, 131, 133, 137
nested literal, 95
nested loop, 276
nested multiline string, 146
nested namespace, 236
nested statement, 54, 162
nested union type, 45
never, 24
never type, 32, 33, 45, 47, 65, 66, 116
new expression, 280
newline character, 10
no-argument return statement, 268
no-break space, 10
non-generic, **5**
non-generic type, **5**
non-abstract class, 168, 169
non-abstract instance method, 186
non-abstract method, 186
non-abstract subclass, 186
non-alias, 45
non-aliased type, 28
non-ambient declaration, 230
non-ambient interface, 235
non-ambient method, 232
non-boolean type, 264
non-class type, 169
non-compatible signature, 256
non-empty body, 293
non-empty string, 264
non-exported declaration, 220
non-generic class, 73, 242
non-generic entity, 71
non-generic function, 73
non-generic interface, 73
non-generic method, 73
non-generic type, 27
non-generic type alias, 73
non-initialized variable, 57
non-interface type, 198
non-native constructor, 191
non-negative integer number, 38
non-nullish type, 45, 47, 106, 113, 121
non-nullish variant, 120
non-nullish-type, 68
non-numeric type, 111
non-optional field, 175
non-optional parameter, 104
non-optional property, 101
non-relative import path, 214
non-relative path, 214
non-standalone expression, 89
non-static class, 171, 177
non-static entity, 176
non-static field, 168, 179–181, 190, 194

```
non-static field declaration, 181
non-static member, 176
non-static method, 186
non-static modifier, 182, 292
non-string operand, 82, 239
non-union type, 45
nonterminal, 2, 3, 5
nonterminal symbol, 3, 5
nonzero, 264
nonzero double, 85
nonzero operand, 131
nonzero value, 31
normal completion, 90–92, 102, 111, 113, 123, 124,
141–144, 148, 150, 153, 163–165, 187, 207
normal execution, 207
normal method call, 280
normalization, 43, 45, 66
normalized union type, 45
notation, 61, 177, 180, 201, 221, 303, 308, 309, 311
notion, 182
null, 24, 26, 49, 113, 120, 137, 253
null expression, 162
null literal, 20, 35
null pointer dereferencing, 207
null reference, 20
null safety, 47
null type, 32, 35, 36, 75, 245
null-coalescing operator, 91
nullable array, 26
nullable reference type, 48
nullable type, 5 , 47
nullish expression, 265
nullish object reference, 106
nullish type, 20, 35, 47, 48, 82, 108, 110, 113, 120,
121, 181, 264
nullish value, 6 , 47, 48, 107, 109, 110, 113, 120, 121
nullish-coalescing assignment, 143
nullish-coalescing expression, 48, 121
nullish-coalescing operator, 121
nullish-safe option, 47, 48
nullish-type, 68
number, 2, 18, 49, 78, 264, 275, 291, 292
number type, 28, 30, 246
numeric base type, 84
numeric casting, 122
numeric casting conversion, 85, 86, 122
numeric constant expression, 203
numeric context, 83
numeric conversion, 83, 84, 86, 135, 136
numeric equality, 136, 137
numeric equality operator, 29, 30, 135, 136
numeric expression, 265
numeric literal, 2, 230
numeric literal type, 45
```
**334 Index**


numeric operand, 130
numeric operation, 31
numeric operator, 29, 30, 81, 239, 240
numeric operator context, 239
numeric promotion, 29–31
numeric relational operator, 29, 30, 133, 134
numeric type, 28–31, 36, 45, 78, 83–85, 97, 110, 116,
122, 124, 125, 127–131, 133, 136, 138, 139,
204, 239, 306
numeric type operand, 131
numeric types conversion, 31, 83, 110, 122, 124,
125, 127, 128, 130, 133, 136, 139
numeric value, 84, 86, 135
numeric widening, 124
numeric widening conversion, 130

### O

Object, 24, 28, 164, 169, 177, 194, 197, 198, 200, 242,
245, 251
object, 2, 32, 35, 38, 40, 58, 68, 89, 105, 106, 114, 130,
162, 186, 191, 197, 199, 240, 275, 279, 293, 317
object field, 192
Object literal, 252
object literal, 76–78, 81, 97–102, 104, 116, 237,
307, 308
object literal expression, 98
object orientation, 2
object reference, 105–107, 261, 286, 288, 295
object reference expression, 106, 110, 111, 113
object reference subexpression, 144
Object type, 45
object type, 254
object-oriented, 2
object-oriented programming( _OOP_ ), 263, 271
octal, 14
OOP( _object-oriented programming_ ), 2, 271
operand, **6** , 29–31, 82, 83, 86, 91, 115, 116, 122, 124–
137, 139–145, 237, 239, 272
operand expression, 126, 130, 138, 144, 145
operand null, 82
operand string, 130
operand type, 132, 237
operand value, 124–126, 130, 139
operation, **6** , 30, 36, 124, 130, 261, 263, 317
operation overflow, 131
operation sign, **6**
operation type, 37
operational function, 304
operator, 10, 13, 20, 29, 30, 38, 40, 90, 91, 110, 115,
118, 120, 121, 123–125, 128, 130, 132–134,
136, 138, 141, 162, 251
operator( _in programming languages_ ), **6**
operator context, 130
operator evaluation, 90

```
operator precedence, 91
operator sign, 2
operator undefined, 82
operator validity test, 28
optional annotation, 147
optional arbitrary code, 192
optional argument, 304
optional field, 175, 179, 180, 182
optional identifier, 191
optional name, 191
optional parameter, 41, 60, 61, 63, 147, 189, 230,
246, 275, 303, 304, 307
optional property, 76, 101, 200–202
ordinary function, 278
ordinary method, 279, 286, 288, 317
original variable, 149
out keyword, 70
out-variance, 67
overflow, 29, 31, 124, 126–132
overlap, 54
overlapping, 135
overload, 176, 217, 232, 283, 285, 286, 291
overload alias, 94, 212, 282–286, 288–292
overload alias name, 291
overload declaration, 107, 109, 176, 212, 217, 259–
261, 271, 282–291, 294
overload keyword, 282
overload resolution, 107, 109, 261, 283
overload set, 283
overload signature, 187, 202, 217, 256, 259, 260,
282, 284, 285
overloaded constructor, 289
overloaded entity, 282, 283, 291
overloaded function, 212, 284, 286, 290
overloaded interface, 287
overloaded method, 285–288, 291, 292
overloading, 6 , 227, 240, 254, 259, 282, 317
overloading signature, 184
overridden field, 183
overridden member, 197
overridden method, 188, 291, 293, 317
override, 183, 184, 288
override compatibility, 257, 258
override keyword, 182
override method, 250
override modifier, 169, 185, 187
override-compatibility, 254–256
override-compatible, 256
override-compatible signature, 172, 190, 195,
199, 202, 249, 254–257
overriding, 2, 88, 167, 172, 182–184, 186–188, 190,
195, 197, 202, 239, 240, 245, 250, 254–256,
258, 259, 278, 286, 291, 293, 294, 317
overriding entity, 249
```
**Index 335**


overriding field, 182–184
overriding method, 187, 189, 317
own( _adj._ ), **6**
own field, 179

### P

paragraph separator character, 10
parallel-run coroutine, 267
parameter, 33, 60, 65, 93, 104, 105, 109, 148, 188, 189,
194, 201, 227, 230, 246, 247, 249, 251, 253,
257, 258, 263, 275, 281, 295, 296, 299, 301,
303, 304, 307, 316
parameter constraint, 239
parameter declaration, 147
parameter list, 40, 60, 62
parameter name, 41, 54, 60, 62, 300
parameter type, 41, 61, 190, 246, 247, 250, 252, 259,
262
parameter with default value, 61
parameterization, 67, 68, 73
parameterized class type, 171
parameterized declaration, 73
parameterized interface, 198
parameterized type, 198
parameterless constructor, 99, 102, 275, 276
parameterless function type, 281
parameterless method, 279
parent class, 254
parenthesis, 13, 26, 91, 92, 115, 142, 308
parenthesized expression, 104, 150
path, 207, 315
path component, 214
path mapping, 214
path rule, 214
pattern, 88
performance, 267
permutation, 249
platform API, 235
platform-dependent code, 293
point of declaration, 54, 263
policy, 311
polymorphism, 282
polymorphism by name, 259, 282
positive infinity, 86, 133, 137
positive zero, 131, 133, 137
postfix, 29, 30, 113, 120, 122
postfix operator, 91
precedence, 26, 54, 91, 134
predefined constant, 29, 31, 272
predefined constructor, 29, 31, 272
predefined method, 29, 31, 272
predefined numeric types conversion, 29–31,
125, 133
predefined operator, 90

```
predefined reference type, 18
predefined type, 23, 24, 27, 28, 42, 44, 83, 123
predefined type declaration, 47
predefined value type, 143, 150
prefix, 16, 17, 29, 30, 40, 62–64, 113, 120, 123, 150,
229, 235, 302, 305, 306
prefix operator, 91
prefix readonly, 26, 39, 60, 61
primary constructor, 191–193
primary expression, 105, 110, 113
primary name, 24
primitive type, 317
primitive type operation, 317
primordial class, 194
private, 177, 178, 195, 285, 286
private field, 79, 173, 175, 190
private member, 169, 178, 296
private method, 170, 202, 254–256, 294
private modifier, 186
production, 2, 3, 6
program completion, 270
program entity, 2, 67
program entry point, 226, 227
promise, 75
promise object, 269
promoted operand value, 124
promoted type, 130, 132
promoted value, 124
promoting, 130
promotion, 124, 125, 132
propagation, 165
property, 42, 76, 77, 100, 101, 173–175, 179, 197, 200–
202, 308
property length, 38
property type, 200
protected, 177, 178, 285
protected member, 296
protected modifier, 178
provably distinct instantiation, 73
proxy, 269
pseudogeneric static method, 316
public, 177, 178, 255, 285, 286
public member, 296
public method, 200
public modifier, 178, 194
punctuator, 6 , 10, 13
```
### Q

```
qualification, 179, 203, 269
qualified access, 51, 54
qualified form, 210
qualified import, 53
qualified name, 6 , 51, 53, 92, 93, 109, 176, 177, 179,
180, 211, 216, 219, 221, 284, 309
```
**336 Index**


qualified type name, 27, 29
qualifier, 219

### R

radix, 14, 17, 18
re-assignment, 47
re-export, 223–225
re-export declaration, 225
re-export directive, 209, 225
re-exported declaration, 225
re-exporting declaration, 223
read permission, 207
readability, 17, 18, 303
readonly, 40, 45, 70, 90, 97, 106, 173, 174, 180, 193,
201, 202
readonly array, 141, 142
readonly array type, 39
readonly field, 102, 106, 174, 180, 183, 311
readonly modifier, 180
readonly name, 312
readonly parameter, 61
readonly tuple, 141, 142
readonly type, 26, 77
reassignment, 77
receiver, 253, 295, 297
receiver body, 302
receiver parameter, 298, 299, 303
receiver type, 296–299, 302, 303
record, 110
record access expression, 142, 144
record element, 144
record indexing expression, 93, 113, 142, 144
record instance, 113, 142, 144
Record type, 78
record type, 101, 110, 112, 116
record utility type, 78
recursive reference, 56
recursive replacement, 28
reference, 32, 197
reference expression, 110, 111
reference subexpression, 142
reference type, 32, 35, 36, 40, 43, 47, 48, 51, 82, 106,
110, 113, 142, 143, 167, 168, 197
reflexive closure, 240
rejected promise, 270
rejection handler, 270
relational expression, 83, 132
relational operator, 31, 35, 91, 132, 134, 150
relative import path, 214
relative location, 214
relative path, 214
remainder operation, 128, 129
remainder operator, 29, 31, 90, 128, 129
renaming, 223, 309

```
repeatable annotation, 308
replacement, 28
required parameter, 60
required property, 77, 200, 201
reserved word, 12
resizable array, 38, 92, 114, 240, 273, 275
resizable array type, 6 , 37, 38
resolution, 136, 214, 269
resolution process, 260
resolving, 214
rest parameter, 41, 60, 62–64, 104, 246, 251
restriction, 78, 112, 181, 233, 234
retention annotation, 311
retention policy, 311
return, 33, 154, 164
return expression, 161
return statement, 65, 66, 148, 154, 161, 187, 193,
226, 227, 268
return type, 33, 34, 40, 60, 65, 66, 108, 109, 147, 148,
154, 161, 177, 187–191, 201, 227, 230, 234,
246, 252, 257, 258, 268, 279, 299
right shift, 132
root coroutine, 269
round to nearest, 31
round toward zero, 31
round-to-nearest mode, 84, 127, 128, 131
round-toward-zero, 86
rounding, 84, 127, 128, 131
rounding mode, 31
rounding rule, 85, 86
rounding rules, 86
rounding toward zero, 31
routine, 128, 262
runtime, 36, 38, 47, 90, 103, 105, 106, 111, 116, 117,
122, 123, 125, 130, 132, 135, 139–143, 148,
149, 164, 165, 180–182, 254, 263, 271, 273,
276, 311, 316
runtime check, 117, 247
runtime error, 42, 84–86, 90, 110, 116–118, 207, 235,
273, 275, 310
runtime evaluation, 148, 263, 276
runtime expression, 271
runtime implementation, 316
runtime polymorphism, 254
runtime system, 208, 247
runtime type, 263
```
### S

```
safe field access, 48, 106
safe function call, 48
safe indexing expression, 48
safe method call, 48, 107
safe operation, 48
safety, 263
```
**Index 337**


scope, 51, 53–55, 67, 147, 156, 165, 167, 168, 176, 198,
200, 211, 215, 216, 225, 263, 284, 303
scope of a name, **6**
secondary constructor, 191, 193
selective binding, 212, 223
selective export, 223
selective export directive, 223
selective import, 211
semantic check, 108, 109, 115, 195, 202, 213, 224,
251, 253, 255, 256
semantic correctness check, 109
semantic equivalent, 200
semantic rule, 237, 239
semantic term, 237
semantics, 47, 135, 137, 140, 180, 191, 193, 203, 226,
231, 232, 239, 240, 257, 261, 264, 265, 269,
282, 283, 305, 310
semi-automatic transition, 2
semicolon, 13, 21, 187, 293, 303
separator, 3, 51, 145, 303
sequence, 3, 35
set, 260
set of functions, 284
set of methods, 284
set of values, 272
set-accessor, 189, 299
setter, 76, 77, 100, 144, 173, 174, 188–190, 200–202,
299
setter parameter, 190
shadow, 184
shadow parameter, 147
shadowing, 65, 147, 185, 195, 197, 198
shared memory, 267
shift, 131, 132
shift distance, 131, 132
shift expression, 30, 83, 131, 132, 150
shift operation, 132
shift operator, 91, 131, 150
short, 28, 29, 49, 84, 86
shortcut notation, 221
side effect, 89, 126, 130, 134, 138–140
sign-extension, 132
signature, 40, 46, 52, 60, 186–188, 202, 228, 249, 254,
256, 257, 259–261, 277–279, 281, 283, 287,
299, 317
signature resolution, 260
signed infinity, 31, 127, 128, 131
signed integer comparison, 133
signed right shift, 131
signed shift operator, 29, 30
signed zero, 31
simple assignment operator, 91, 141
simple name, **7** , 51, 53, 92, 93, 212, 216, 303
simple type name, 27, 29

```
simulation, 267
single quote, 19, 272
slash character, 214
smart cast, 115, 118, 253, 261
smart compiler, 252
smart type, 252–254, 261
smart typing, 253
soft keyword, 12, 13
source, 214, 311
source code, 10, 23, 317
source file, 315
source-level compatibility, 203
space, 10, 305
space allocation, 276
special character, 2
specified type, 263
spread, 104
spread expression, 63, 64, 88, 103, 251
spread operator, 62–64, 251
square bracket, 13, 40
standalone expression, 89, 145, 237–239
standard annotation, 310, 311
standard library, 30, 38, 128, 207–209, 216
state, 28, 32
statement, 21, 90, 153–158, 160, 161, 163, 165, 225–
227
statement execution, 153
static, 180
static block, 176, 177, 262
static class, 171
static data member, 179
static dispatch, 263, 297
static entity, 54, 176
static field, 106, 179, 180, 262, 263
static field access, 106
static initialization, 219, 262, 263, 316
static initializer, 268
static keyword, 293
static member, 7 , 51, 53, 54, 88, 167, 176, 177
static member declaration, 54
static method, 36, 94, 107–109, 184–186, 189, 276,
280, 281, 284, 316
static method call, 108
static modifier, 182, 185–187, 292
static overload alias, 285
static override field, 183
static type, 65, 253, 254
statically typed language, 23
sting concatenation, 130
sting literal, 316
sting type, 130
storage, 211, 225, 315
storage management, 2, 315
```
**338 Index**


string, 18, 24, 26, 28–32, 35–37, 42, 45, 66, 73, 81,
89, 110–113, 130, 135, 143, 145, 158, 162, 173,
201, 220, 246, 251, 262, 264, 278, 291, 292,
299, 304
string comparison, 133
string concatenation, 30, 36, 130, 143, 239
string concatenation operator, 29–31, 145, 146
string context, 82, 83
string conversion, 82, 83, 130, 239
string element, 111
string indexing, 111
string interpolation, 20
string interpolation expression, 145
string length, 111
string literal, 18–20, 35, 36, 145, 214, 230, 245, 311
string literal type, 78, 135
string object, 36, 130
string operand, 82
string operator, 239, 240
string operator context, 239
string relational operator, 133, 134
string type, 32, 36, 78, 82, 85, 111, 113, 118, 130, 135,
145, 150, 203–205, 227, 239, 245, 278, 306, 311
string value, 36, 111, 133, 134, 205
strong typing, 252
struct keyword, 316
structured concurrency, 267
structured coroutine, 270
structuring rule, 3
subclass, 105, 168–170, 177, 187, 188, 208, 240, 254,
255, 263, 286, 291, 293, 317
subclasssubinterface, 242
subcomponent( _derived component, child component_ ), **7**
subexpression, 90, 110, 142, 143
subinterface, 171, 177, 198, 240, 254, 256, 288
subset, 189, 214
substitution, 171, 198
subtraction, 13, 122–124, 131
subtype, 32, 68, 73, 90, 115, 117, 239, 240, 242, 245–
250, 257, 259, 269, 279
subtyping, 37, 41, 45, 59, 68, 70, 105, 115, 239, 240,
245–249, 257, 279, 300
super, 88, 181, 192, 195
super call, 255
super keyword, 148, 180, 185, 194
superclass, 88, 99, 105–107, 167–169, 171, 179, 180,
182–184, 187, 193–195, 197, 240, 242, 245,
254, 255, 257, 286, 291
superclass constructor, 183, 194
superclass constructor call, 194
superclass property, 106
supercomponent( _base component, parent component_ ),
**7**
superconstructor, 192

```
superinstance, 177
superinterface, 167, 171–173, 175, 179, 182–184,
195, 197, 198, 202, 240, 242, 245, 254, 256,
257, 288
supertype, 34, 37, 48, 59, 97, 240, 242, 245, 247, 250,
254, 259, 261
surrounding class, 302
surrounding context, 252
surrounding function, 155, 160, 161
surrounding interface, 302
surrounding method, 155, 160, 161
surrounding scope, 165
surrounding type, 148
switch expression, 161, 162
switch statement, 159–162
sybtyping, 117
synchronization, 263, 267
syntactic grammar, 3, 10, 21
syntactic notation, 2
syntactic structure, 2
syntactical form, 114
syntax, 1, 38–41, 62, 65, 67, 68, 88, 92, 95, 102–105,
107, 108, 114, 115, 118, 125, 130–132, 134,
138–141, 145, 150, 154–164, 169, 171, 176,
179, 184, 191, 197, 200, 202, 203, 209, 210,
213, 217, 219, 223–225, 229–235, 269, 272,
275, 284, 288, 289, 294, 299, 301–303, 306–
308, 310
syntax production, 21
```
### T

```
target type, 82, 83, 85, 86, 89, 90, 116, 117, 237, 239,
261
template, 2
terminal, 3, 7
terminal symbol, 3, 7 , 9, 10
termination, 161
ternary conditional expression, 31, 145, 264
ternary conditional operator, 29–31, 145, 150
ternary operator, 91
then-block, 156
third-party library API, 235
this, 181, 192, 302
this keyword, 65, 104, 105, 148, 180, 185, 186, 188,
192, 194, 295, 296, 299, 301, 302
this method, 181
this statement, 188
throw, 31, 33
throw statement, 163, 208
thrown object, 163
thrown value, 163
token, 2, 7 , 9, 10, 13, 51
tokenization, 7 , 10
```
**Index 339**


top-level declaration, 209, 217, 218, 223, 236, 295,
299, 307
top-level function, 316
top-level overload declaration, 290
top-level statement, 60, 161, 209, 222, 225–227,
236, 262
top-level statements, 227
top-level type, 217
top-level variable, 65, 217
trailing comma, 95, 98
trailing lambda, 107, 302–304
trailing lambda call, 107, 108
transitive closure, 170, 198, 240
traversing, 279
treemap, 214
truncated number, 31
truncation, 31, 36, 125, 128, 131, 132, 139
truthiness, 264
try block, 163, 165
try statement, 163, 165, 208
try-catch, 164
tuple, 40, 95, 97, 104, 141, 251
tuple argument, 63
tuple type, 23, 25, 26, 40, 61–63, 72, 103, 240, 249,
262
two’s-complement format, 126, 130
two’s-complement integer, 132
two’s-complement representation, 124
two’s-complement value, 124
type, 24, 28, 44, 49, 51, 58–60, 63, 73, 76–79, 81, 89, 90,
92, 96, 97, 99, 104, 107, 110–113, 115, 117,
120–122, 132, 135, 155, 157, 158, 163, 169,
171, 176, 182, 188, 204, 205, 211, 224, 237–
240, 245–247, 249, 252, 253, 257, 261–263,
275, 277, 278, 281, 288, 295, 298, 306, 307,
309, 310, 316
type alias, 25, 27–29, 38, 41, 45, 55, 56, 67, 68, 71,
73, 249, 306
type alias declaration, 27, 56, 249
type annotation, 34–36, 57–59, 65, 89, 96, 108, 109,
147, 201, 230, 252, 282
type argument, 27, 29, 34, 35, 56, 68, 69, 71–74, 93,
94, 171, 198, 262, 283, 316
type boolean, 138
type call expression, 280, 281
type cast, 115
type char, 272
type checking, 259
type declaration, 23, 40, 43, 47, 55, 85, 154, 217, 226
type enum, 203
type enumeration, 204
type erasure, 115, 117, 261, 262, 316, 317
type for annotation field, 306
type identity, 38, 249

```
type import, 213
type in parentheses, 25
type inference, 23, 45, 47, 58, 59, 65, 66, 74, 81, 89,
95–100, 116, 125, 147, 148, 189, 190, 238–240,
252, 279
type instantiation, 69
type int, 83, 84, 203, 275
type Iterator, 279
type mapping, 261, 262
type modifier, 211, 213
type name, 27, 29, 36, 115, 280
type null, 20
type of expression, 237
type parameter, 23, 25, 27, 32, 34, 48, 54, 56, 60, 67–
71, 73, 107, 115, 120, 147, 177, 239, 245, 249,
257, 258, 262, 276, 283, 296, 298, 317
type parameter declaration, 27
type parameter scope, 7
type parameterized entity, 2
type preservation, 262
type property, 101
type readonly, 77
type reference, 7 , 25, 27–29, 68, 115
type safety, 45, 247, 252
type string, 35, 204
type structure, 26
type undefined, 21
type void, 65
type-safe call, 317
typed catch clause, 163
typeof expression, 118
typeof operator, 91
types conversion, 139
```
### U

```
unary bitwise complement expression, 125
unary expression, 83, 122
unary logical complement expression, 125
unary minus, 124, 150
unary minus operator, 30
unary negation operation, 124
unary numeric expression, 239
unary numeric promotion, 124
unary operator, 29, 30, 91, 122, 124, 125, 150
unary plus, 124, 150
unary plus expression, 124
unary plus operator, 30, 124
undefined, 24, 61, 109, 113, 120, 137, 180
undefined literal, 21, 35
undefined type, 32, 34–36, 41, 48, 49, 75, 245
undefined value, 47, 101
underflow, 29, 31, 127–129, 131
underscore character, 16–18
unhandled promise, 270
```
**340 Index**


unhandled rejection, 270
Unicode character, 9, 11
Unicode code point, 9, 11, 272
Unicode code unit, 35
Unicode escape sequence, 19
Unicode input character, 10
Unicode Standard, 11
uninitialized field, 181
union, 32, 43, 44, 71, 107, 169, 279
union component type, 98
union type, 23, 25–27, 32, 43–45, 47, 55, 66, 72, 78,
97, 98, 101, 102, 112, 120, 121, 136, 246, 249,
253, 258, 264
union type normalization, 68, 97, 145, 246
unqualified form, 210
unqualified identifier, 67
unqualified import, 309
unqualified name, 51, 53
unsigned right shift, 131
unsigned shift operator, 29, 30
user-defined entity, 74
user-defined getter, 76
user-defined setter, 76
user-defined type, 23, 25, 28, 47, 203
user-defined type declaration, 47
utility type, 67, 74, 75, 77–79, 101, 110, 112, 116,
169

### V

value, 13, 16, 23, 24, 28, 30, 33–36, 38, 40, 42–44, 54,
57, 58, 78, 81, 84, 85, 89–92, 95, 99, 101–
105, 111–113, 116, 118, 122–126, 128, 132–
135, 137, 141–144, 163, 180, 188, 192, 193,
197, 202–205, 218, 230, 249, 252, 263, 272,
274, 276, 277, 306–308, 311, 316
value equality, 136, 137, 272
value equality operator, 272
value name, 36
value set, 131, 142
value set conversion, 124
value type, 28, 36, 45, 78, 101, 150, 264, 316
variable, **7** , 28, 32, 33, 37, 38, 48, 51, 53, 57, 58, 76–
78, 81, 84, 88, 89, 91, 93, 96, 97, 106, 110, 111,
122–126, 141–144, 149, 158, 159, 197, 217,
220, 238, 253, 262, 263, 277, 282, 316
variable declaration, **7** , 57, 81, 96, 179, 217, 225,
252
variance, 67, 70, 71, 245, 249, 250
variance interleaving, 71
variance modifier, 70, 71
vertical tab, 19
virtual machine, 90
void, 24
void type, 32, 34, 65, 66, 161, 227

### W

```
well-formed instantiation, 73
well-formed parameterized type, 198
while statement, 157, 160, 264
white space, 2, 7 , 9, 10, 305
widening, 29, 31, 84, 136, 239
widening conversion, 7 , 110, 136
widening numeric conversion, 83, 125, 127, 128,
130, 136, 139
```
### Z

```
zero-extension, 132
zero-width joiner, 11
zero-width no-break space, 10
zero-width non-joiner, 11
```
**Index 341**



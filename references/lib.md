# ArkTS Standard Library

## Release 1.2.0-alpha

**2025.05.**



**CONTENTS**

**1 Functions 1
2 Classes 5**

```
i
```

**ii**


### CHAPTER

**ONE**

**FUNCTIONS**

exportdecodeURIComponent(uriComponent: _string_ ): _string_
The decodeURIComponent() function decodes a Uniform Resource Identifier (URI) component previously created byencodeURIComponent() or by a similar routine.

**Returns:** decoded uri
**Arguments:**

- uriComponent: _string_

exportencodeURI(uri: _string_ ): _string_
The encodeURI() function encodes a URI by replacing each instance of certain characters by one, two, three, or fourescape sequences representing the UTF-8 encoding of the character (will only be four escape sequences for characters
composed of two surrogate characters). Compared to encodeURIComponent(), this function encodes fewercharacters, preserving those that are part of the URI syntax.

**Returns:** encoded uri
**Arguments:**

- uri: _string_

exportencodeURIComponent(uriComponent: _string_ ): _string_
The encodeURIComponent() function encodes a URI by replacing each instance of certain characters by one, two,three, or four escape sequences representing the UTF-8 encoding of the character (will only be four escape sequences
for characters composed of two surrogate characters). Compared to encodeURI(), this function encodes morecharacters, including those that are part of the URI syntax.

**Returns:** encoded uri

```
1
```

**Arguments:**

- uriComponent: _string_

exportescape(str: _string_ ): _string_
**DEPRECATED:** already been removed from the relevant web standards, may be in the process of being dropped, or may only be keptThis feature is no longer recommended. Though some browsers might still support it, it may have
for compatibility purposes. Avoid using it, and update existing code if possible; see the compatibility table at thebottom of this page to guide your decision. Be aware that this feature may cease to work at any time.

**Returns:** string with hexadecimal escape sequences
**Arguments:**

- str: _string_
**Note:** compatibility. It is not required to be implemented by all JavaScript engines and may not work everywhere. Useescape() is a non-standard function implemented by browsers and was only standardized for cross-engine
encodeURIComponent() or encodeURI() if possible.The escape() function computes a new string in which certain characters have been replaced by hexadecimal escape
sequences.

exportunescape(str: _string_ ): _string_
**DEPRECATED:** already been removed from the relevant web standards, may be in the process of being dropped, or may only be keptThis feature is no longer recommended. Though some browsers might still support it, it may have
for compatibility purposes. Avoid using it, and update existing code if possible; see the compatibility table at thebottom of this page to guide your decision. Be aware that this feature may cease to work at any time.

**Returns:** unexcaped string
**Arguments:**

- str: _string_
**Note:** compatibility. It is not required to be implemented by all JavaScript engines and may not work everywhere. Useunescape() is a non-standard function implemented by browsers and was only standardized for cross-engine
decodeURIComponent() or decodeURI() if possible.The unescape() function computes a new string in which hexadecimal escape sequences are replaced with the
characters that they represent. The escape sequences might be introduced by a function like escape().

**2 Chapter 1. Functions**


-


**4 Chapter 1. Functions**


### CHAPTER

**TWO**

**CLASSES**

**2.1 Array<T>**
export
Represents JS API-compatible Array

**2.1.1 Methods**
publicat(index:number): T |null
Takes an integer value and returns the item at that index, allowing for positive and negative integers. Negative integerscount back from the last item in the array.

**Returns:** `length()`.The element in the array matching the given index. Returns null if`index`<`-length()`or`index`>=

**Arguments:**

- index:of the array — ifnumberZero-based index of the array element to be returned. Negative index counts back from the end`index`< 0, index +`array.length()`is accessed.

publicconcat(other: _Array_ <T>): _Array_ <T>
Creates a new`Array`from this`Array`instance and given`Array`instance.
**Returns:** New`Array`instance, constructed from`this`and given`other`instances of`Array`class.
**Arguments:**

- other: _Array_ <T> to concatenate into a new array.
publicconstructor():void
**5**


Creates a new empty instance of Array

publicconstructor(d: T[]):void
Creates a new instance of Array based on object[]
**Arguments:**

- d: T[] Array initializer
publiccopyWithin( target:number, start:number, end:number): _Array_ <T>
Makes a shallow copy of the Array part to another location in the same Array and returns it without modifying itslength.

**Returns:** this array after transformation
**Arguments:**

- • target:start:numbernumberindex at which to start copying elements fromindex at which to copy the sequence
- end:numberindex at which to end copying elements from
publiccopyWithin(target:number, start:number): _Array_ <T>
Makes a shallow copy of the Array part to another location in the same Array and returns it without modifying itslength.

**Returns:** this array after transformation
**Arguments:**

- target:numberindex at which to copy the sequence
- start:numberindex at which to start copying elements from
publiccopyWithin(target:number): _Array_ <T>
Makes a shallow copy of the Array part to another location in the same Array and returns it without modifying itslength.

**6 Chapter 2. Classes**


**Returns:** this array after transformation
**Arguments:**

- target:numberindex at which to copy the sequence
publicevery( fn: (v: T, k:number) =>boolean):boolean
Tests whether all elements in the array pass the test implemented by the provided function. It returns a Boolean value.
**Returns:** `true`if`fn`returns a`true`value for every array element. Otherwise,`false`.
**Arguments:**
- fn: (v: T, k:indicate the element passes the test, and anumber) =>booleanfunction to execute for each element in the array. It should return a`false`value otherwise. `true`to

publicevery( fn: (v: T) =>boolean):boolean
Tests whether all elements in the array pass the test implemented by the provided function. It returns a Boolean value.
**Returns:** `true`if`fn`returns a`true`value for every array element. Otherwise,`false`.
**Arguments:**

- fn: (v: T) =>element passes the test, and abooleanfunction to execute for each element in the array. It should return a`false`value otherwise. `true`to indicate the

publicfill( value: T, start:number, end:number): _Array_ <T>
Changes all elements in the Array to a static value, from a start index to an end index
**Returns:** this array after transformation
**Arguments:**

- value: T to fill the array with
- start:numberindex at which to start filling
- end:numberindex at which to end filling, but not including
publicfill(value: T): _Array_ <T>
**2.1. Array<T> 7**


Changes all elements in the Array to a static value
**Returns:** this array after transformation
**Arguments:**

- value: T to fill the array with

publicfilter( fn: (v: T, k:number) =>boolean): _Array_ <T>
Constructs a newfrom the given array that pass the test implemented by the provided function.`Array`instance and populates it with portion of a given array, filtered down to just the elements

**Returns:** New`Array`instance constructed from`this`with elements filtered using test function`fn`.
**Arguments:**

- fn: (v: T, k:number) =>booleantest function, applied to each element of an array.

publicfilter( fn: (v: T) =>boolean): _Array_ <T>
Creates a newthe given array that pass the test implemented by the provided function.`Array`instance and populates it with portion of a given array, filtered down to just the elements from

**Returns:** New`Array`instance constructed from`this`with elements filtered using test function`fn`.
**Arguments:**

- fn: (v: T) =>booleantest function, applied to each element of an array.

publicfind( fn: (elem: T) =>boolean): T |null
Returns the first element in the provided array that satisfies the provided testing function
**Returns:** found element or null otherwise
**Arguments:**

- fn: (elem: T) =>booleantesting function

publicfindIndex( fn: (elem: T) =>boolean):number

**8 Chapter 2. Classes**


Returns the index of the first element in an array that satisfies the provided testing function
**Returns:** found element index or -1 otherwise
**Arguments:**

- fn: (elem: T) =>booleantesting function

publicfindLast( fn: (elem: T) =>boolean): T |null
Iterates the array in reverse order and returns the value of the first element that satisfies the provided testing function
**Returns:** found element or null otherwise
**Arguments:**

- fn: (elem: T) =>booleantesting function

publicfindLastIndex( fn: (element: T) =>boolean):number
Iterates the array in reverse order and returns the index of the first element that satisfies the provided testing function.If no elements satisfy the testing function, -1 is returned.

**Returns:** index of first element satisfying to fn, -1 if no such element
**Arguments:**

- fn: (element: T) =>booleantesting function

publicflat(depth:number): _Array_ < _object_ |null>
Creates a new Array with all sub-array elements concatenated into it recursively up to the specified depth.
**Returns:** a flattened Array with respect to depth
**Arguments:**

- depth:number

publicflat(): _Array_ < _object_ |null>

**2.1. Array<T> 9**


Creates a new Array with all sub-array elements concatenated
**Returns:** a flattened Array

publicflatMap( fn: (v: T, k:number) => _object_ ): _Array_ < _object_ |null>
Applies flat and than mapfn a function to apply

**Return:** new Array after map and than flat

publicflatMap( fn: (v: T) => _object_ ): _Array_ < _object_ |null>
Applies flat and than mapfn a function to apply

**Return:** new Array after map and than flat

publicforEach( fn: (a: T) =>void):void
Executes a provided function once for each array element.
**Arguments:**

- fn: (a: T) =>voidto apply for each element of the Array

publicincludes(val: T):boolean
Checks whether an Array includes a certain value among its entries, returning true or false as appropriate.
**Returns:** true if val is in Array
**Arguments:**

- val: T value to search
**10 Chapter 2. Classes**


publicindexOf(val: T):number
Returns the first index at which a given element can be found in the array, or -1 if it is not present.
**Returns:** index of val, -1 otherwise
**Arguments:**

- val: T value to search
publicjoin(sep: _string_ ): _string_
Creates and returns a new string by concatenating all of the elements in anstring. If the array has only one item, then that item will be returned without using the separator.`Array`, separated by a specified separator

**Returns:** A string with all array elements joined. If`length()`is 0, the empty string is returned.
**Arguments:**

- sep: _string_ specifies a separator
publicjoin(): _string_
Creates and returns a new string by concatenating all of the elements in anarray has only one item, then that item will be returned without using the separator.`Array`, separated by a comma. If the

**Returns:** A string with all array elements joined. If`length()`is 0, the empty string is returned.

publickeys(): _IterableIterator_ <number>
Returns an iterator over all indices

publiclastIndexOf(element: T, fromIndex:number):number
Returns the last index at which a given element can be found in the array, or -1 if it is not present. The array issearched backwards, starting at fromIndex.

**2.1. Array<T> 11**


**Returns:** The last index of the element in the array; -1 if not found.\
**Arguments:**

- element: T element to locate in the array.
- fromIndex:the end of the array — ifnumberzero-based index at which to start searching backwards. Negative index counts back from`fromIndex`< 0,`fromIndex`+`length()`is used. If`fromIndex`<`-length()`, the
    array is not searched and -1 is returned. Ifentire array to be searched. `fromIndex`>=`length()`then`array.length - 1`is used, causing the

publiclastIndexOf(element: T):number
Returns the last index at which a given element can be found in the array, or -1 if it is not present.
**Returns:** The last index of the element in the array; -1 if not found.
**Arguments:**

- element: T to find in the array.

publiclength():number
Returns the number of elements in the Array.
**Returns:** Element count in the`Array`instance.

publicof(items: T[]): Array<T>
Creates a new`Array`object from initializer
**Returns:** `Array`instance, constructed from`this`and given argument.
**Arguments:**

- items: T[]

publicmap<U>( fn: (v: T, k:number) => U ): _Array_ <U>

**12 Chapter 2. Classes**


Creates a newinstance of`Array`Array`class.`object and populates it with the results of calling a provided function on every element in`this`

**Returns:** `Array`instance, constructed from`this`and given function.
**Arguments:**

- fn: (v: T, k:number) => U mapping function, applied to each element of an array.

publicpop(): T |null
Removes the last element from an array and returns that element. This method changes the length of the array.
**Returns:** removed element

publicpush(val: T):number
Adds the specified elements to the end of an array and returns the new length of the array.
**Returns:** new length

publicreduce<U>( fn: (a: U, b: T) => U, initVal: U ): U
Executes a user-supplied "reducer" callback function on each element of the array, in order, passing in the returnvalue from the calculation on the preceding element. The final result of running the reducer across all elements of the
array is a single value. Order is from left-to-right.
**Returns:** a result after applying fn over all elements of the Array
**Arguments:**

- fn: (a: U, b: T) => U reduce function
- initVal: U start value
publicreduce( fn: (a: T, b: T) => T ): T

**2.1. Array<T> 13**


Executes a user-supplied "reducer" callback function on each element of the array, in order, passing in the returnvalue from the calculation on the preceding element. The final result of running the reducer across all elements of the
array is a single value. Order is from left-to-right.
**Returns:** a result after applying fn over all elements of the Array
**Arguments:**

- fn: (a: T, b: T) => T reduce function

publicreduceRight<U>( fn: (a: U, b: T) => U, initVal: U ): U
Executes a user-supplied "reducer" callback function on each element of the array, in order, passing in the returnvalue from the calculation on the preceding element. The final result of running the reducer across all elements of the
array is a single value. Order is from right-to-left.
**Returns:** a result after applying fn over all elements of the Array
**Arguments:**

- • fn: (a: U, b: T) => U reduce functioninitVal: U start value

publicreduceRight( fn: (a: T, b: T) => T ): T
Executes a user-supplied "reducer" callback function on each element of the array, in order, passing in the returnvalue from the calculation on the preceding element. The final result of running the reducer across all elements of the
array is a single value. Order is from right-to-left.
**Returns:** a result after applying fn over all elements of the Array
**Arguments:**

- fn: (a: T, b: T) => T reduce function

publicreverse():void
Modifiesto that previously stated.`this`instance of`Array`class and populates it with same elements ordered towards the direction opposite

**Note:** Mutating method

**14 Chapter 2. Classes**


publicshift(): T |null
Removes the first element from an array and returns that removed element. This method changes the length of thearray.

**Returns:** shifted element, i.e. that was at index zero

publicslice(start:number, end:number): _Array_ <T>
Creates a new`end`(`end`not included) where`Array`object and populates it with elements of`start`and`end`represent the index of items in that array.`this`instance of`Array`class selected from`start`to

**Returns:** `Array`instance, constructed from extracted elements of`this`instance.
**Arguments:**

- start:numberzero-based index at which to start extraction
- end:numberzero-based index at which to end extraction.`slice()`extracts up to but not including end.

publicslice(start:number): _Array_ <T>
Creates a new`Int.MAX_VALUE`Array`, which means ‘to the end of an array’.`object and populates it with elements of`this`instance of`Array`class selected from`start`to

**Returns:** `Array`instance, constructed from extracted elements of`this`instance.
**Arguments:**

- start:numberzero-based index at which to start extraction

publicslice(): _Array_ <T>
Creates a new`Array`object and populates it with elements of`this`instance of`Array`class
**Returns:** `Array`instance, constructed all elements of`this`instance.
**Note:** This method creates full copy of original`Array`instance.

**2.1. Array<T> 15**


publicsome( fn: (v: T, k:number) =>boolean):boolean
Tests whether at least one element in the array pass the test implemented by the provided function. It returns aBoolean value.

**Returns:** `true`if`fn`returns a`true`value for at least one array element. Otherwise,`false`.
**Arguments:**

- fn: (v: T, k:indicate the element passes the test, and anumber) =>booleanfunction to execute for each element in the array. It should return a`false`value otherwise. `true`to

publicsort( comparator: (a: T, b: T) =>number):void
Reorders elements of`this`using comparator function.
**Arguments:**

- comparator: (a: T, b: T) =>numberfunction that defines the sort order.
**Note:** TODO clarify UTF-16 or UTF-8Mutating method

publicsort():void
Reorders elements ofelements into strings, then comparing their sequences of UTF-16 code units values.`this`using a default comparator. Elements sorted in ascending order built upon converting the

**Note:** TODO clarify UTF-16 or UTF-8Mutating method

publicsplice(start:number, delete:number): _Array_ <T>
Changes the contents of an array by removing or replacing existing elements and/or adding new elements in place.
**Returns:** an Array with deleted elements
**16 Chapter 2. Classes**


**Arguments:**

- start:numberindex
- delete:numbernumber of items after start index
publicsplice(start:number): _Array_ <T>
Changes the contents of an array by removing or replacing existing elements and/or adding new elements in place.
**Returns:** an Array with deleted elements from start to the last element of the current instance
**Arguments:**
- start:numberindex
publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Returns a locale string representing the specified array and its elements.
**Returns:** string representation
**Arguments:**
- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Returns a locale string representing the specified array and its elements.
**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Returns a locale string representing the specified array and its elements.
**2.1. Array<T> 17**


**Returns:** string representation

publictoReversed(): _Array_ <T>
Copying version of the reverse() method. It returns a new array with the elements in reversed order.
**Returns:** reversed copy of the current Array

publictoSorted(): _Array_ <T>
Copying version of the sort() method. It returns a new array with the elements sorted in ascending order.
**Returns:** sorted copy of hte current instance using default comparator

publictoSorted( comparator: (a: T, b: T) =>number): _Array_ <T>
Copying version of the sort() method. It returns a new array with the elements sorted in ascending order.
**Returns:** sorted copy of the current instance comparator
**Arguments:**

- comparator: (a: T, b: T) =>numberfunction to compare to elements of the Array

publictoSpliced(start:number, delete:number): _Array_ <T>
Copying version of the splice() method.
**Returns:** a new Array with some elements removed and/or replaced at a given index.
**Arguments:**

- start:numberindex

**18 Chapter 2. Classes**


- delete:numbernumber of items after start index
publictoSpliced(start:number): _Array_ <T>
Copying version of the splice() method.
**Returns:** a new Array with some elements removed and/or replaced at a given index.
**Arguments:**
- start:numberindex

public overridetoString(): _string_
Returns a string representing the specified array and its elements.
**Returns:** string representation

publicvalues(): _IterableIterator_ <T>
Returns an iterator over all values

publicwith(index:number, value: T): _Array_ <T>
Copying version of using the bracket notation to change the value of a given index. It returns a new Array with theelement at the given index replaced with the given value.

**Returns:** a new Array with the element at the given index replaced with the given value
**Arguments:**

- index:numberto replace
- value: T new value
public staticfind<T>( fn: (elem: T) =>boolean, thisArg: _Array_ <T> ): T |null

**2.1. Array<T> 19**


Returns the first element in the provided array that satisfies the provided testing function
**Returns:** found element or null otherwise
**Arguments:**

- fn: (elem: T) =>booleantesting function
- thisArg: _Array_ <T> an Array to search

public staticfindIndex<T>( fn: (elem: T) =>boolean, thisArg: _Array_ <T> ):number
Returns the index of the first element in an array that satisfies the provided testing function
**Returns:** found element index or -1 otherwise
**Arguments:**

- fn: (elem: T) =>booleantesting function
- thisArg: _Array_ <T> an Array to search

public staticfindLast<T>( fn: (elem: T) =>boolean, thisArg: _Array_ <T> ): T |null
Iterates the array in reverse order and returns the value of the first element that satisfies the provided testing function
**Returns:** found element or null otherwise
**Arguments:**

- fn: (elem: T) =>booleantesting function
- thisArg: _Array_ <T> an Array to search

public staticfrom<T>(arr: T[]): _Array_ <T>
Creates a new`Array`instance from`object[]`primitive array.
**Returns:** `Array`intance constructed from`object[]`primitive array.
**Arguments:**

- arr: T[] primitive ‘object’ array to be converted to`Array`instance.

**20 Chapter 2. Classes**


public staticfrom<T, U>( arr: T[], fn: (v: T, k:number) => U ): _Array_ <U>
Creates a new`Array`instance from`object[]`primitive array.
**Returns:** `Array`intance constructed from`object[]`primitive array and given function.
**Arguments:**

- arr: T[] primitive ‘object’ array, converted to`Array`instance.
- fn: (v: T, k:array is first passed through this function, andnumber) => U map function to call on every element of the array. Every value to be added to the`fn`’s return value is added to the array instead.

public staticfrom<U>( str: _string_ , fn: (v: _string_ , k:number) => U ): _Array_ <U>
Creates a new`Array`instance from characters of`string`and mapping function.
**Returns:** `Array`intance constructed from characters of source`string`and given function.
**Arguments:**

- str: _string_ source string to be converted to array of character’s`string`
- fn: (v: _string_ , k:number) => U map function to call on every character of source string.

public staticfrom<U>( str: _string_ , fn: (v: _string_ , k:number) => U ): _Array_ <U>
Creates a new`Array`instance from characters of`string`and mapping function.
**Returns:** `Array`intance constructed from characters of source`string`and given function.
**Arguments:**

- str: _string_ source string to be converted to array of character’s`string`
- fn: (v: _string_ , k:number) => U map function to call on every character of source string.

public staticfrom(str: _string_ ): _Array_ < _string_ >
Creates a new`Array`instance from characters of`string`.
**Returns:** `Array`intance constructed from characters of source`string`.
**2.1. Array<T> 21**


**Arguments:**

- str: _string_ source string to be converted to array of character’s`string`

public staticfromAsync<T, U>( arrLike: T[], mapFn: (a: T, i:number) => U ): _Array_ <U>
Creates a new Array from array-like or iterable
**Returns:** a new instance of an Array
**Arguments:**

- arrLike: T[] array-like or an iterable object
- mapFn: (a: T, i:number) => U a function to apply over all elements of arrLike

public staticfromAsync<T>(arrLike: T[]): _Array_ <T>
Creates a new Array from array-like or iterable
**Returns:** new instance of an Array
**Arguments:**

- arrLike: T[] array-like or an iterable object

public staticisArray<T>(arr: T[]):boolean
Checks whether the passed value is an Array.
**Returns:** true is arr is a non-null array, false otherwise
**Arguments:**

- arr: T[]

public staticisArray<T>(arr: _Array_ <T>):boolean
Checks whether the passed value is an Array.

**22 Chapter 2. Classes**


**Returns:** true is arr is a non-null and non-empty array, false otherwise
**Arguments:**

- arr: _Array_ <T>

**2.1.2 Properties**

- length:number
**2.2 ArrayBuffer**
export
**Class:** JS ArrayBuffer API-compatible class

**2.2.1 Methods**
publicresize(newLen:number):void
Resizes the ArrayBuffer
**Arguments:**

- newLen:numbernew length

publicslice(begin:number, end:number): _ArrayBuffer_
Creates a new ArrayBuffer with copy of bytes in range [begin;end)
**Returns:** data taken from current ArrayBuffer with respect to begin and end parameters
**Arguments:**

- begin:numberan inclusive index to start copying with
- end:numbera last exclusive index to stop copying

public staticisView(obj: _object_ ):boolean
Checks if the passed object is a View
**2.2. ArrayBuffer 23**


**Returns:** true if obj is instance of typed array
**Arguments:**

- obj: _object_ to check

**2.2.2 Properties**

- byteLength:number
**2.3 Atomics**
export
**Class:** Represents JS API-compatible Atomics

**2.3.1 Methods**
public staticadd( typedArray: _Int8Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int8Array_
- index:number
- value:number
public staticadd( typedArray: _Uint8Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint8Array_
- index:number
**24 Chapter 2. Classes**


- value:number
public staticadd( typedArray: _Int16Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Int16Array_
- index:number
- value:number
public staticadd( typedArray: _Uint16Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint16Array_
- index:number
- value:number

public staticadd( typedArray: _Int32Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int32Array_
- index:number
- value:number

public staticadd( typedArray: _Uint32Array_ , index:number, value:number):number

**2.3. Atomics 25**


Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint32Array_
- index:number
- value:number
public staticadd( typedArray: _BigInt64Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _BigInt64Array_
- index:number
- value:number
public staticadd( typedArray: _BigUint64Array_ , index:number, value:number):number
Adds a given [value] at a given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _BigUint64Array_
- index:number
- value:number
public staticand( typedArray: _Int8Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:
26 Chapter 2. Classes**


- typedArray: _Int8Array_
- index:number
- value:number
public staticand( typedArray: _Uint8Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint8Array_
- index:number
- value:number
public staticand( typedArray: _Int16Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Int16Array_
- index:number
- value:number
public staticand( typedArray: _Uint16Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint16Array_
- index:number
- value:number

**2.3. Atomics 27**


public staticand( typedArray: _Int32Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int32Array_
- index:number
- value:number
public staticand( typedArray: _Uint32Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint32Array_
- • index:value:numbernumber

public staticand( typedArray: _BigInt64Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigInt64Array_
- index:number
- value:number
public staticand( typedArray: _BigUint64Array_ , index:number, value:number):number
Computes a bitwise AND of the given [value] and the value at the given [index] in the [typedArray].
**28 Chapter 2. Classes**


**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigUint64Array_
- index:number
- value:number
public staticValue:number):compareExchange( typedArray:number _Int8Array_ , index: number, expectedValue:number, replacement-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int8Array_
- index:number
- expectedValue:number
- replacementValue:number

public staticValue:number):compareExchange( typedArray:number _Uint8Array_ , index:number, expectedValue:number, replacement-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint8Array_
- index:number
- expectedValue:number
- replacementValue:number

public staticValue:number):compareExchange( typedArray:number _Int16Array_ , index:number, expectedValue:number, replacement-

**2.3. Atomics 29**


Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int16Array_
- index:number
- expectedValue:number
- replacementValue:number

public staticValue:number):compareExchange( typedArray:number _Uint16Array_ , index:number, expectedValue:number, replacement-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint16Array_
- index:number
- expectedValue:number
- replacementValue:number

public staticValue:number):compareExchange( typedArray:number _Int32Array_ , index:number, expectedValue:number, replacement-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int32Array_
- index:number
- expectedValue:number
- replacementValue:number

**30 Chapter 2. Classes**


public staticValue:number):compareExchange( typedArray:number _Uint32Array_ , index:number, expectedValue:number, replacement-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint32Array_
- index:number
- expectedValue:number
- replacementValue:number
public staticmentValue:numbercompareExchange( typedArray:):number _BigInt64Array_ , index:number, expectedValue:number, replace-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigInt64Array_
- index:number
- expectedValue:number
- replacementValue:number
public staticmentValue:numbercompareExchange( typedArray:):number _BigUint64Array_ , index:number, expectedValue:number, replace-

Exchanges a given [replacementValue] at a given [index] in the [typedArray], if a given [expectedValue] equals theold value. Returns the old value at that position whether it was equal to the expected value or not.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigUint64Array_
**2.3. Atomics 31**


- index:number
- expectedValue:number
- replacementValue:number

public staticexchange( typedArray: _Int8Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int8Array_
- index:number
- value:number
public staticexchange( typedArray: _Uint8Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint8Array_
- index:number
- value:number
public staticexchange( typedArray: _Int16Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Int16Array_
- index:number
- value:number

**32 Chapter 2. Classes**


public staticexchange( typedArray: _Uint16Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint16Array_
- index:number
- value:number
public staticexchange( typedArray: _Int32Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Int32Array_
- • index:value:numbernumber

public staticexchange( typedArray: _Uint32Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint32Array_
- index:number
- value:number
public staticexchange( typedArray: _BigInt64Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**2.3. Atomics 33**


**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigInt64Array_
- index:number
- value:number
public staticexchange( typedArray: _BigUint64Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _BigUint64Array_
- index:number
- value:number
public staticisLockFree(size:number):boolean
isLockFree(n) checks whether atomic operations for typed arrays of the given element size use hardware atomicsinstructions instead of locks.

**Returns:** a boolean result
**Arguments:**

- size:number

public staticload(typedArray: _Int8Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**

- typedArray: _Int8Array_

**34 Chapter 2. Classes**


- index:number
public staticload(typedArray: _Uint8Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**
- typedArray: _Uint8Array_
- index:number
public staticload(typedArray: _Int16Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**
- typedArray: _Int16Array_
- index:number
public staticload(typedArray: _Uint16Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**
- typedArray: _Uint16Array_
- index:number
public staticload(typedArray: _Int32Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value

**2.3. Atomics 35**


**Arguments:**

- typedArray: _Int32Array_
- index:number
public staticload(typedArray: _Uint32Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**
- typedArray: _Uint32Array_
- index:number
public staticload(typedArray: _BigInt64Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**
- typedArray: _BigInt64Array_
- index:number

public staticload(typedArray: _BigUint64Array_ , index:number):number
Returns a value at the given [index] in the [typedArray].
**Returns:** the read value
**Arguments:**

- typedArray: _BigUint64Array_
- index:number

public staticnotify(typedArray: _Int32Array_ , offset:number):number

**36 Chapter 2. Classes**


Notifies (wakes up) threads that are suspended by the Atomics.wait() calls at the given index. (index =typedArray.byteOffset + offset * 4)
Note: This method also wakes up threads suspended by the BigInt64Array Atomics.wait(t64, offset64) calls. But ifand only if ‘t64’ views the same ArrayBuffer as ‘typedArray’ and ‘offset64’ and ‘offset’ point at the same index in
that ArrayBuffer.
**Returns:** the number of notified threads
**Arguments:**

- typedArray: _Int32Array_
- offset:number

public staticnotify( typedArray: _Int32Array_ , offset:number, count:number):number
Operates exactly like Atomics.notify(Int32Array, int) but specifies the maximum number of threads to notify using‘count’.

**Returns:** the number of notified threads
**Arguments:**

- • typedArray:offset:number _Int32Array_
- count:number

public staticnotify(typedArray: _BigInt64Array_ , offset:number):number
Notifies (wakes up) threads that are suspended by the Atomics.wait() calls at the given index. (index =typedArray.byteOffset + offset * 8)
Note: This method also wakes up threads suspended by the Int32Array Atomics.wait(t32, offset32) calls. But if andonly if ‘t32’ views the same ArrayBuffer as ‘typedArray’ and ‘offset32’ and ‘offset’ point at the same index in that
ArrayBuffer.
**Returns:** the number of notified threads
**Arguments:**

- typedArray: _BigInt64Array_
- offset:number

public staticnotify( typedArray: _BigInt64Array_ , offset:number, count:number):number

**2.3. Atomics 37**


Operates exactly like Atomics.notify(BigInt64Array, int) but specifies the maximum number of threads to notifyusing ‘count’.

**Returns:** the number of notified threads
**Arguments:**

- typedArray: _BigInt64Array_
- offset:number
- count:number
public staticor( typedArray: _Int8Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int8Array_
- index:number
- value:number
public staticor( typedArray: _Uint8Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint8Array_
- index:number
- value:number
public staticor( typedArray: _Int16Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**38 Chapter 2. Classes**


**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int16Array_
- index:number
- value:number
public staticor( typedArray: _Uint16Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint16Array_
- index:number
- value:number
public staticor( typedArray: _Int32Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int32Array_
- index:number
- value:number
public staticor( typedArray: _Uint32Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**2.3. Atomics 39**


**Arguments:**

- typedArray: _Uint32Array_
- • index:value:numbernumber

public staticor( typedArray: _BigInt64Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigInt64Array_
- • index:value:numbernumber

public staticor( typedArray: _BigUint64Array_ , index:number, value:number):number
Computes a bitwise OR of the given [value] and the value at the given [index] in the [typedArray]. Updates the valuein the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigUint64Array_
- index:number
- value:number
public staticstore( typedArray: _Int8Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**

**40 Chapter 2. Classes**


- typedArray: _Int8Array_
- index:number
- value:number
public staticstore( typedArray: _Uint8Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**
- typedArray: _Uint8Array_
- index:number
- value:number
public staticstore( typedArray: _Int16Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**
- typedArray: _Int16Array_
- index:number
- value:number
public staticstore( typedArray: _Uint16Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**
- typedArray: _Uint16Array_
- index:number
- value:number

**2.3. Atomics 41**


public staticstore( typedArray: _Int32Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**

- typedArray: _Int32Array_
- index:number
- value:number
public staticstore( typedArray: _Uint32Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**
- typedArray: _Uint32Array_
- • index:value:numbernumber

public staticstore( typedArray: _BigInt64Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**

- typedArray: _BigInt64Array_
- index:number
- value:number
public staticstore( typedArray: _BigUint64Array_ , index:number, value:number):number
Stores a given [value] at a given [index] in the [typedArray] and returns that value.
**42 Chapter 2. Classes**


**Returns:** the new value (i.e. the [value] parameter)
**Arguments:**

- typedArray: _BigUint64Array_
- index:number
- value:number
public staticsub( typedArray: _Int8Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Int8Array_
- index:number
- value:number
public staticsub( typedArray: _Uint8Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint8Array_
- index:number
- value:number

public staticsub( typedArray: _Int16Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**

**2.3. Atomics 43**


- typedArray: _Int16Array_
- index:number
- value:number
public staticsub( typedArray: _Uint16Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint16Array_
- index:number
- value:number
public staticsub( typedArray: _Int32Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Int32Array_
- index:number
- value:number
public staticsub( typedArray: _Uint32Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _Uint32Array_
- index:number
- value:number

**44 Chapter 2. Classes**


public staticsub( typedArray: _BigInt64Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigInt64Array_
- index:number
- value:number
public staticsub( typedArray: _BigUint64Array_ , index:number, value:number):number
Subtracts a given [value] at a given [index] in the array and returns the old value at that position.
**Returns:** the old value at that position
**Arguments:**
- typedArray: _BigUint64Array_
- • index:value:numbernumber

public staticwait( typedArray: _Int32Array_ , offset:number, value:number): _string_
Suspends the current thread if "typedArray[offset] != value" until it is notified by Atomics.notify. Note: AnAtomics.notify call will wake up this thread even if "typedArray[offset] == value" is true. In other words, the
"typedArray[offset] != value" condition is checked only once.
**Returns:** notified returns "ok""not-equal" if the the value the the given [offset] was not equal to the given [value], otherwise after being

**Arguments:**

- typedArray: _Int32Array_
- offset:number
- value:number
public staticwait( typedArray: _Int32Array_ , offset:number, value:number, timeout:number): _string_
**2.3. Atomics 45**


Operates exactly like Atomics.wait(Int32Array, int, int) but also returns if the given [timeout] (in ms.) passes.
**Returns:** "not-equal" and "ok" like Atomics.wait(Int32Array, int, int), but also "timed-out" if the timeout passes
**Arguments:**

- typedArray: _Int32Array_
- offset:number
- value:number
- timeout:number
public staticwait( typedArray: _BigInt64Array_ , offset:number, value:number): _string_
Suspends the current thread if "typedArray[offset] != value" until it is notified by Atomics.notify. Note 1: AnAtomics.notify call will wake up this thread even if "typedArray[offset] == value" is true. In other words, the
"typedArray[offset] != value" condition is checked only once. Note 2: A call to Atomic.notify(Int32Array, int) willwake up this thread, but only if both offsets point at the same index in the underlying ArrayBuffer. In the other words,
a notification issued to the right 32-bit half of the 64-bit integer will not wake up this thread.
**Returns:** notified returns "ok""not-equal" if the the value the the given [offset] was not equal to the given [value], otherwise after being

**Arguments:**

- typedArray: _BigInt64Array_
- offset:number
- value:number
public staticwait( typedArray: _BigInt64Array_ , offset:number, value:number, timeout:number): _string_
Operates exactly like Atomics.wait(BigInt64Array, int, long) but also returns if the given [timeout] (in ms.) passes.
**Returns:** "not-equal" and "ok" like Atomics.wait(Int32Array, int, int), but also "timed-out" if the timeout passes
**Arguments:**
- typedArray: _BigInt64Array_
- offset:number
- value:number
- timeout:number
public staticxor( typedArray: _Int8Array_ , index:number, value:number):number
**46 Chapter 2. Classes**


Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int8Array_
- index:number
- value:number
public staticxor( typedArray: _Uint8Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint8Array_
- index:number
- value:number
public staticxor( typedArray: _Int16Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int16Array_
- index:number
- value:number
public staticxor( typedArray: _Uint16Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**2.3. Atomics 47**


**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint16Array_
- index:number
- value:number
public staticxor( typedArray: _Int32Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Int32Array_
- index:number
- value:number
public staticxor( typedArray: _Uint32Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _Uint32Array_
- index:number
- value:number
public staticxor( typedArray: _BigInt64Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**48 Chapter 2. Classes**


**Arguments:**

- typedArray: _BigInt64Array_
- • index:value:numbernumber

public staticxor( typedArray: _BigUint64Array_ , index:number, value:number):number
Computes a bitwise XOR of the given [value] and the value at the given [index] in the [typedArray]. Updates thevalue in the array and returns the old value at that position.

**Returns:** the old value at that position
**Arguments:**

- typedArray: _BigUint64Array_
- • index:value:numbernumber

**2.4 BigInt**
export
BigInt class stub

**2.4.1 Methods**
publicconstructor():void
Constructs new BigInt

publicconstructor(d:number):void
Constructs new BigInt from int
**Arguments:
2.4. BigInt 49**


- d:numberinitializer
publicconstructor(d: _string_ ):void
Constructs new BigInt from string
**Arguments:**
- d: _string_ initializer

publicconstructor(d:boolean):void
Constructs new BigInt from string
**Arguments:**

- d:booleaninitializer

publicconstructor(d: _BigInt_ ):void
Constructs new BigInt from string
**Arguments:**

- d: _BigInt_ initializer

publicconstructor(d:number):void
Constructs new BigInt from string
**Arguments:**

- d:number
publicconstructor(d:number):void
Constructs new BigInt from number
**Arguments:**

**50 Chapter 2. Classes**


- d:number
publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Returns a locale string representing the specified array and its elements.
**Returns:** string representation
**Arguments:**
- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Returns a locale string representing the specified array and its elements.
**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Returns a locale string representing the specified array and its elements.
**Returns:** string representation

public overridetoString(): _string_
Returns string representation
**Returns:** string representation

publicvalueOf(): _BigInt_
**2.4. BigInt 51**


Returns a BigInt instance
**Returns:** a BigInt instance

**2.5 BigInt64Array**
export
JS BigInt64Array API-compatible class

**2.5.1 Methods**
publicat(index:number): _BigInt_
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at

publicconstructor():void
Creates an empty BigInt64Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an BigInt64Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type long in newly created BigInt64Array
**52 Chapter 2. Classes**


publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an BigInt64Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an BigInt64Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _BigInt64Array_ ):void
Creates a copy of BigInt64Array.
**Arguments:**

- other: _BigInt64Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of BigInt64Array.
**2.5. BigInt64Array 53**


**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of BigInt64Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element: _BigInt_ ) =>boolean):boolean
Checks that all elements of BigInt64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element: _BigInt_ ) =>booleancheck function

publicevery( fn: (element: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean):boolean
Checks that all elements of BigInt64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleancheck function

publicevery( fn: (element: _BigInt_ , index:number) =>boolean):boolean
Checks that all elements of BigInt64Array satisfy the passed function
**Returns:** true if all elements satisfy fn

**54 Chapter 2. Classes**


**Arguments:**

- fn: (element: _BigInt_ , index:number) =>booleancheck function

publicfill( value: _BigInt_ , start:number, end:number): _BigInt64Array_
Fills the BigInt64Array with specified value
**Returns:** modified BigInt64Array
**Arguments:**

- value: _BigInt_ new valuy
- start:number
- end:number
publicfill(value: _BigInt_ , start:number): _BigInt64Array_
Fills the BigInt64Array with specified value
**Returns:** modified BigInt64Array
**Arguments:**
- value: _BigInt_ new valuy
- start:number
publicfill(value:number): _BigInt64Array_
Fills the BigInt64Array with specified value
**Returns:** modified BigInt64Array
**Arguments:**
- value:numbernew valuy

publicfilter( fn: (val: _BigInt_ ) =>boolean): _BigInt64Array_
creates a new BigInt64Array from current BigInt64Array based on a condition fn
**2.5. BigInt64Array 55**


**Returns:** a new BigInt64Array with elements from current BigInt64Array that satisfy condition fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean): _BigInt64Array_
Creates a new BigInt64Array from current BigInt64Array based on a condition fn.
**Returns:** a new BigInt64Array with elements from current BigInt64Array that satisfy condition fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val: _BigInt_ , index:number) =>boolean): _BigInt64Array_
creates a new BigInt64Array from current BigInt64Array based on a condition fn
**Returns:** a new BigInt64Array with elements from current BigInt64Array that satisfy condition fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val: _BigInt_ ) =>boolean):number
Finds the first element in the BigInt64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfind( fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean): _BigInt_
Finds the first element in the BigInt64Array that satisfies the condition

**56 Chapter 2. Classes**


**Returns:** the first element that satisfies fn TODO: return long | undefined as in JS
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds the first element in the BigInt64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ ) =>boolean):number
Finds an index of the first element in the BigInt64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean):number
Finds an index of the first element in the BigInt64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds an index of the first element in the BigInt64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**2.5. BigInt64Array 57**


**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindLast( fn: (val: _BigInt_ ) =>boolean): _BigInt_
Finds the last element in the BigInt64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfindLast( fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean):number
Finds the last element in the BigInt64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleancondition

publicfindLast( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds the last element in the BigInt64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ ) =>boolean):number
Finds an index of the last element in the BigInt64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise

**58 Chapter 2. Classes**


**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean):number
Finds an index of the last element in the BigInt64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds an index of the last element in the BigInt64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicforEach( fn: (val: _BigInt_ ) =>number):void
Applies a function over all elements of BigInt64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ ) =>numberfunction to apply

publicforEach( fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>number):void
Applies a function over all elements of BigInt64Array
**Returns:** undefined
**Arguments:
2.5. BigInt64Array 59**


- fn: (val: _BigInt_ , index:number, array: _BigInt64Array_ ) =>numberfunction to apply

publicforEach( fn: (val: _BigInt_ , index:number) =>number):void
Applies a function over all elements of BigInt64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>numberfunction to apply

publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _BigInt64Array_
Creates an BigInt64Array from array-like argument
**Returns:** new BigInt64Array
**Arguments:**

- o: _object_ array-like object to initialize BigInt64Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each

publicfrom(o: _object_ ): _BigInt64Array_
Creates an BigInt64Array from array-like argument
**Returns:** new BigInt64Array
**Arguments:**

- o: _object_ array-like object to initialize BigInt64Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _BigInt64Array_
Creates an BigInt64Array from array-like argument
**Returns:** new BigInt64Array
**Arguments:**

**60 Chapter 2. Classes**


- o: _object_ array-like object to initialize BigInt64Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e: _BigInt_ , fromIndex:number):boolean
Checks if specified argument is in BigInt64Array
**Returns:** true if e is in BigInt64Array, false otherwise
**Arguments:**

- e: _BigInt_ search element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in BigInt64Array
**Returns:** true if e is in BigInt64Array, false otherwise
**Arguments:**

- e:numbersearch element

publicindexOf(e: _BigInt_ , fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**

- e: _BigInt_ search element
- fromIndex:numberstart index to search from

publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**2.5. BigInt64Array 61**


**Arguments:**

- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**
- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publickeys(): _IterableIterator_ <number>
Returns keys of the BigInt64Array
**Returns:** iterator over keys

publiclastIndexOf(val: _BigInt_ , fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val: _BigInt_ a value to search
**62 Chapter 2. Classes**


- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val: _BigInt_ ):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val: _BigInt_ a value to search

publicmap( fn: (val: _BigInt_ ) => _BigInt_ ): _BigInt64Array_
Creates a new BigInt64Array using fn(arr[i]) over all elements of current BigInt64Array
**Returns:** a new BigInt64Array where for each element from current BigInt64Array fn was applied
**Arguments:**

- fn: (val: _BigInt_ ) => _BigInt_ a function to apply for each element of current BigInt64Array

publicmap( fn: (val: _BigInt_ , index:number) => _BigInt_ ): _BigInt64Array_
Creates a new BigInt64Array using fn(arr[i]) over all elements of current BigInt64Array.
**Returns:** a new BigInt64Array where for each element from current BigInt64Array fn was applied
**Arguments:**

- fn: (val: _BigInt_ , index:number) => _BigInt_ a function to apply for each element of current BigInt64Array

publicof(data:number[]): _BigInt64Array_
Creates a new BigInt64Array using initializer
**Returns:** a new BigInt64Array from data
**Arguments:**

- data:number[] initializer
**2.5. BigInt64Array 63**


public _BigInt_ reduce( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ , init: _BigInt_ ):

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ condition
- init: _BigInt_ initial value

publicreduce( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ ): _BigInt_
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ condition

public): _BigInt_ reduceRight( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ , init: _BigInt_

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ condition
- init: _BigInt_ initial value

publicreduceRight( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ ): _BigInt_
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result

**64 Chapter 2. Classes**


**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigInt64Array_ ) => _BigInt_ condition

publicreverse(): _BigInt64Array_
Creates a new BigInt64Array using reversed data from the current one
**Returns:** a new BigInt64Array using reversed data from the current one

publicset(insertPos:number, val: _BigInt_ ):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(long[], insertPos)
**Arguments:**

- • insertPos:val: _BigInt_ numbervalue to setindex to change

publicset(arr: _BigInt_ [], insertPos1:number):void
Copies all elements of arr to the current BigInt64Array starting from insertPos.
**Arguments:**

- arr: _BigInt_ [] array to copy data from
- insertPos1:number
publicset(arr: _BigInt_ []):void
Copies all elements of arr to the current BigInt64Array.
**Arguments:**
- arr: _BigInt_ [] array to copy data from

**2.5. BigInt64Array 65**


publicslice(begin:number, end:number): _BigInt64Array_
Creates a slice of current BigInt64Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new BigInt64Array with elements of current BigInt64Array[begin;end) where end index is excluded

**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _BigInt64Array_
Creates a slice of current BigInt64Array using range [begin, this.length).
**Returns:** a new BigInt64Array with elements of current BigInt64Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _BigInt64Array_
Creates a slice of current BigInt64 with all elements.
**Returns:** a new BigInt64Array with elements of current BigInt64Array

publicsome( fn: (element: _BigInt_ ) =>boolean):boolean
Checks that at least one element of BigInt64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ ) =>booleancheck function

**66 Chapter 2. Classes**


publicsome( fn: (element: _BigInt_ , index:number, array: _BigInt64Array_ ) =>boolean):boolean
Checks that at least one element of BigInt64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ , index:number, array: _BigInt64Array_ ) =>booleancheck function

publicsome( fn: (element: _BigInt_ , index:number) =>boolean):boolean
Checks that at least one element of BigInt64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ , index:number) =>booleancheck function

publicsort(): _BigInt64Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted BigInt64Array

publicsort( fn: (a: _BigInt_ , b: _BigInt_ ) =>number): _BigInt64Array_
Sorts in-place
**Returns:** sorted BigInt64Array
**Arguments:**

- fn: (a: _BigInt_ , b: _BigInt_ ) =>numbercomparator

publicsubarray(begin:number, end:number): _BigInt64Array_
**2.5. BigInt64Array 67**


Creates a BigInt64Array with the same underlying ArrayBuffer
**Returns:** new BigInt64Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive
publicsubarray(begin:number): _BigInt64Array_
Creates a BigInt64Array with the same ArrayBuffer
**Returns:** new BigInt64Array with the same ArrayBuffer
**Arguments:**
- begin:numberstart index, inclusive
publicsubarray(): _BigInt64Array_
Creates a BigInt64Array with the same ArrayBuffer
**Returns:** new BigInt64Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts BigInt64Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_
publictoLocaleString(locales: _object_ ): _string_
Converts BigInt64Array to a string with respect to locale
**68 Chapter 2. Classes**


**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts BigInt64Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _BigInt64Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _BigInt64Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the BigInt64Array
**Returns:** a string representation of the BigInt64Array

publicvalues(): _IterableIterator_ <number>
**2.5. BigInt64Array 69**


Returns array values iterator
**Returns:** an iterator

publicwith(index:number, value:number): _BigInt64Array_
Creates a copy with replaced value on index
**Returns:** an BigInt64Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.5.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- • byteLength:byteOffset:numbernumber
- length:number
**2.6 BigUint64Array**
export
JS BigUint64Array API-compatible class

**2.6.1 Methods**
publicat(index:number): _BigInt_
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

**70 Chapter 2. Classes**


- index:numberindex to look at
publicconstructor():void
Creates an empty BigUint64Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an BigUint64Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type BigInt in newly created BigUint64Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an BigUint64Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an BigUint64Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _BigUint64Array_ ):void
Creates a copy of BigUint64Array.

**2.6. BigUint64Array 71**


**Arguments:**

- other: _BigUint64Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of BigUint64Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of BigUint64Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):boolean
Checks that all elements of BigUint64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

**72 Chapter 2. Classes**


- fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancheck function

publicevery( fn: (element: _BigInt_ , index:number) =>boolean):boolean
Checks that all elements of BigUint64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element: _BigInt_ , index:number) =>booleancheck function

publicevery( fn: (element: _BigInt_ ) =>boolean):boolean
Checks that all elements of BigUint64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element: _BigInt_ ) =>booleancheck function

publicevery( fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):boolean
Checks that all elements of BigUint64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancheck function

publicevery( fn: (element: _BigInt_ , index:number) =>boolean):boolean
Checks that all elements of BigUint64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element: _BigInt_ , index:number) =>booleancheck function
**2.6. BigUint64Array 73**


publicfill( value: _BigInt_ , start:number, end:number): _BigUint64Array_
Fills the BigUint64Array with specified value
**Returns:** modified BigUint64Array
**Arguments:**

- value: _BigInt_ new valuy
- start:number
- end:number
publicfill(value: _BigInt_ , start:number): _BigUint64Array_
Fills the BigUint64Array with specified value
**Returns:** modified BigUint64Array
**Arguments:**
- value: _BigInt_ new valuy
- start:number
publicfill(value: _BigInt_ ): _BigUint64Array_
Fills the BigUint64Array with specified value
**Returns:** modified BigUint64Array
**Arguments:**
- value: _BigInt_ new valuy

publicfilter( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean): _BigUint64Array_
Creates a new BigUint64Array from current BigUint64Array based on a condition fn.
**Returns:** a new BigUint64Array with elements from current BigUint64Array that satisfy condition fn

**74 Chapter 2. Classes**


**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val: _BigInt_ , index:number) =>boolean): _BigUint64Array_
creates a new BigUint64Array from current BigUint64Array based on a condition fn
**Returns:** a new BigUint64Array with elements from current BigUint64Array that satisfy condition fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val: _BigInt_ ) =>boolean): _BigUint64Array_
creates a new BigUint64Array from current BigUint64Array based on a condition fn
**Returns:** a new BigUint64Array with elements from current BigUint64Array that satisfy condition fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean): _BigUint64Array_
Creates a new BigUint64Array from current BigUint64Array based on a condition fn.
**Returns:** a new BigUint64Array with elements from current BigUint64Array that satisfy condition fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val: _BigInt_ , index:number) =>boolean): _BigUint64Array_
creates a new BigUint64Array from current BigUint64Array based on a condition fn
**Returns:** a new BigUint64Array with elements from current BigUint64Array that satisfy condition fn
**Arguments:
2.6. BigUint64Array 75**


- fn: (val: _BigInt_ , index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean): _BigInt_
Finds the first element in the BigUint64Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return BigInt | undefined as in JS
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val: _BigInt_ , index:number) =>boolean): _BigInt_
Finds the first element in the BigUint64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfind( fn: (val: _BigInt_ ) =>boolean): _BigInt_
Finds the first element in the BigUint64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfind( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean): _BigInt_
Finds the first element in the BigUint64Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return BigInt | undefined as in JS
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleanthe condition to apply for each element
**76 Chapter 2. Classes**


publicfind( fn: (val: _BigInt_ , index:number) =>boolean): _BigInt_
Finds the first element in the BigUint64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):number
Finds an index of the first element in the BigUint64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds an index of the first element in the BigUint64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ ) =>boolean):number
Finds an index of the first element in the BigUint64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

**2.6. BigUint64Array 77**


publicfindIndex( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):number
Finds an index of the first element in the BigUint64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancondition

publicfindIndex( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds an index of the first element in the BigUint64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindLast( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean): _BigInt_
Finds the last element in the BigUint64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancondition

publicfindLast( fn: (val: _BigInt_ , index:number) =>boolean): _BigInt_
Finds the last element in the BigUint64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

**78 Chapter 2. Classes**


publicfindLast( fn: (val: _BigInt_ ) =>boolean): _BigInt_
Finds the last element in the BigUint64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfindLast( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean): _BigInt_
Finds the last element in the BigUint64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancondition

publicfindLast( fn: (val: _BigInt_ , index:number) =>boolean): _BigInt_
Finds the last element in the BigUint64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):number
Finds an index of the last element in the BigUint64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancondition

**2.6. BigUint64Array 79**


publicfindLastIndex( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds an index of the last element in the BigUint64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ ) =>boolean):number
Finds an index of the last element in the BigUint64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ ) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):number
Finds an index of the last element in the BigUint64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancondition

publicfindLastIndex( fn: (val: _BigInt_ , index:number) =>boolean):number
Finds an index of the last element in the BigUint64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val: _BigInt_ , index:number) =>booleancondition

**80 Chapter 2. Classes**


publicforEach( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) => _BigInt_ ):void
Applies a function over all elements of BigUint64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) => _BigInt_ function to apply

publicforEach( fn: (val: _BigInt_ , index:number) => _BigInt_ ):void
Applies a function over all elements of BigUint64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ , index:number) => _BigInt_ function to apply

publicforEach( fn: (val: _BigInt_ ) => _BigInt_ ):void
Applies a function over all elements of BigUint64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ ) => _BigInt_ function to apply

publicforEach( fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) => _BigInt_ ):void
Applies a function over all elements of BigUint64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ , index:number, array: _BigUint64Array_ ) => _BigInt_ function to apply

**2.6. BigUint64Array 81**


publicforEach( fn: (val: _BigInt_ , index:number) => _BigInt_ ):void
Applies a function over all elements of BigUint64Array
**Returns:** undefined
**Arguments:**

- fn: (val: _BigInt_ , index:number) => _BigInt_ function to apply
publicfrom( o: _object_ , mapFn: (e: _object_ ) => _BigInt_ ): _BigUint64Array_
Creates an BigUint64Array from array-like argument
**Returns:** new BigUint64Array
**Arguments:**
- o: _object_ array-like object to initialize BigUint64Array
- mapFn: (e: _object_ ) => _BigInt_ function to apply for each

publicfrom(o: _object_ ): _BigUint64Array_
Creates an BigUint64Array from array-like argument
**Returns:** new BigUint64Array
**Arguments:**

- o: _object_ array-like object to initialize BigUint64Array
publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) => _BigInt_ ): _BigUint64Array_
Creates an BigUint64Array from array-like argument
**Returns:** new BigUint64Array
**Arguments:**
- o: _object_ array-like object to initialize BigUint64Array
**82 Chapter 2. Classes**


- mapFn: (e: _object_ , index:number) => _BigInt_ function to apply for each

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) => _BigInt_ ): _BigUint64Array_
Creates an BigUint64Array from array-like argument
**Returns:** new BigUint64Array
**Arguments:**

- o: _object_ array-like object to initialize BigUint64Array
- mapFn: (e: _object_ , index:number) => _BigInt_ function to apply for each

publicincludes(e: _BigInt_ , fromIndex:number):boolean
Checks if specified argument is in BigUint64Array
**Returns:** true if e is in BigUint64Array, false otherwise
**Arguments:**

- e: _BigInt_ search element
- fromIndex:numberstart index to search from

publicincludes(e: _BigInt_ ):boolean
Checks if specified argument is in BigUint64Array
**Returns:** true if e is in BigUint64Array, false otherwise
**Arguments:**

- e: _BigInt_ search element

publicindexOf(e: _BigInt_ , fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise

**2.6. BigUint64Array 83**


**Arguments:**

- e: _BigInt_ search element
- fromIndex:numberstart index to search from
publicindexOf(e: _BigInt_ ):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e: _BigInt_ search element

publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**

- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publiclastIndexOf(val: _BigInt_ , fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:
84 Chapter 2. Classes**


- val: _BigInt_ a value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val: _BigInt_ ):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val: _BigInt_ a value to search

publicmap( fn: (val: _BigInt_ , index:number) => _BigInt_ ): _BigUint64Array_
Creates a new BigUint64Array using fn(arr[i]) over all elements of current BigUint64Array.
**Returns:** a new BigUint64Array where for each element from current BigUint64Array fn was applied
**Arguments:**

- fn: (val: _BigInt_ , index:number) => _BigInt_ a function to apply for each element of current BigUint64Array

publicmap( fn: (val: _BigInt_ ) => _BigInt_ ): _BigUint64Array_
Creates a new BigUint64Array using fn(arr[i]) over all elements of current BigUint64Array
**Returns:** a new BigUint64Array where for each element from current BigUint64Array fn was applied
**Arguments:**

- fn: (val: _BigInt_ ) => _BigInt_ a function to apply for each element of current BigUint64Array

publicmap( fn: (val: _BigInt_ , index:number) => _BigInt_ ): _BigUint64Array_
Creates a new BigUint64Array using fn(arr[i]) over all elements of current BigUint64Array.
**Returns:** a new BigUint64Array where for each element from current BigUint64Array fn was applied
**Arguments:**

**2.6. BigUint64Array 85**


- fn: (val: _BigInt_ , index:number) => _BigInt_ a function to apply for each element of current BigUint64Array

publicof(data:bigint[]): _BigUint64Array_
Creates a new BigUint64Array using initializer
**Returns:** a new BigUint64Array from data
**Arguments:**

- data:bigint[] initializer

public _BigInt_ reduce( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ , init: _BigInt_ ):

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition
- init: _BigInt_ initial value

publicreduce( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ ): _BigInt_
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition

public _BigInt_ reduce( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ , init: _BigInt_ ):

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result

**86 Chapter 2. Classes**


**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition
- init: _BigInt_ initial value
publicreduce( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ ): _BigInt_
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition

public _BigInt_ ):reduceRight( fn: (acc: _BigInt BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ , init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition
- init: _BigInt_ initial value

publicreduceRight( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ ): _BigInt_
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition
public _BigInt_ ):reduceRight( fn: (acc: _BigInt BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ , init:

Reduces data into a single value using right-to-left traversal
**2.6. BigUint64Array 87**


**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition
- init: _BigInt_ initial value

publicreduceRight( fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ ): _BigInt_
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc: _BigInt_ , curVal: _BigInt_ , curIndex:number, array: _BigUint64Array_ ) => _BigInt_ condition

publicreverse(): _BigUint64Array_
Creates a new BigUint64Array using reversed data from the current one
**Returns:** a new BigUint64Array using reversed data from the current one

publicset(insertPos:number, val: _BigInt_ ):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(BigInt[], insertPos)
**Arguments:**

- insertPos:numberindex to change
- val: _BigInt_ value to set

publicset(arr: _BigInt_ [], insertPos1:number):void
Copies all elements of arr to the current BigUint64Array starting from insertPos.
**88 Chapter 2. Classes**


**Arguments:**

- arr: _BigInt_ [] array to copy data from
- insertPos1:number
publicset(arr: _BigInt_ []):void
Copies all elements of arr to the current BigUint64Array.
**Arguments:**
- arr: _BigInt_ [] array to copy data from

publicslice(begin:number, end:number): _BigUint64Array_
Creates a slice of current BigUint64Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new BigUint64Array with elements of current BigUint64Array[begin;end) where end index is excluded

**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _BigUint64Array_
Creates a slice of current BigUint64Array using range [begin, this.length).
**Returns:** a new BigUint64Array with elements of current BigUint64Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _BigUint64Array_
Creates a slice of current BigUint64 with all elements.
**Returns:** a new BigUint64Array with elements of current BigUint64Array
**2.6. BigUint64Array 89**


publicsome( fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):boolean
Checks that at least one element of BigUint64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancheck function

publicsome( fn: (element: _BigInt_ , index:number) =>boolean):boolean
Checks that at least one element of BigUint64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ , index:number) =>booleancheck function

publicsome( fn: (element: _BigInt_ ) =>boolean):boolean
Checks that at least one element of BigUint64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ ) =>booleancheck function

publicsome( fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>boolean):boolean
Checks that at least one element of BigUint64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

**90 Chapter 2. Classes**


- fn: (element: _BigInt_ , index:number, array: _BigUint64Array_ ) =>booleancheck function

publicsome( fn: (element: _BigInt_ , index:number) =>boolean):boolean
Checks that at least one element of BigUint64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element: _BigInt_ , index:number) =>booleancheck function

publicsort(): _BigUint64Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted BigUint64Array

publicsort( fn: (a: _BigInt_ , b: _BigInt_ ) =>number): _BigUint64Array_
Sorts in-place
**Returns:** sorted BigUint64Array
**Arguments:**

- fn: (a: _BigInt_ , b: _BigInt_ ) =>numbercomparator

publicsubarray(begin:number, end:number): _BigUint64Array_
Creates a BigUint64Array with the same underlying ArrayBuffer
**Returns:** new BigUint64Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive
**2.6. BigUint64Array 91**


publicsubarray(begin:number): _BigUint64Array_
Creates a BigUint64Array with the same ArrayBuffer
**Returns:** new BigUint64Array with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

publicsubarray(): _BigUint64Array_
Creates a BigUint64Array with the same ArrayBuffer
**Returns:** new BigUint64Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts BigUint64Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts BigUint64Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_

**92 Chapter 2. Classes**


publictoLocaleString(): _string_
Converts BigUint64Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _BigUint64Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _BigUint64Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the BigUint64Array
**Returns:** a string representation of the BigUint64Array

publicwith(index:number, value: _BigInt_ ): _BigUint64Array_
Creates a copy with replaced value on index
**Returns:** an BigUint64Array with replaced value on index

**2.6. BigUint64Array 93**


**Arguments:**

- index:number
- value: _BigInt_

**2.6.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
- length:number
**2.7 Boolean**
export extends _object_
Represents boxed boolean value and related operations

**2.7.1 Methods**
publicconstructor():void
Constructs a new Boolean with false value

publicconstructor(value:boolean):void
Constructs a new Boolean with provided value
**Arguments:**

- value:boolean— value to construct class instance with
publicconstructor(value: _Boolean_ ):void
Constructs a new Boolean with provided value

**94 Chapter 2. Classes**


**Arguments:**

- value: _Boolean_ — value to construct class instance with
public overridetoString(): _string_
Converts this object to a string
**Returns:** "True" if this instance is true, "False" otherwise

public staticvalueOf(b:boolean): _Boolean_
Static method that converts primitive boolean to boxed version
**Returns:** boxed value that represents provided primitive value
**Arguments:**

- b:boolean— value to be converted

**2.8 DataView**
export
DataView representation

**2.8.1 Methods**
publicconstructor(buffer: _ArrayBuffer_ ):void
Constructs view
**Arguments:**

- ArrayBuffer: _ArrayBuffer_ underlying ArrayBuffer

publicconstructor(buffer: _ArrayBuffer_ , byteOffset:number):void
**2.8. DataView 95**


Constructs view
**Arguments:**

- ArrayBuffer: _ArrayBuffer_ underlying ArrayBuffer
- byteOffset:numberoffset to start from
**Throws:**
- _RangeError_ if offset is out of array

publicconstructor( ArrayBuffer: _ArrayBuffer_ , byteOffset:number, byteLength:number):void
Constructs view
**Arguments:**

- ArrayBuffer: _ArrayBuffer_ underlying ArrayBuffer
- • byteOffset:byteLength:numbernumberoffset to start fromlenth of bytes to take

**Throws:**

- _RangeError_ if provided indicies are invalid

publicgetBigInt64(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetBigInt64(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**Returns:** read value
**96 Chapter 2. Classes**


**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetBigUint64(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetBigUint64(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**Returns:** read value
**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetFloat32(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetFloat32(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**2.8. DataView 97**


**Returns:** read value
**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetFloat64(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetFloat64(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**Returns:** read value
**Arguments:**

- • byteOffset:littleEndian:numberbooleanzero index to readread as little or big endian

publicgetInt16(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetInt16(byteOffset:number, littleEndian:boolean):number

**98 Chapter 2. Classes**


Read bytes as they represent given type
**Returns:** read value
**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetInt32(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetInt32(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**Returns:** read value
**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetInt8(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetUint16(byteOffset:number):number
**2.8. DataView 99**


Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetUint16(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**Returns:** read value
**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetUint32(byteOffset:number):number
Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicgetUint32(byteOffset:number, littleEndian:boolean):number
Read bytes as they represent given type
**Returns:** read value
**Arguments:**

- byteOffset:numberzero index to read
- littleEndian:booleanread as little or big endian

publicgetUint8(byteOffset:number):number
**100 Chapter 2. Classes**


Read bytes as they represent given type
**Returns:** read value (big endian)
**Arguments:**

- byteOffset:numberzero index to read

publicsetBigInt64(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
publicsetBigInt64( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:**
- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetBigUint64(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
publicsetBigUint64( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type

**2.8. DataView 101**


**Arguments:**

- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetFloat32(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number

publicsetFloat32( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetFloat64(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
publicsetFloat64( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:
102 Chapter 2. Classes**


- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetInt16(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
publicsetInt16( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:**
- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetInt32(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
publicsetInt32( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:**
- byteOffset:numberzero index to write
**2.8. DataView 103**


- value:number
- littleEndian:booleanread as little or big endian

publicsetInt8(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
publicsetUint16(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**
- byteOffset:numberzero index to write (big endian)
- value:number

publicsetUint16( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetUint32(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number
**104 Chapter 2. Classes**


publicsetUint32( byteOffset:number, value:number, littleEndian:boolean):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write
- value:number
- littleEndian:booleanread as little or big endian

publicsetUint8(byteOffset:number, value:number):void
Sets bytes as they represent given type
**Arguments:**

- byteOffset:numberzero index to write (big endian)
- value:number

**2.8.2 Properties**

- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
**2.9 Date**
export
Date JS API-compatible class

**2.9.1 Methods**
publicconstructor():void
Default constructor.
**Description:** Initializes Date instance with current time.
**2.9. Date 105**


**See:** ECMA-262, 21.4.2.1

publicconstructor(d: _Date_ ):void throws
`Date`constructor.
**Description:** Initializes`Date`instance with another`Date`instance.
**Arguments:**

- d: _Date_
**See:** ECMA-262, 21.4.2.1

publicconstructor(ms:number):void
`Date`constructor.
**Description:** Initialize`Date`instance with milliseconds given.
**Arguments:**

- ms:number
**See:** ECMA-262, 21.4.2.1

publicconstructor(year:number, month:number):void
`Date`constructor.
**Description:** Initialize`Date`instance with year and month given.
**Arguments:**

**106 Chapter 2. Classes**


- year:number
- month:number
**See:** ECMA-262, 21.4.2.1

publicconstructor( year:number, month:number, day:number):void
`Date`constructor.
**Description:** Initialize`Date`instance with year, month and day given.
**Arguments:**

- • year:month:numbernumber
- day:number
**See:** ECMA-262, 21.4.2.1

publicconstructor( year:number, month:number, day:number, hours:number):void
`Date`constructor.
**Description:** Initialize`Date`instance with year, month, day and hours given.
**Arguments:**

- year:number
- month:number
- day:number
- hours:number
**See:** ECMA-262, 21.4.2.1

**2.9. Date 107**


publicconstructor( year:number, month:number, day:number, hours:number, minutes:number):void
`Date`constructor.
**Description:** Initialize`Date`instance with year, month, day, hours and minutes given.
**Arguments:**

- year:number
- month:number
- • day:hours:numbernumber
- minutes:number
**See:** ECMA-262, 21.4.2.1

publicnumber):constructor( year:void number, month: number, day: number, hours: number, minutes: number, seconds:

`Date`constructor.
**Description:** Initialize`Date`instance with year, month, day, hours, minutes and seconds given.
**Arguments:**

- year:number
- month:number
- day:number
- hours:number
- minutes:number
- seconds:number
**See:** ECMA-262, 21.4.2.1

**108 Chapter 2. Classes**


publicnumber, ms:constructor( year:number):voidnumber, month: number, day: number, hours: number, minutes: number, seconds:

`Date`constructor.
**Description:** Initialize`Date`instance with year, month, day, hours, minutes, seconds and milliseconds given.
**Arguments:**

- year:number
- month:number
- • day:hours:numbernumber
- minutes:number
- seconds:number
- ms:number
**See:** ECMA-262, 21.4.2.1

publicconstructor(ms:number):void
`Date`constructor.
**Arguments:**

- ms:number
**See:** ECMA-262, 21.4.2.1

publicconstructor(year:number, month:number):void
`Date`constructor.
**Arguments:
2.9. Date 109**


- year:number
- month:number
**See:** ECMA-262, 21.4.2.1

publicconstructor( year:number, month:number, day:number):void
`Date`constructor.
**Arguments:**

- year:number
- month:number
- day:number
**See:** ECMA-262, 21.4.2.1

publicconstructor( year:number, month:number, day:number, hours:number):void
`Date`constructor.
**Arguments:**

- year:number
- month:number
- • day:hours:numbernumber

**See:** ECMA-262, 21.4.2.1

publicconstructor( year:number, month:number, day:number, hours:number, minutes:number):void
`Date`constructor.
**110 Chapter 2. Classes**


**Arguments:**

- year:number
- • month:day:numbernumber
- hours:number
- minutes:number
**See:** ECMA-262, 21.4.2.1

publicnumber):constructor( year:void number, month: number, day: number, hours: number, minutes: number, seconds:

`Date`constructor.
**Arguments:**

- year:number
- month:number
- day:number
- hours:number
- minutes:number
- seconds:number
**See:** ECMA-262, 21.4.2.1

publicnumber, ms:constructor( year:number):voidnumber, month: number, day: number, hours: number, minutes: number, seconds:

`Date`constructor.
**Arguments:**

- year:number

**2.9. Date 111**


- month:number
- day:number
- hours:number
- minutes:number
- seconds:number
- ms:number
**See:** ECMA-262, 21.4.2.1

publicgetDate():number
The`getDate()`method returns the day of the month for the specified date according to local time.
**Returns:** local time.An integer number, between 1 and 31, representing the day of the month for the given date according to

**See:** ECMA-262, 21.4.4.2

publicgetDay():number
Returns the day of the week for the specified date according to local time, where 0 represents Sunday. For the day ofthe month, seegetDayOfMonth.

**Returns:** local time: 0 for Sunday, 1 for Monday, 2 for Tuesday, and so on.An integer number, between 0 and 6, corresponding to the day of the week for the given date, according to

**See:** ECMA-262, 21.4.4.3

publicgetTime():number throws
Returns the number of milliseconds since the epoch, which is defined as the midnight at the beginning of January 1,1970, UTC.

**Returns:** A number representing the milliseconds elapsed between 1 January 1970 00:00:00 UTC and the given date.

**112 Chapter 2. Classes**


**See:** ECMA-262, 21.4.4.10

publicgetTimezoneOffset():number
Returns the difference, in minutes, between a date as evaluated in the UTC time zone, and the same date as evaluatedin the local time zone.

**Returns:** evaluated in the local time zone.the difference, in minutes, between a date as evaluated in the UTC time zone, and the same date as

publicgetUTCDate():number
Returns the day of the month (from 1 to 31) in the specified date according to universal time.
**Returns:** local time.An integer number, between 1 and 31, representing the day of the month for the given date according to

publicgetUTCDay():number
Returns the day of the week in the specified date according to universal time, where 0 represents Sunday.
**Returns:** local time: 0 for Sunday, 1 for Monday, 2 for Tuesday, and so on.An integer number, between 0 and 6, corresponding to the day of the week for the given date, according to

publicgetUTCFullYear():number
Returns the year of the specified date according to local time.
**Description:** 9999,`getUTCFullYear()The value returned by`returns a four-digit number, for example, 1995. Use this function to make sure a year is`getUTCFullYear()`is an absolute number. For dates between the years 1000 and
compliant with years after 2000.
**Returns:** A year of the specified date according to local time. year

**2.9. Date 113**


publicgetUTCHours():number
Returns the hours in the specified date according to universal time.
**Returns:** An integer number, between 0 and 23, representing the hour for the given date according to UTC time.

publicgetUTCMilliseconds():number
Returns the milliseconds portion of the time object’s value according to universal time.
**Returns:** the milliseconds portion of the time object’s value according to universal time.

publicgetUTCMinutes():number
Returns the minutes in the specified date according to universal time.
**Returns:** the minutes in the specified date according to universal time.

publicgetUTCMonth():number
Returns the month of the specified date according to universal time, as a zero-based value (where zero indicates thefirst month of the year).

**Returns:** corresponds to January, 1 to February, and so on.An integer number, between 0 and 11, representing the month in the given date according to UTC time. 0

publicgetUTCSeconds():number
Returns the seconds in the specified date according to universal time.
**Returns:** the seconds in the specified date according to universal time.
**114 Chapter 2. Classes**


publicgetYear():number
Returns the year of the specified date according to local time.
**Returns:** year
**See:** ECMA-262, 21.4.4.4 deprecated
**Note:** This function is an alias togetFullYearand left for compatibility with ECMA-262.

publicisDateValid():boolean
Therelative to January 1, 1970 UTC (that is, April 20, 271821 BCE ~ September 13, 275760 CE) can be represented by`isDateValid()`method checks if constructed date is maximum of±100,000,000 (one hundred million) days
the standard Date object (equivalent to±8,640,000,000,000,000 milliseconds).

publicsetDate(value:number):void
Changes the day of the month of a given Date instance, based on local time.
**Arguments:**

- value:numbernew day.

publicsetDay(value:number):void
Alias tosetDateand left for compatibility with ECMA-262.
**Arguments:**

- value:numbernew day.

publicsetFullYear(value:number):void
**2.9. Date 115**


Sets the full year for a specified date according to local time.
**Arguments:**

- value:numbernew year

publicsetHours(value:number):void
Sets the hours for a specified date according to local time.
**Arguments:**

- value:numbernew hours

publicsetMilliseconds(value:number):void
Sets the milliseconds for a specified date according to local time.
**Arguments:**

- value:numbernew ms
publicsetMinutes(value:number):void
Sets the minutes for a specified date according to local time.
**Arguments:**
- value:numbernew minutes
publicsetMonth(month:number):void
Sets the month for a specified date according to the currently set year.
**Arguments:**
- month:numbernew month

publicsetSeconds(value:number):void

**116 Chapter 2. Classes**


Sets the seconds for a specified date according to local time.
**Arguments:**

- value:numbernew seconds
publicsetTime(value:number):void
Sets the number of milliseconds since the epoch, which is defined as the midnight at the beginning of January 1,1970, UTC.

**Returns:** A number representing the milliseconds elapsed between 1 January 1970 00:00:00 UTC and the given date.
**Arguments:**

- value:numbernew ms
**See:** ECMA-262, 21.4.4.10

publicsetTimezoneOffset(value:number):number
Sets the difference, in minutes, between a date as evaluated in the UTC time zone, and the same date as evaluated inthe local time zone.

**Arguments:**

- value:numbernew timezone offset
publicsetUTCDate(value:number):void
Changes the day of the month of a given Date instance, based on UTC time.
**Arguments:**
- value:numbernew day.

publicsetUTCDay(value:number):void
Changes the day of the month of a given Date instance, based on UTC time.
**2.9. Date 117**


**Arguments:**

- value:numbernew day.

publicsetUTCFullYear(value:number):void
Sets the full year for a specified date according to universal time.
**Arguments:**

- value:numbernew year

publicsetUTCHours(value:number):void
Sets the hour for a specified date according to universal time.
**Arguments:**

- value:numbernew hours
publicsetUTCMilliseconds(value:number):void
Sets the milliseconds for a specified date according to universal time.
**Arguments:**
- value:numbernew ms
publicsetUTCMinutes(value:number):void
Sets the minutes for a specified date according to universal time.
**Arguments:**
- value:numbernew minutes
publicsetUTCMonth(month:number):void
Sets the month for a specified date according to universal time.
**118 Chapter 2. Classes**


**Arguments:**

- month:numbernew month
publicsetUTCSeconds(value:number):void
Sets the seconds for a specified date according to universal time.
**Arguments:**
- value:numbernew seconds

publicsetYear(value:number):void
This function is an alias tosetFullYearand left for compatibility with ECMA-262.
**Arguments:**

- value:numbernew year

publictoJSON(): _string_
Returns a string representation of the Date object.
**Returns:** JSON representation of the current instance

publictoLocaleDatestring(): _string_
Gets a string with a language-sensitive representation of the date portion of the specified date in the user agent’stimezone.

**Returns:** timezone.a string with a language-sensitive representation of the date portion of the specified date in the user agent’s

publictoLocaleDatestring(locale: _string_ ): _string_
**2.9. Date 119**


Returns a string with a language-sensitive representation of the date portion of the specified date in the user agent’stimezone.

**Returns:** timezone.a string with a language-sensitive representation of the date portion of the specified date in the user agent’s

publictoLocaleString(): _string_
Gets a string with a language-sensitive representation of this date.
**Returns:** a language-sensitive representation of this date.

publictoLocaleString(locale: _string_ ): _string_
Gets a string with a language-sensitive representation of this date with respect to locale.
**Returns:** a language-sensitive representation of this date.
**Arguments:**

- locale: _string_

publictoLocaleTimestring(): _string_
Gets a string with a language-sensitive representation of the time portion of the date.
**Returns:** a language-sensitive representation of the time portion of the date.

publictoLocaleTimestring(locale: _string_ ): _string_
Gets a string with a language-sensitive representation of the time portion of the date with respect to locale.
**Returns:** a language-sensitive representation of the time portion of the date with respect to locale.
**Arguments:
120 Chapter 2. Classes**


- locale: _string_
publicvalueOf():number throws
The`valueOf()`method returns the primitive value of a`Date`object.
**Returns:** throws InvalidDate - Throws if Date object is invalid (The number of milliseconds between 1 January 1970 00:00:00 UTC and the given date.isDateValidis`false`).

**See:** ECMA-262, 21.4.4.44

public staticUTC(d: _Date_ ):number throws
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- d: _Date_ to be converted to milliseconds.
**See:** ECMA-262, 21.4.3.1

public staticUTC(year:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
public staticUTC(year:number, month:number):number
**2.9. Date 121**


Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
public staticUTC( year:number, month:number, day:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.

public staticUTC( year:number, month:number, day:number, hours:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- hours:numberto be converted to milliseconds.
public staticUTC( year:number, month:number, day:number, hours:number, minutes:number):number

**122 Chapter 2. Classes**


Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- • hours:minutes:numbernumberto be converted to milliseconds.to be converted to milliseconds.

public staticnumber):numberUTC( year:number, month: number, day:number, hours:number, minutes: number, seconds:

Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- • year:month:numbernumberto be converted to milliseconds.to be converted to milliseconds.
- day:numberto be converted to milliseconds.
- hours:numberto be converted to milliseconds.
- minutes:numberto be converted to milliseconds.
- seconds:numberto be converted to milliseconds.

public staticnumber, ms:numberUTC( year:):numbernumber, month: number, day:number, hours:number, minutes: number, seconds:

Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

**2.9. Date 123**


- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- hours:numberto be converted to milliseconds.
- minutes:numberto be converted to milliseconds.
- seconds:numberto be converted to milliseconds.
- ms:numberto be converted to milliseconds.
public staticUTC(year:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.

public staticUTC(year:number, month:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
public staticUTC( year:number, month:number, day:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**124 Chapter 2. Classes**


**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.

public staticUTC( year:number, month:number, day:number, hours:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- hours:numberto be converted to milliseconds.
public staticUTC( year:number, month:number, day:number, hours:number, minutes:number):number
Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- hours:numberto be converted to milliseconds.
- minutes:numberto be converted to milliseconds.
public staticnumber):numberUTC( year:number, month: number, day:number, hours:number, minutes: number, seconds:

**2.9. Date 125**


Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- • hours:minutes:numbernumberto be converted to milliseconds.to be converted to milliseconds.
- seconds:numberto be converted to milliseconds.

public staticnumber, ms:numberUTC( year:):numbernumber, month: number, day:number, hours:number, minutes: number, seconds:

Returns the number of milliseconds elapsed since the epoch, which is defined as the midnight at the beginning ofJanuary 1, 1970, UTC.

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**Arguments:**

- year:numberto be converted to milliseconds.
- month:numberto be converted to milliseconds.
- day:numberto be converted to milliseconds.
- hours:numberto be converted to milliseconds.
- minutes:numberto be converted to milliseconds.
- seconds:numberto be converted to milliseconds.
- ms:numberto be converted to milliseconds.

public staticgetLocalTimezoneOffset():number
Gets local time offset.
**Returns:** local time offset.

**126 Chapter 2. Classes**


public staticgetLocalestring( format: _string_ , locale: _string_ , ms:number, isUTC:boolean): _string_
Gets locale string representation according to format.
**Returns:** locale string in the specified format.
**Arguments:**

- format: _string_
- • locale:ms:number _string_
- isUTC:boolean

public staticgetTimezoneName(): _string_
Gets time zone name.
**Returns:** time zone name.

public staticnow():number
Themidnight at the beginning of January 1, 1970, UTC.`now()`static method returns the number of milliseconds elapsed since the epoch, which is defined as the

**Returns:** midnight at the beginning of January 1, 1970, UTC.A number representing the number of milliseconds elapsed since the epoch, which is defined as the

**See:** ECMA-262, 21.4.3.1

public staticparse(dateStr: _string_ ):number throws
Parses a string representation of a date, and returns the number of milliseconds since January 1, 1970, 00:00:00 UTCor raises`InvalidDate`if the string is unrecognized or, in some cases, contains illegal date values (e.g. 2015-02-31).
Only the ISO 8601 format (YYYY-MM-DDTHH:mm:ss.sssZ) is explicitly specified to be supported. Other formatsare implementation-defined and may not work across all browsers (targets). A library can help if many different
formats are to be accommodated.
**2.9. Date 127**


**Returns:** obtained by parsing the given string representation of a date. If the argument doesn’t represent a valid date,A number representing the milliseconds elapsed since January 1, 1970, 00:00:00 UTC and the date
`InvalidDate`exception is thrown.
**Arguments:**

- dateStr: _string_ to be parsed
**See:** ECMA-262, 21.4.3.2

**2.10 Error**
export
Strores information about stacktrace and cause in case of an error. Serves as a base class for all error classes.

**2.10.1 Methods**
publicconstructor():void
Constructs a new empty error instance

publicconstructor(msg: _string_ ):void
Constructs a new error instance with provided message
**Arguments:**

- msg: _string_ message of the error

publicconstructor(msg: _string_ , cause: _object_ ):void
Constructs a new error instance with provided message and cause

**128 Chapter 2. Classes**


**Arguments:**

- msg: _string_ message of the error
- cause: _object_ cause of the error

public overridetoString(): _string_
Converts this error to a string Result includes error message and the stacktrace
**Returns:** result of the conversion

**2.10.2 Properties**

- cause: _object_
- • message:name: _stringstring_
- stack: _string_
**2.11 EvalError**
export extends _Error_
**Class:** Represents an error that occurs when global eval() function fails

**2.11.1 Methods**
publicconstructor():void
Constructs a new instance of error

publicconstructor(s: _string_ ):void
Constructs a new instance of error
**Arguments:
2.11. EvalError 129**


- s: _string_

publicconstructor(s: _string_ , cause: _object_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_
- cause: _object_

**2.11.2 Properties**

- cause: _object_
- message: _string_
- name: _string_
- stack: _string_
**2.12 FinalizationRegistry<T>**
export
**Interface:** Represents a FinalizationRegistry

**2.12.1 Methods**
publicregister(target: _WeakKey_ , heldValue: T, unregisterToken?: _WeakKey_ ): void;

publicunregister(unregisterToken: _WeakKey_ ):void

**130 Chapter 2. Classes**


**2.13 Float32Array**
export
JS Float32Array API-compatible class

**2.13.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at
publicconstructor():void
Creates an empty Float32Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Float32Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- • byteOffset:length:numbernumbersize of elements of type float in newly created Float32Arraybyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Float32Array with respect to buf and byteOffset.
**Arguments:**

**2.13. Float32Array 131**


- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Float32Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Float32Array_ ):void
Creates a copy of Float32Array.
**Arguments:**

- other: _Float32Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Float32Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

**132 Chapter 2. Classes**


publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Float32Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Float32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Float32Array_ ) =>boolean):boolean
Checks that all elements of Float32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Float32Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Float32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Float32Array_
**2.13. Float32Array 133**


Fills the Float32Array with specified value
**Returns:** modified Float32Array
**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Float32Array_
Fills the Float32Array with specified value
**Returns:** modified Float32Array
**Arguments:**
- value:numbernew valuy
- start:number
publicfill(value:number): _Float32Array_
Fills the Float32Array with specified value
**Returns:** modified Float32Array
**Arguments:**
- value:numbernew valuy

publicfilter( fn: (val:number) =>boolean): _Float32Array_
creates a new Float32Array from current Float32Array based on a condition fn
**Returns:** a new Float32Array with elements from current Float32Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

**134 Chapter 2. Classes**


publicfilter( fn: (val:number, index:number, array: _Float32Array_ ) =>boolean): _Float32Array_
Creates a new Float32Array from current Float32Array based on a condition fn.
**Returns:** a new Float32Array with elements from current Float32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Float32Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Float32Array_
creates a new Float32Array from current Float32Array based on a condition fn
**Returns:** a new Float32Array with elements from current Float32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Float32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Float32Array_ ) =>boolean):number
Finds the first element in the Float32Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return float | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Float32Array_ ) =>booleanthe condition to apply for each element

**2.13. Float32Array 135**


publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Float32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Float32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Float32Array_ ) =>boolean):number
Finds an index of the first element in the Float32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Float32Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Float32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**136 Chapter 2. Classes**


publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Float32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Float32Array_ ) =>boolean):number
Finds the last element in the Float32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Float32Array_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Float32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Float32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

**2.13. Float32Array 137**


publicfindLastIndex( fn: (val:number, index:number, array: _Float32Array_ ) =>boolean):number
Finds an index of the last element in the Float32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Float32Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Float32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Float32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply

publicforEach( fn: (val:number, index:number, array: _Float32Array_ ) =>number):void
Applies a function over all elements of Float32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Float32Array_ ) =>numberfunction to apply

**138 Chapter 2. Classes**


publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Float32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply
publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Float32Array_
Creates an Float32Array from array-like argument
**Returns:** new Float32Array
**Arguments:**
- o: _object_ array-like object to initialize Float32Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each

publicfrom(o: _object_ ): _Float32Array_
Creates an Float32Array from array-like argument
**Returns:** new Float32Array
**Arguments:**

- o: _object_ array-like object to initialize Float32Array
publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Float32Array_
Creates an Float32Array from array-like argument
**Returns:** new Float32Array
**Arguments:**
- o: _object_ array-like object to initialize Float32Array
**2.13. Float32Array 139**


- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Float32Array
**Returns:** true if e is in Float32Array, false otherwise
**Arguments:**

- • e:fromIndex:numbersearch elementnumberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Float32Array
**Returns:** true if e is in Float32Array, false otherwise
**Arguments:**

- e:numbersearch element
publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
- fromIndex:numberstart index to search from
publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:
140 Chapter 2. Classes**


- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**
- s: _string_ separator
publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publickeys(): _IterableIterator_ <number>
Returns keys of the Float32Array
**Returns:** iterator over keys

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space
publiclastIndexOf(val:number):number
**2.13. Float32Array 141**


Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search
publicmap( fn: (val:number) =>number): _Float32Array_
Creates a new Float32Array using fn(arr[i]) over all elements of current Float32Array
**Returns:** a new Float32Array where for each element from current Float32Array fn was applied
**Arguments:**
- fn: (val:number) =>numbera function to apply for each element of current Float32Array
publicmap( fn: (val:number, index:number) =>number): _Float32Array_
Creates a new Float32Array using fn(arr[i]) over all elements of current Float32Array.
**Returns:** a new Float32Array where for each element from current Float32Array fn was applied
**Arguments:**
- fn: (val:number, index:number) =>numbera function to apply for each element of current Float32Array
publicof(data:number[]): _Float32Array_
Creates a new Float32Array using initializer
**Returns:** a new Float32Array from data
**Arguments:**
- data:number[] initializer
public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Float32Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**142 Chapter 2. Classes**


**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Float32Array_ ) =>numbercondition
- init:numberinitial value
publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Float32Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Float32Array_ ) =>numbercondition
publicnumberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Float32Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Float32Array_ ) =>numbercondition
- init:numberinitial value
publicnumberreduceRight( fn: (acc: number, curVal: number, curIndex: number, array: _Float32Array_ ) =>number):

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Float32Array_ ) =>numbercondition
publicreverse(): _Float32Array_
**2.13. Float32Array 143**


Creates a new Float32Array using reversed data from the current one
**Returns:** a new Float32Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(float[], insertPos)
**Arguments:**

- insertPos:numberindex to change
- val:numbervalue to set
publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Float32Array starting from insertPos.
**Arguments:**
- arr:number[] array to copy data from
- insertPos1:number
publicset(arr:number[]):void
Copies all elements of arr to the current Float32Array.
**Arguments:**
- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Float32Array_
Creates a slice of current Float32Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Float32Array with elements of current Float32Array[begin;end) where end index is excluded

**144 Chapter 2. Classes**


**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _Float32Array_
Creates a slice of current Float32Array using range [begin, this.length).
**Returns:** a new Float32Array with elements of current Float32Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _Float32Array_
Creates a slice of current Float32 with all elements.
**Returns:** a new Float32Array with elements of current Float32Array

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Float32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicsome( fn: (element:number, index:number, array: _Float32Array_ ) =>boolean):boolean
Checks that at least one element of Float32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

**2.13. Float32Array 145**


- fn: (element:number, index:number, array: _Float32Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Float32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Float32Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted Float32Array

publicsort( fn: (a:number, b:number) =>number): _Float32Array_
Sorts in-place
**Returns:** sorted Float32Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Float32Array_
Creates a Float32Array with the same underlying ArrayBuffer
**Returns:** new Float32Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive
**146 Chapter 2. Classes**


publicsubarray(begin:number): _Float32Array_
Creates a Float32Array with the same ArrayBuffer
**Returns:** new Float32Array with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

publicsubarray(): _Float32Array_
Creates a Float32Array with the same ArrayBuffer
**Returns:** new Float32Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Float32Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts Float32Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_

**2.13. Float32Array 147**


publictoLocaleString(): _string_
Converts Float32Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Float32Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Float32Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Float32Array
**Returns:** a string representation of the Float32Array

publicvalues(): _IterableIterator_ <number>
Returns array values iterator
**Returns:** an iterator

**148 Chapter 2. Classes**


publicwith(index:number, value:number): _Float32Array_
Creates a copy with replaced value on index
**Returns:** an Float32Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.13.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
- length:number
**2.14 Float64Array**
export
JS Float64Array API-compatible class

**2.14.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at

publicconstructor():void

**2.14. Float64Array 149**


Creates an empty Float64Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Float64Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type number in newly created Float64Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Float64Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Float64Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Float64Array_ ):void
Creates a copy of Float64Array.
**Arguments:**

- other: _Float64Array_ data initializer

**150 Chapter 2. Classes**


publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Float64Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Float64Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Float64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function
publicevery( fn: (element:number, index:number, array: _Float64Array_ ) =>boolean):boolean
**2.14. Float64Array 151**


Checks that all elements of Float64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Float64Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Float64Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Float64Array_
Fills the Float64Array with specified value
**Returns:** modified Float64Array
**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Float64Array_
Fills the Float64Array with specified value
**Returns:** modified Float64Array
**Arguments:**
- value:numbernew valuy
- start:number

**152 Chapter 2. Classes**


publicfill(value:number): _Float64Array_
Fills the Float64Array with specified value
**Returns:** modified Float64Array
**Arguments:**

- value:numbernew valuy

publicfilter( fn: (val:number) =>boolean): _Float64Array_
creates a new Float64Array from current Float64Array based on a condition fn
**Returns:** a new Float64Array with elements from current Float64Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Float64Array_ ) =>boolean): _Float64Array_
Creates a new Float64Array from current Float64Array based on a condition fn.
**Returns:** a new Float64Array with elements from current Float64Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Float64Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Float64Array_
creates a new Float64Array from current Float64Array based on a condition fn
**Returns:** a new Float64Array with elements from current Float64Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

**2.14. Float64Array 153**


publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Float64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Float64Array_ ) =>boolean):number
Finds the first element in the Float64Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Float64Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Float64Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Float64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

**154 Chapter 2. Classes**


publicfindIndex( fn: (val:number, index:number, array: _Float64Array_ ) =>boolean):number
Finds an index of the first element in the Float64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Float64Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Float64Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Float64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Float64Array_ ) =>boolean):number
Finds the last element in the Float64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Float64Array_ ) =>booleancondition

**2.14. Float64Array 155**


publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Float64Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Float64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Float64Array_ ) =>boolean):number
Finds an index of the last element in the Float64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Float64Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Float64Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**156 Chapter 2. Classes**


publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Float64Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply
publicforEach( fn: (val:number, index:number, array: _Float64Array_ ) =>number):void
Applies a function over all elements of Float64Array
**Returns:** undefined
**Arguments:**
- fn: (val:number, index:number, array: _Float64Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Float64Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply
publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Float64Array_
Creates an Float64Array from array-like argument
**Returns:** new Float64Array
**Arguments:**
- o: _object_ array-like object to initialize Float64Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each
**2.14. Float64Array 157**


publicfrom(o: _object_ ): _Float64Array_
Creates an Float64Array from array-like argument
**Returns:** new Float64Array
**Arguments:**

- o: _object_ array-like object to initialize Float64Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Float64Array_
Creates an Float64Array from array-like argument
**Returns:** new Float64Array
**Arguments:**

- o: _object_ array-like object to initialize Float64Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Float64Array
**Returns:** true if e is in Float64Array, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Float64Array
**Returns:** true if e is in Float64Array, false otherwise
**Arguments:**

**158 Chapter 2. Classes**


- e:numbersearch element
publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
- fromIndex:numberstart index to search from

publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**

- e:numbersearch element

publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**

- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

**2.14. Float64Array 159**


publickeys(): _IterableIterator_ <number>
Returns keys of the Float64Array
**Returns:** iterator over keys

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publicmap( fn: (val:number) =>number): _Float64Array_
Creates a new Float64Array using fn(arr[i]) over all elements of current Float64Array
**Returns:** a new Float64Array where for each element from current Float64Array fn was applied
**Arguments:**

- fn: (val:number) =>numbera function to apply for each element of current Float64Array

publicmap( fn: (val:number, index:number) =>number): _Float64Array_
Creates a new Float64Array using fn(arr[i]) over all elements of current Float64Array.
**Returns:** a new Float64Array where for each element from current Float64Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Float64Array

**160 Chapter 2. Classes**


publicof(data: :number[]): _Float64Array_
Creates a new Float64Array using initializer
**Returns:** a new Float64Array from data
**Arguments:**

- data:number[] initializer

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Float64Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Float64Array_ ) =>numbercondition
- init:numberinitial value

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Float64Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Float64Array_ ) =>numbercondition

publicnumberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Float64Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:
2.14. Float64Array 161**


- fn: (acc:number, curVal:number, curIndex:number, array: _Float64Array_ ) =>numbercondition
- init:numberinitial value
publicnumberreduceRight( fn: (acc: number, curVal: number, curIndex: number, array: _Float64Array_ ) =>number):

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Float64Array_ ) =>numbercondition

publicreverse(): _Float64Array_
Creates a new Float64Array using reversed data from the current one
**Returns:** a new Float64Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(number[], insertPos)
**Arguments:**

- insertPos:numberindex to change
- val:numbervalue to set
publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Float64Array starting from insertPos.
**Arguments:**
- arr:number[] array to copy data from

**162 Chapter 2. Classes**


- insertPos1:number
publicslice(begin:number, end:number): _Float64Array_
Creates a slice of current Float64Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Float64Array with elements of current Float64Array[begin;end) where end index is excluded

**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _Float64Array_
Creates a slice of current Float64Array using range [begin, this.length).
**Returns:** a new Float64Array with elements of current Float64Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _Float64Array_
Creates a slice of current Float64 with all elements.
**Returns:** a new Float64Array with elements of current Float64Array

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Float64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function
**2.14. Float64Array 163**


publicsome( fn: (element:number, index:number, array: _Float64Array_ ) =>boolean):boolean
Checks that at least one element of Float64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Float64Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Float64Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Float64Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted Float64Array

publicsort( fn: (a:number, b:number) =>number): _Float64Array_
Sorts in-place
**Returns:** sorted Float64Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Float64Array_
**164 Chapter 2. Classes**


Creates a Float64Array with the same underlying ArrayBuffer
**Returns:** new Float64Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive
publicsubarray(begin:number): _Float64Array_
Creates a Float64Array with the same ArrayBuffer
**Returns:** new Float64Array with the same ArrayBuffer
**Arguments:**
- begin:numberstart index, inclusive
publicsubarray(): _Float64Array_
Creates a Float64Array with the same ArrayBuffer
**Returns:** new Float64Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Float64Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_
publictoLocaleString(locales: _object_ ): _string_
Converts Float64Array to a string with respect to locale
**2.14. Float64Array 165**


**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Float64Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Float64Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Float64Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Float64Array
**Returns:** a string representation of the Float64Array

publicvalues(): _IterableIterator_ <number>
**166 Chapter 2. Classes**


Returns array values iterator
**Returns:** an iterator

publicwith(index:number, value:number): _Float64Array_
Creates a copy with replaced value on index
**Returns:** an Float64Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.14.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- • byteLength:byteOffset:numbernumber
- length:number
**2.15 Int16Array**
export
JS Int16Array API-compatible class

**2.15.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

**2.15. Int16Array 167**


- index:numberindex to look at
publicconstructor():void
Creates an empty Int16Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Int16Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type short in newly created Int16Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Int16Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Int16Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Int16Array_ ):void
Creates a copy of Int16Array.

**168 Chapter 2. Classes**


**Arguments:**

- other: _Int16Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Int16Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Int16Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Int16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

**2.15. Int16Array 169**


- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Int16Array_ ) =>boolean):boolean
Checks that all elements of Int16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Int16Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Int16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Int16Array_
Fills the Int16Array with specified value
**Returns:** modified Int16Array
**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Int16Array_
Fills the Int16Array with specified value
**Returns:** modified Int16Array

**170 Chapter 2. Classes**


**Arguments:**

- value:numbernew valuy
- start:number
publicfill(value:number): _Int16Array_
Fills the Int16Array with specified value
**Returns:** modified Int16Array
**Arguments:**
- value:numbernew valuy

publicfilter( fn: (val:number) =>boolean): _Int16Array_
creates a new Int16Array from current Int16Array based on a condition fn
**Returns:** a new Int16Array with elements from current Int16Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Int16Array_ ) =>boolean): _Int16Array_
Creates a new Int16Array from current Int16Array based on a condition fn.
**Returns:** a new Int16Array with elements from current Int16Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Int16Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Int16Array_
creates a new Int16Array from current Int16Array based on a condition fn
**Returns:** a new Int16Array with elements from current Int16Array that satisfy condition fn

**2.15. Int16Array 171**


**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Int16Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Int16Array_ ) =>boolean):number
Finds the first element in the Int16Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return short | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Int16Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Int16Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Int16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:
172 Chapter 2. Classes**


- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Int16Array_ ) =>boolean):number
Finds an index of the first element in the Int16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Int16Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Int16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Int16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Int16Array_ ) =>boolean):number
Finds the last element in the Int16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Int16Array_ ) =>booleancondition
**2.15. Int16Array 173**


publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Int16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Int16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Int16Array_ ) =>boolean):number
Finds an index of the last element in the Int16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Int16Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Int16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**174 Chapter 2. Classes**


publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Int16Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply
publicforEach( fn: (val:number, index:number, array: _Int16Array_ ) =>number):void
Applies a function over all elements of Int16Array
**Returns:** undefined
**Arguments:**
- fn: (val:number, index:number, array: _Int16Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Int16Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply
publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Int16Array_
Creates an Int16Array from array-like argument
**Returns:** new Int16Array
**Arguments:**
- o: _object_ array-like object to initialize Int16Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each
**2.15. Int16Array 175**


publicfrom(o: _object_ ): _Int16Array_
Creates an Int16Array from array-like argument
**Returns:** new Int16Array
**Arguments:**

- o: _object_ array-like object to initialize Int16Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Int16Array_
Creates an Int16Array from array-like argument
**Returns:** new Int16Array
**Arguments:**

- o: _object_ array-like object to initialize Int16Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Int16Array
**Returns:** true if e is in Int16Array, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Int16Array
**Returns:** true if e is in Int16Array, false otherwise
**Arguments:**

**176 Chapter 2. Classes**


- e:numbersearch element
publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
- fromIndex:numberstart index to search from

publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**

- e:numbersearch element

publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**

- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

**2.15. Int16Array 177**


publickeys(): _IterableIterator_ <number>
Returns keys of the Int16Array
**Returns:** iterator over keys

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search

publicmap( fn: (val:number) =>number): _Int16Array_
Creates a new Int16Array using fn(arr[i]) over all elements of current Int16Array
**Returns:** a new Int16Array where for each element from current Int16Array fn was applied
**Arguments:**

- fn: (val:number) =>numbera function to apply for each element of current Int16Array

**178 Chapter 2. Classes**


publicmap( fn: (val:number, index:number) =>number): _Int16Array_
Creates a new Int16Array using fn(arr[i]) over all elements of current Int16Array.
**Returns:** a new Int16Array where for each element from current Int16Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Int16Array

publicof(data:number[]): _Int16Array_
Creates a new Int16Array using initializer
**Returns:** a new Int16Array from data
**Arguments:**

- data:number[] initializer

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Int16Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- • fn: (acc:init:numbernumberinitial value, curVal:number, curIndex:number, array: _Int16Array_ ) =>numbercondition

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Int16Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

**2.15. Int16Array 179**


- fn: (acc:number, curVal:number, curIndex:number, array: _Int16Array_ ) =>numbercondition

publicnumber):reduceRight( fn: (acc:number number, curVal: number, curIndex:number, array: _Int16Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Int16Array_ ) =>numbercondition
- init:numberinitial value
publicreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Int16Array_ ) =>number):number
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Int16Array_ ) =>numbercondition

publicreverse(): _Int16Array_
Creates a new Int16Array using reversed data from the current one
**Returns:** a new Int16Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(short[], insertPos)
**Arguments:**

- insertPos:numberindex to change
**180 Chapter 2. Classes**


- val:numbervalue to set
publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Int16Array starting from insertPos.
**Arguments:**
- arr:number[] array to copy data from
- insertPos1:number
publicset(arr:number[]):void
Copies all elements of arr to the current Int16Array.
**Arguments:**
- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Int16Array_
Creates a slice of current Int16Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Int16Array with elements of current Int16Array[begin;end) where end index is excluded

**Arguments:**

- • begin:end:numbernumberlast index to be taken into slicestart index to be taken into slice

publicslice(begin:number): _Int16Array_
Creates a slice of current Int16Array using range [begin, this.length).
**Returns:** a new Int16Array with elements of current Int16Array[begin, this.length)
**Arguments:**

- begin:numberstart index to be taken into slice

**2.15. Int16Array 181**


publicslice(): _Int16Array_
Creates a slice of current Int16 with all elements.
**Returns:** a new Int16Array with elements of current Int16Array

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Int16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicsome( fn: (element:number, index:number, array: _Int16Array_ ) =>boolean):boolean
Checks that at least one element of Int16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Int16Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Int16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Int16Array_
**182 Chapter 2. Classes**


Sorts in-place according to the numeric ordering
**Returns:** sorted Int16Array

publicsort( fn: (a:number, b:number) =>number): _Int16Array_
Sorts in-place
**Returns:** sorted Int16Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Int16Array_
Creates a Int16Array with the same underlying ArrayBuffer
**Returns:** new Int16Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive

publicsubarray(begin:number): _Int16Array_
Creates a Int16Array with the same ArrayBuffer
**Returns:** new Int16Array with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

publicsubarray(): _Int16Array_
Creates a Int16Array with the same ArrayBuffer

**2.15. Int16Array 183**


**Returns:** new Int16Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Int16Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts Int16Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Int16Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Int16Array_
Creates a reversed copy
**Returns:** a reversed copy

**184 Chapter 2. Classes**


publictoSorted(): _Int16Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Int16Array
**Returns:** a string representation of the Int16Array

publicvalues(): _IterableIterator_ <number>
Returns array values iterator
**Returns:** an iterator

publicwith(index:number, value:number): _Int16Array_
Creates a copy with replaced value on index
**Returns:** an Int16Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.15. Int16Array 185**


**2.15.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
- length:number
**2.16 Int32Array**
export
JS Int32Array API-compatible class

**2.16.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at

publicconstructor():void
Creates an empty Int32Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Int32Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

**186 Chapter 2. Classes**


- length:numbersize of elements of type int in newly created Int32Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Int32Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Int32Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Int32Array_ ):void
Creates a copy of Int32Array.
**Arguments:**

- other: _Int32Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void

**2.16. Int32Array 187**


Makes a copy of internal elements to insertPos from startPos to end of Int32Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Int32Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Int32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Int32Array_ ) =>boolean):boolean
Checks that all elements of Int32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Int32Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Int32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**188 Chapter 2. Classes**


**Arguments:**

- fn: (element:number, index:number) =>booleancheck function
publicfill( value:number, start:number, end:number): _Int32Array_
Fills the Int32Array with specified value
**Returns:** modified Int32Array
**Arguments:**
- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Int32Array_
Fills the Int32Array with specified value
**Returns:** modified Int32Array
**Arguments:**
- value:numbernew valuy
- start:number
publicfill(value:number): _Int32Array_
Fills the Int32Array with specified value
**Returns:** modified Int32Array
**Arguments:**
- value:numbernew valuy
publicfilter( fn: (val:number) =>boolean): _Int32Array_
creates a new Int32Array from current Int32Array based on a condition fn
**2.16. Int32Array 189**


**Returns:** a new Int32Array with elements from current Int32Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Int32Array_ ) =>boolean): _Int32Array_
Creates a new Int32Array from current Int32Array based on a condition fn.
**Returns:** a new Int32Array with elements from current Int32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Int32Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Int32Array_
creates a new Int32Array from current Int32Array based on a condition fn
**Returns:** a new Int32Array with elements from current Int32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Int32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Int32Array_ ) =>boolean):number
Finds the first element in the Int32Array that satisfies the condition

**190 Chapter 2. Classes**


**Returns:** the first element that satisfies fn TODO: return int | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Int32Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Int32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Int32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Int32Array_ ) =>boolean):number
Finds an index of the first element in the Int32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Int32Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Int32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**2.16. Int32Array 191**


**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Int32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Int32Array_ ) =>boolean):number
Finds the last element in the Int32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Int32Array_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Int32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Int32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise

**192 Chapter 2. Classes**


**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Int32Array_ ) =>boolean):number
Finds an index of the last element in the Int32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Int32Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Int32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Int32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply

publicforEach( fn: (val:number, index:number, array: _Int32Array_ ) =>number):void
Applies a function over all elements of Int32Array
**Returns:** undefined
**Arguments:
2.16. Int32Array 193**


- fn: (val:number, index:number, array: _Int32Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Int32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Int32Array_
Creates an Int32Array from array-like argument
**Returns:** new Int32Array
**Arguments:**

- o: _object_ array-like object to initialize Int32Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each

publicfrom(o: _object_ ): _Int32Array_
Creates an Int32Array from array-like argument
**Returns:** new Int32Array
**Arguments:**

- o: _object_ array-like object to initialize Int32Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Int32Array_
Creates an Int32Array from array-like argument
**Returns:** new Int32Array
**Arguments:**

**194 Chapter 2. Classes**


- o: _object_ array-like object to initialize Int32Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Int32Array
**Returns:** true if e is in Int32Array, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Int32Array
**Returns:** true if e is in Int32Array, false otherwise
**Arguments:**

- e:numbersearch element

publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**2.16. Int32Array 195**


**Arguments:**

- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**
- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publickeys(): _IterableIterator_ <number>
Returns keys of the Int32Array
**Returns:** iterator over keys

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
**196 Chapter 2. Classes**


- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search
publicmap( fn: (val:number) =>number): _Int32Array_
Creates a new Int32Array using fn(arr[i]) over all elements of current Int32Array
**Returns:** a new Int32Array where for each element from current Int32Array fn was applied
**Arguments:**
- fn: (val:number) =>numbera function to apply for each element of current Int32Array

publicmap( fn: (val:number, index:number) =>number): _Int32Array_
Creates a new Int32Array using fn(arr[i]) over all elements of current Int32Array.
**Returns:** a new Int32Array where for each element from current Int32Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Int32Array

publicof(data:number[]): _Int32Array_
Creates a new Int32Array using initializer
**Returns:** a new Int32Array from data
**Arguments:**

- data:number[] initializer
**2.16. Int32Array 197**


public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>numbercondition
- init:numberinitial value
publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>numbercondition

publicnumber):reduceRight( fn: (acc:number number, curVal: number, curIndex:number, array: _Int32Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>numbercondition
- init:numberinitial value
publicreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>number):number
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result

**198 Chapter 2. Classes**


**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Int32Array_ ) =>numbercondition

publicreverse(): _Int32Array_
Creates a new Int32Array using reversed data from the current one
**Returns:** a new Int32Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(int[], insertPos)
**Arguments:**

- • insertPos:val:numbernumbervalue to setindex to change

publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Int32Array starting from insertPos.
**Arguments:**

- arr:number[] array to copy data from
- insertPos1:number

publicset(arr:number[]):void
Copies all elements of arr to the current Int32Array.
**Arguments:**

- arr:number[] array to copy data from

**2.16. Int32Array 199**


publicslice(begin:number, end:number): _Int32Array_
Creates a slice of current Int32Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Int32Array with elements of current Int32Array[begin;end) where end index is excluded

**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _Int32Array_
Creates a slice of current Int32Array using range [begin, this.length).
**Returns:** a new Int32Array with elements of current Int32Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _Int32Array_
Creates a slice of current Int32 with all elements.
**Returns:** a new Int32Array with elements of current Int32Array

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Int32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

**200 Chapter 2. Classes**


publicsome( fn: (element:number, index:number, array: _Int32Array_ ) =>boolean):boolean
Checks that at least one element of Int32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Int32Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Int32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Int32Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted Int32Array

publicsort( fn: (a:number, b:number) =>number): _Int32Array_
Sorts in-place
**Returns:** sorted Int32Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Int32Array_
**2.16. Int32Array 201**


Creates a Int32Array with the same underlying ArrayBuffer
**Returns:** new Int32Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive
publicsubarray(begin:number): _Int32Array_
Creates a Int32Array with the same ArrayBuffer
**Returns:** new Int32Array with the same ArrayBuffer
**Arguments:**
- begin:numberstart index, inclusive
publicsubarray(): _Int32Array_
Creates a Int32Array with the same ArrayBuffer
**Returns:** new Int32Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Int32Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_
publictoLocaleString(locales: _object_ ): _string_
Converts Int32Array to a string with respect to locale
**202 Chapter 2. Classes**


**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Int32Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Int32Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Int32Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Int32Array
**Returns:** a string representation of the Int32Array

publicvalues(): _IterableIterator_ <number>
**2.16. Int32Array 203**


Returns array values iterator
**Returns:** an iterator

publicwith(index:number, value:number): _Int32Array_
Creates a copy with replaced value on index
**Returns:** an Int32Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.16.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- • byteLength:byteOffset:numbernumber
- length:number
**2.17 Int8Array**
export
JS Int8Array API-compatible class

**2.17.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

**204 Chapter 2. Classes**


- index:numberindex to look at
publicconstructor():void
Creates an empty Int8Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Int8Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type byte in newly created Int8Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Int8Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Int8Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Int8Array_ ):void
Creates a copy of Int8Array.

**2.17. Int8Array 205**


**Arguments:**

- other: _Int8Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Int8Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Int8Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Int8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

**206 Chapter 2. Classes**


- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Int8Array_ ) =>boolean):boolean
Checks that all elements of Int8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Int8Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Int8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Int8Array_
Fills the Int8Array with specified value
**Returns:** modified Int8Array
**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Int8Array_
Fills the Int8Array with specified value
**Returns:** modified Int8Array

**2.17. Int8Array 207**


**Arguments:**

- value:numbernew valuy
- start:number
publicfill(value:number): _Int8Array_
Fills the Int8Array with specified value
**Returns:** modified Int8Array
**Arguments:**
- value:numbernew valuy

publicfilter( fn: (val:number) =>boolean): _Int8Array_
creates a new Int8Array from current Int8Array based on a condition fn
**Returns:** a new Int8Array with elements from current Int8Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Int8Array_ ) =>boolean): _Int8Array_
Creates a new Int8Array from current Int8Array based on a condition fn.
**Returns:** a new Int8Array with elements from current Int8Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Int8Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Int8Array_
creates a new Int8Array from current Int8Array based on a condition fn
**Returns:** a new Int8Array with elements from current Int8Array that satisfy condition fn

**208 Chapter 2. Classes**


**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Int8Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Int8Array_ ) =>boolean):number
Finds the first element in the Int8Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return byte | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Int8Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Int8Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Int8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:
2.17. Int8Array 209**


- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Int8Array_ ) =>boolean):number
Finds an index of the first element in the Int8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Int8Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Int8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Int8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Int8Array_ ) =>boolean):number
Finds the last element in the Int8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Int8Array_ ) =>booleancondition
**210 Chapter 2. Classes**


publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Int8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Int8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Int8Array_ ) =>boolean):number
Finds an index of the last element in the Int8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Int8Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Int8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**2.17. Int8Array 211**


publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Int8Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply
publicforEach( fn: (val:number, index:number, array: _Int8Array_ ) =>number):void
Applies a function over all elements of Int8Array
**Returns:** undefined
**Arguments:**
- fn: (val:number, index:number, array: _Int8Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Int8Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply
publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Int8Array_
Creates an Int8Array from array-like argument
**Returns:** new Int8Array
**Arguments:**
- o: _object_ array-like object to initialize Int8Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each
**212 Chapter 2. Classes**


publicfrom(o: _object_ ): _Int8Array_
Creates an Int8Array from array-like argument
**Returns:** new Int8Array
**Arguments:**

- o: _object_ array-like object to initialize Int8Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Int8Array_
Creates an Int8Array from array-like argument
**Returns:** new Int8Array
**Arguments:**

- o: _object_ array-like object to initialize Int8Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Int8Array
**Returns:** true if e is in Int8Array, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Int8Array
**Returns:** true if e is in Int8Array, false otherwise
**Arguments:**

**2.17. Int8Array 213**


- e:numbersearch element
publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
- fromIndex:numberstart index to search from

publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**

- e:numbersearch element

publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**

- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

**214 Chapter 2. Classes**


publickeys(): _IterableIterator_ <number>
Returns keys of the Int8Array
**Returns:** iterator over keys

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search

publicmap( fn: (val:number) =>number): _Int8Array_
Creates a new Int8Array using fn(arr[i]) over all elements of current Int8Array
**Returns:** a new Int8Array where for each element from current Int8Array fn was applied
**Arguments:**

- fn: (val:number) =>numbera function to apply for each element of current Int8Array

**2.17. Int8Array 215**


publicmap( fn: (val:number, index:number) =>number): _Int8Array_
Creates a new Int8Array using fn(arr[i]) over all elements of current Int8Array.
**Returns:** a new Int8Array where for each element from current Int8Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Int8Array

publicof(data:number[]): _Int8Array_
Creates a new Int8Array using initializer
**Returns:** a new Int8Array from data
**Arguments:**

- data:number[] initializer

publicnumberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Int8Array_ ) =>number, init:number):

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- • fn: (acc:init:numbernumberinitial value, curVal:number, curIndex:number, array: _Int8Array_ ) =>numbercondition

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Int8Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

**216 Chapter 2. Classes**


- fn: (acc:number, curVal:number, curIndex:number, array: _Int8Array_ ) =>numbercondition

publicnumber):reduceRight( fn: (acc:number number, curVal: number, curIndex: number, array: _Int8Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Int8Array_ ) =>numbercondition
- init:numberinitial value
publicreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Int8Array_ ) =>number):number
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Int8Array_ ) =>numbercondition

publicreverse(): _Int8Array_
Creates a new Int8Array using reversed data from the current one
**Returns:** a new Int8Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(byte[], insertPos)
**Arguments:**

- insertPos:numberindex to change
**2.17. Int8Array 217**


- val:numbervalue to set
publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Int8Array starting from insertPos.
**Arguments:**
- arr:number[] array to copy data from
- insertPos1:number
publicset(arr:number[]):void
Copies all elements of arr to the current Int8Array.
**Arguments:**
- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Int8Array_
Creates a slice of current Int8Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Int8Array with elements of current Int8Array[begin;end) where end index is excluded

**Arguments:**

- • begin:end:numbernumberlast index to be taken into slicestart index to be taken into slice

publicslice(begin:number): _Int8Array_
Creates a slice of current Int8Array using range [begin, this.length).
**Returns:** a new Int8Array with elements of current Int8Array[begin, this.length)
**Arguments:**

- begin:numberstart index to be taken into slice

**218 Chapter 2. Classes**


publicslice(): _Int8Array_
Creates a slice of current Int8 with all elements.
**Returns:** a new Int8Array with elements of current Int8Array

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Int8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicsome( fn: (element:number, index:number, array: _Int8Array_ ) =>boolean):boolean
Checks that at least one element of Int8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Int8Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Int8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Int8Array_
**2.17. Int8Array 219**


Sorts in-place according to the numeric ordering
**Returns:** sorted Int8Array

publicsort( fn: (a:number, b:number) =>number): _Int8Array_
Sorts in-place
**Returns:** sorted Int8Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Int8Array_
Creates a Int8Array with the same underlying ArrayBuffer
**Returns:** new Int8Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive

publicsubarray(begin:number): _Int8Array_
Creates a Int8Array with the same ArrayBuffer
**Returns:** new Int8Array with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

publicsubarray(): _Int8Array_
Creates a Int8Array with the same ArrayBuffer

**220 Chapter 2. Classes**


**Returns:** new Int8Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Int8Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts Int8Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Int8Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Int8Array_
Creates a reversed copy
**Returns:** a reversed copy

**2.17. Int8Array 221**


publictoSorted(): _Int8Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Int8Array
**Returns:** a string representation of the Int8Array

publicvalues(): _IterableIterator_ <number>
Returns array values iterator
**Returns:** an iterator

publicwith(index:number, value:number): _Int8Array_
Creates a copy with replaced value on index
**Returns:** an Int8Array with replaced value on index
**Arguments:**

- index:number
- value:number

**222 Chapter 2. Classes**


**2.17.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
- length:number
**2.18 IterableIterator<T>**
export extends _Iterator_ <T>
IterableIterator

**2.18.1 Methods**
publicnext(args: _object_ []): _IteratorResult_
next
**Arguments:** args: _object_ []

**Returns** IteratorResult

publicreturn(value: _object_ ): _IteratorResult_
return
**Arguments:** value: _object_

**Returns** IteratorResult

publicthrow(value: _object_ ): _IteratorResult_
throw
**2.18. IterableIterator<T> 223**


**Arguments:** value: _object_

**Returns** IteratorResult

**2.19 Iterator<T, TReturn, TNext>**
export
Iterator

**2.19.1 Methods**
publicnext(args: _object_ []): _IteratorResult_
next
**Arguments:** args: _object_ []

**Returns** IteratorResult

publicreturn(value: _object_ ): _IteratorResult_
return
**Arguments:** value: _object_

**Returns** IteratorResult

publicthrow(value: _object_ ): _IteratorResult_

**224 Chapter 2. Classes**


throw
**Arguments:** value: _object_

**Returns** IteratorResult

**2.20 IteratorResult<V>**
export
IteratorResult

**2.20.1 Properties**

- done:boolean|undefined
- value: _object_

**2.21 JSON**
export
JSON class

**2.21.1 Methods**
public staticstringify(d:number): _string_
Converts byte to JSON format
**Returns:** string - JSON representation of byte
**Arguments:**

- d:number:number- byte to be converted to a JSON as a number

public staticstringify(d:number): _string_
**2.20. IteratorResult<V> 225**


Converts short to JSON format
**Returns:** string - JSON representation of short
**Arguments:**

- d:number:number- short to be converted to a JSON as a number
public staticstringify(d:number): _string_
Converts int to JSON format
**Returns:** string - JSON representation of int
**Arguments:**
- d:number:number- int to be converted to a JSON as a number

public staticstringify(d:number): _string_
Converts long to JSON format
**Returns:** string - JSON representation of long
**Arguments:**

- d:number:number- long to be converted to a JSON as a number

public staticstringify(d:number): _string_
Converts float to JSON format
**Returns:** string - JSON representation of float
**Arguments:**

- d:number:number- float to be converted to a JSON as a number
public staticstringify(d:number): _string_
Converts number to JSON format
**226 Chapter 2. Classes**


**Returns:** string - JSON representation of number
**Arguments:**

- d:number:number- number to be converted to a JSON as a number

public staticstringify(d:char): _string_
Converts char to JSON format
**Returns:** string - JSON representation of char
**Arguments:**

- d:char: char - char to be converted to a JSON as a string

public staticstringify(d:boolean): _string_
Converts boolean to JSON format
**Returns:** string - JSON representation of boolean
**Arguments:**

- d:boolean: boolean - boolean to be converted to a JSON as a Boolean literal
public staticstringify(d: _string_ ): _string_
Converts string to JSON format
**Returns:** string - JSON representation of byte
**Arguments:**
- d: _string_ : string - byte to be converted to a JSON as a string

public staticstringify(d: _object_ ): _string_
Converts object to JSON format

**2.21. JSON 227**


**Returns:** string - JSON representation of object
**Arguments:**

- d: _object_ : object - byte to be converted to a JSON as an object

public staticstringify(d:number[]): _string_
Converts bytes array to JSON format
**Returns:** string - JSON representation of bytes array
**Arguments:**

- d:number[] :number[] - bytes array to be converted to a JSON as an Array of numbers

public staticstringify(d:number[]): _string_
Converts shorts array to JSON format
**Returns:** string - JSON representation of shorts array
**Arguments:**

- d:number[] :number[] - shorts array to be converted to a JSON as an Array of numbers

public staticstringify(d:number[]): _string_
Converts ints array to JSON format
**Returns:** string - JSON representation of ints array
**Arguments:**

- d:number[] :number[] - ints array to be converted to a JSON as an Array of numbers

public staticstringify(d:number[]): _string_
Converts longs array to JSON format
**Returns:** string - JSON representation of longs array
**228 Chapter 2. Classes**


**Arguments:**

- d:number[] :number[] - longs array to be converted to a JSON as an Array of numbers

public staticstringify(d:number[]): _string_
Converts array of bytes to JSON format
**Returns:** string - JSON representation of array of bytes
**Arguments:**

- d:number[] :number[] - array of byte to be converted to a JSON as an Array of numbers

public staticstringify(d:number[]): _string_
Converts numbers array to JSON format
**Returns:** string - JSON representation of numbers array
**Arguments:**

- d:number[] :number[] - numbers array to be converted to a JSON as an Array of numbers

public staticstringify(d:char[]): _string_
Converts chars array to JSON format
**Returns:** string - JSON representation of chars array
**Arguments:**

- d:char[] : char[] - chars array to be converted to a JSON as an Array of numbers

public staticstringify(d:boolean[]): _string_
Converts booleans array to JSON format
**Returns:** string - JSON representation of booleans array

**2.21. JSON 229**


**Arguments:**

- d:boolean[] : boolean[] - booleans array to be converted to a JSON as an Array of Boolean literals

**2.22 Map<K, V>**
export
**Class:** Map

**2.22.1 Methods**
publicclear():void
Deletes all elements from the Map

publicconstructor():void
Constructs an empty Map

publicdelete(k: K):void
Removes an Entry with specified key from the Map
**Arguments:**

- k: K the key to remove

publicentries(): _IterableIterator_
Returns elements from the Map as an array of Entries. TODO: return type is incorrect
**Returns:** an array of Entries

**230 Chapter 2. Classes**


publicforEach( fn: (v: V, k: K) =>void):void
Applies a function over all elements of the Map
**Arguments:**

- fn: (v: V, k: K) =>voidto apply

publicforEach( fn: (v: V) =>void):void
Applies a function over all elements of the Map
**Arguments:**

- fn: (v: V) =>voidto apply

publicget(k: K): V | undefined
Returns a value assosiated with key if present
**Returns:** value if assosiated with key presents.
**Arguments:**

- k: K the key to find in the Map

publichas(k: K):boolean
Checks if a key is in the Map
**Returns:** true if the value is in the Map
**Arguments:**

- k: K the key to find in the Map
publickeys(): _IterableIterator_ <K>
Returns elements from the Map as an keys Iterator. TODO: return type is incorrect
**2.22. Map<K, V> 231**


**Returns:** ValueIterator with map keys

publicset(k: K, v: V):void
Puts a pair of key and value into the Map
**Arguments:**

- k: K the key to put into the Map
- v: V the value to put into the Map

publicvalues(): _IterableIterator_ <V>
Returns elements from the Map as an values Iterator. TODO: return type is incorrect
**Returns:** ValueIterator with map values

**2.22.2 Properties**

- size:number
**2.23 Math**
export
**Class:** The Math class contains static properties and methods for mathematical constants and functions.

**2.23.1 Methods**
public staticasin(x:number):number
Arcsine of angle`v`
**Returns:** Arcsine of angle`v`

**232 Chapter 2. Classes**


**Arguments:**

- x:number
public staticatan2(y:number, x:number):number
Method returns the angle in the plane (in radians) between the positive x-axis and the ray from (0, 0) to the point (x,y), for Math.atan2(y, x).

**Returns:** point (x, y).The angle in radians (between - and , inclusive) between the positive x-axis and the ray from (0, 0) to the

**Remark:** point (x, y). Note that the arguments to this function pass the y-coordinate first and the x-coordinate second.The atan2() method measures the counterclockwise angle , in radians, between the positive x-axis and the

public staticcbrt(x:number):number
Cube root of a number.
**Arguments:**

- x:numberarbitrary number
**Remark:** Math.brt() = x = the unique y such that y^3 = x.

public staticfround(x:number):number
"fround" returns nearest 32-bit single fp representation of a number in a 64-bit formMath.fround(1.337) == 1.337 // false, result would be 1.3370000123977661 Math.fround(1.5) == 1.5 // true
Math.fround(-5.05) == -5.05 //false, result would be -5.050000190734863 Math.fround(Infinity) // InfinityMath.fround(NaN) // NaN

public statichypot(u:number, v:number):number
Square root of the sum of squares of`v`and`u`
**2.23. Math 233**


**Returns:** The square root of the sum of squares of its arguments
**Arguments:**

- u:numberarbitrary number
- v:numberarbitrary number

public staticimul(a:number, b:number):number
Method returns the result of the C-like 32-bit manipulation of the two parameters
**Returns:** Math.imul(Infinity, 1) = 0 Math.imul(NaN, 1) = 0 Math.imul(2.5, 2.5) = 4 Math.imul(-5, 5.05) = 25(a * b) % 2 ** 32

public staticmax(u:number, v:number):number
Largest value of`u`and`v`
**Returns:** Largest value of`u`and`v`
**Arguments:**

- u:numberarbitrary number
- v:numberarbitrary number

public staticmin(u:number, v:number):number
Smallest value of`u`and`v`
**Returns:** Smallest value of`u`and`v`
**Arguments:**

- u:numberarbitrary number
- v:numberarbitrary number

public staticrandom():number
**234 Chapter 2. Classes**


Pseudo-random number in the range [0.0, 1.0)
**Returns:** approximately uniform distribution over that range — which you can then scale to your desired range. Initial seed toA floating-point, pseudo-random number that’s greater than or equal to 0 and less than 1, with
the random number generator algorithm can be given usingseedRandom()function.

public staticsign(x:number):number
**Returns:** -1 if`x`is negative, 1 if`x`is positive, 0 if`x`is close to zero (epsilon is 1e-13)
**Arguments:**

- x:numberarbitrary number
public statictrunc(x:number):number
Integer part of`v`
**Returns:** The integer part of a number by removing any fractional digits.
**Arguments:**
- x:number
**Notes:** NaN, NaN is returnedIf arg is +Infinity or -Infinity, it is returned unmodified. If arg is +0 or -0, it is returned unmodified. If arg is

**2.23.2 Properties**

- static E:number
- static LN10:number
- • static LN2:static LOG10E:numbernumber
- static LOG2E:number
- static PI:number
- static SQRT1_2:number
- static SQRT2:number
**2.23. Math 235**


**2.24 Number**
export
Represents boxed number value and related operations

**2.24.1 Methods**
publicconstructor():void
Constructs a new number instance with initial value zero

publicconstructor(value:number):void
Constructs a new number instance with provided initial value
**Arguments:**

- value:number— the initial value

publicconstructor(value:number):void
Constructs a new number instance with provided initial value
**Arguments:**

- value:number— the initial value

publicisFinite():boolean
isFinite() checks if the underlying number is a floating point value (not a NaN or infinity)
**Returns:** true if the underlying number is a floating point value

publicisInteger():boolean

**236 Chapter 2. Classes**


Checks if the underlying number is similar to an integer value
**Returns:** true if the underlying number is similar to an integer value

publicisNaN():boolean
isNaN() checks if the underlying number is NaN (not a number)
**Returns:** true if the underlying number is NaN

publicisSafeInteger():boolean
Checks if number is a safe integer value
**Returns:** true if the underlying number is a safe integer

publictoExponential(d:number): _string_
toExponential(number) returns string representing the underlying number in exponential notation
**Returns:** the result of conversion
**Arguments:**

- d:number— the exponent (rounded to nearest integer); must be in [0, 100]
**Note:** "2.50e-1" If d = new number(0.25); d.toExponential(1) -> "2.5e-1" If d = new number(12345.01);If d = new number(0.25); d.toExponential(2) -> "2.50e-1" If d = new number(0.25); d.toExponential(2.5) ->
d.toExponential(10) -> "1.2345010000e+4" If d = new number(NaN); d.toExponential(10) -> "NaN"; If d = newnumber(number.POSITIVE_INFINITY); d.toExponential(10) -> "Infinity"; "-Infinity" for negative

**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#673).`toExponential()`intrinsic [declaration](https://gitee.com/openharmony-
ECMA reference: https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.toexponential
**2.24. Number 237**


publictoExponential(): _string_
toExponential() returns string representing the underlying number in exponential notation
**Returns:** the result of conversion

publictoFixed(d:number): _string_
toFixed(number) returns string representing the underlying number using fixed-point notation
**Returns:** the result of conversion
**Arguments:**

- d:number— fixed size (integer part); must be in [0, 100]
**Note:** number(0.12345); d.toFixed(1) -> "0.1" If d = new number(0.12345); d.toFixed(3) -> "0.123" If d = newIf d = new number(0.1); d.toFixed(0) -> "0" If d = new number(0.7); d.toFixed(0) -> "1" If d = new
number(number.POSITIVE_INFINITY); d.toFixed(3) -> "Infinity" If d = new number(number.NaN); d.toFixed(3) ->"NaN" If d = new number(0.25); d.toFixed(200) -> thrown ArgumentOutOfRangeException

**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#693).`toFixed()`intrinsic [declaration](https://gitee.com/openharmony-
ECMA reference: https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.tofixed

publictoFixed(): _string_
toFixed(number) returns string representing the underlying number using fixed-point notation
**Returns:** the result of conversion

**238 Chapter 2. Classes**


publictoLocaleString(locale: _string_ )
Accepts a locale and returns string in language-sensitive representation
**Returns:** result of the conversion in a local representation
**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#741).`toLocaleString()`intrinsic [declaration](https://gitee.com/openharmony-

publictoLocaleString(locale: _string_ ): _string_
Accepts a locale and returns string in language-sensitive representation
**Returns:** result of the conversion in a local representation
**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#741).`toLocaleString()`intrinsic [declaration](https://gitee.com/openharmony-

publictoLocaleString(): _string_
Without an argument method returns just toString value
**Returns:** result of the conversion

publictoPrecision(d:number): _string_
toPrecision(number) returns string representing the underlying number on the specified precision
**Returns:** the result of conversion

**2.24. Number 239**


**Arguments:**

- d:number— precision (rounded to nearest integer); must be in [1, 100]
**Note:** If d = new number(0.25); d.toPrecision(0) -> thrown ArgumentOutOfRangeException If d = newIf d = new number(0.25); d.toPrecision(4) -> "0.2500" If d = new number(1.01); d.toPrecision(4.7) -> "1.010"
number(12345.123455); d.toPrecision(10) -> "12345.12346"
**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#683).`toPrecision()`intrinsic [declaration](https://gitee.com/openharmony-
ECMA reference: https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.prototype.toprecision

publictoPrecision(): _string_
toPrecision() returns string representing the underlying number in exponential notation basically, if toPrecision calledwith no argument it’s just toString according to spec

**Returns:** the result of conversion

public overridetoString(): _string_
Converts this object to a string
**Returns:** result of the conversion

publicvalueOf():number
Returns the value of this number
**Returns:** the value of this number

public staticisFinite(v:number):boolean
**240 Chapter 2. Classes**


isFinite(number) checks if number is a floating point value (not a NaN or infinity)
**Returns:** true if the argument is a floating point value
**Arguments:**

- v:number— the number to test
public staticisInteger(v:number):boolean
Checks if number is similar to an integer value
**Returns:** true if the argument is similar to an integer value
**Arguments:**
- v:number— the number to test
public staticisNaN(v:number):boolean
isNaN(number) checks if number is NaN (not a number)
**Returns:** true if the argument is NaN
**Arguments:**
- v:number— the number to test

public staticisSafeInteger(v:number):boolean
Checks if number is a safe integer value
**Returns:** true if the argument is integer ans less than MAX_SAFE_INTEGER
**Arguments:**

- v:number— the number to test

public staticparseFloat(s: _string_ ):number
parseFloat(string) converts string to number
**2.24. Number 241**


**Returns:** the result of conversion
**Arguments:**

- s: _string_ — the string to convert
**Note:** return value is 0 or -0. If arg has leading zeroes, it’s ignored: "0001.5" -> 1.5, "-0001.5" -> -1.5 If arg starts from ".",If arg is "+Infinity", "Infinity" or "-Infinity", return value is`inf`or`-inf`respectively. If arg is "+0" or "-0",
leading zero is implied: ".5" -> 0.5, "-.5" -> -0.5 If arg successfully parsed, trailing non-digits ignored: "-.6ffg" ->-0.6 If arg can not be parsed into a number, NaN is returned

**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#653).`parseFloat()`intrinsic [declaration](https://gitee.com/openharmony-
ECMA reference: https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.parsefloat

public staticparseInt(s: _string_ , r:number):number
parseInt(string, int) parses from string an integer of specified radix
**Returns:** the result of parsing
**Arguments:**

- s: _string_ — the string to convert
- r:number— the radix of conversion; should be [2, 36]; 0 assumed to be 10
**Note:** ArgumentOutOfRangeException If args ("10", 2) -> 2 If args ("10", 10) -> 10, ("10", 0) -> 10 If args ("ff", 16) -> 255If args ("10", 1) -> thrown ArgumentOutOfRangeException, ("10", 37) -> thrown
etc.
**Remark:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#663).`parseInt()`intrinsic [declaration](https://gitee.com/openharmony-
ECMA reference: https://tc39.es/ecma262/multipage/numbers-and-dates.html#sec-number.parseint

public staticparseInt(s: _string_ ):number
**242 Chapter 2. Classes**


parseInt(string) parses from string an integer of radix 10
**Returns:** the result of parsing
**Arguments:**

- s: _string_ — the string to convert

**2.25 Object**
export
Common ancestor amongst all other classes

**2.25.1 Methods**
publicconstructor():void
Constructs a new blank object

publictoString(): _string_
Converts this object to a string
**Returns:** result of the conversion

**2.26 Promise<T>**
export
Class represents a result of an asynchronous operation in the future.

**2.25. Object 243**


**2.26.1 Methods**
publiccatch<U>( onRejected: (error: _object_ |null) => U ): _Promise_ <U>

publicconstructor( callback: (resolve: (value: T) =>void) =>void):void

publicconstructor( callback: (resolve: (value: T) =>void, reject: (error: _object_ ) =>void) =>void):void

publicfinally( onFinally: () =>void): _Promise_ <T>

publicthen<U>( onFulfilled: () => U ): _Promise_ <U>

publicthen<U>( onFulfilled: (value: T) => U ): _Promise_ <U>

publicthen<U>( onFulfilled: (value: T) => U, onRejected: (error: _object_ |null) => U ): _Promise_ <U>

public staticresolve<U>(value: U): _Promise_ <U>

**2.27 RangeError**
export extends _Error_
**Class:** Represents an error that occurs when provided collection index is out of range

**2.27.1 Methods**
publicconstructor():void
Constructs a new instance of error

publicconstructor(s: _string_ ):void
Constructs a new instance of error
**Arguments:
244 Chapter 2. Classes**


- s: _string_
publicconstructor(s: _string_ , cause: _object_ ):void
Constructs a new instance of error
**Arguments:**
- • s:cause: _stringobject_

**2.27.2 Properties**

- cause: _object_
- message: _string_
- • name:stack: _stringstring_

**2.28 ReferenceError**
export extends _Error_
**Class:** scope is referencedRepresents an error that occurs when a variable that doesn’t exist (or hasn’t yet been initialized) in the current

**2.28.1 Methods**
publicconstructor():void
Constructs a new instance of error

publicconstructor(s: _string_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_
**2.28. ReferenceError 245**


publicconstructor(s: _string_ , cause: _object_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_
- cause: _object_

**2.28.2 Properties**

- cause: _object_
- • message:name: _stringstring_
- stack: _string_
**2.29 RegExp**
export extends _object_
**Class:** Regular expression

**2.29.1 Methods**
publiccompile(pattern: _string_ , flags: _string_ ):void
Recompiles a regular expression with new source and flags after the RegExp object has already been created
**Arguments:**

- pattern: _string_ the text of the regular expression
- flags: _string_ any combination of flag values

publicconstructor(pattern: _string_ ):void
Constructs a new RegExp using pattern
**Arguments:
246 Chapter 2. Classes**


- pattern: _string_ description of a pattern

publicconstructor(pattern: _string_ , flags: _string_ ):void
Constructs a new RegExp using pattern and flags
**Arguments:**

- pattern: _string_ description of a pattern
- flags: _string_ description of flags to be used

publicexec(str: _string_ ): _RegExpExecArray_
Executes a search for a match in a specified string and returns a result array
**Returns:** RegExp result
**Arguments:**

- str: _string_ the string against which to match the regular expression
**See:** RegExpExecArray

publictest(str: _string_ ):boolean
Executes a search for a match between a regular expression and specified string
**Returns:** true if match was found
**Arguments:**

- str: _string_ the string against which to match the regular expression

public overridetoString(): _string_
Returns a string representing the given object
**Returns:** a string representing the given object
**2.29. RegExp 247**


public staticadvancestringIndex( s: _string_ , index:number, unicode:boolean):number
Returns next index from a passed one
**Returns:** new index
**Arguments:**

- s: _string_
- index:numberstart position
- unicode:booleantrue if unicode is used

**2.29.2 Properties**

- dotAll:boolean
- flags: _string_
- global:boolean
- hasIndices:boolean
- ignoreCase:boolean
- lastIndex:number
- multiline:boolean
- source: _string_
- sticky:boolean
- unicode:boolean
**2.30 RegExpExecArray**
TODO: align with TS spec
export extends _object_
**Class:** Regular expression result descriptor

**248 Chapter 2. Classes**


**2.30.1 Methods**
publicconstructor( isCorrect:boolean, index:number, input: _string_ , result: _string_ [] ):void
Creates a RegExpExecArray
**Arguments:**

- isCorrect:boolean
- • index:input: _string_ number
- result: _string_ []

public overrideequals(other: _object_ |null):boolean
Creates a RegExpExecArray
**Arguments:**

- other: _object_ |null

publicget(index:number): _string_
Returns result string by index
**Returns:** resulting string
**Arguments:**

- index:number
publicget(index:number): _string_
Returns result string by index
**Returns:** resulting string
**Arguments:**
- index:number

**2.30. RegExpExecArray 249**


**2.31 Set<K>**
export
**Class:** Set implementation via tree

**2.31.1 Methods**
publicadd(v: K):void
Puts a value into the Set
**Arguments:**

- v: K the value to put into the Set

publicclear():void
Deletes all elements from the Set

publicconstructor():void
Constructs an empty TreeSet

publicdelete(v: K):void
Removes a value from the Set
**Arguments:**

- v: K the value to remove
publicforEach( fn: (k: K) =>void):void
Applies a function over all elements of the Set
**250 Chapter 2. Classes**


**Arguments:**

- fn: (k: K) =>voidto apply

publichas(v: K):boolean
Checks if a value is in the Set
**Returns:** true if the value is in the Set
**Arguments:**

- v: K the value to find in the Set
publiclength():number
Returns number of unique elements in the Set
**Returns:** number of unique elements in the Set

publicvalues(): _IterableIterator_ <K>
Returns elements from the Set as an array
**Returns:** an array of Set values

**2.31.2 Properties**

- size:number
**2.32 String**
export extends _object_
Unicode string

**2.32. String 251**


**2.32.1 Methods**
publiccharAt(index:number): _string_
Getter for char at some index
**Returns:** throws stringIndexOutOfBoundsException if index is negative or >= lengthchar value at index

**Arguments:**

- index:number— index in char array inside string
**Remarks:** Implemented as native function,
**See:** sig/arkcompiler_runtime_core/blob/master/plugins/ets/runtime/ets_libbase_runtime.yaml#L585).`charAt()`intrinsic [declaration](https://gitee.com/openharmony-

publiccharCodeAt(index:number):number
The charCodeAt() method returns an integer between 0 and 65535 representing the UTF-16 code unit at the givenindex.

publicconcat(to: _string_ ): _string_
Concatenation of this and another string.
**Returns:** new string which is a concatenation of this + to
**Arguments:**

- to: _string_ — string to concat with throws NullPointerException if to param is null

publicconstructor():void
Constructs an empty string

**252 Chapter 2. Classes**


publicconstructor(data:char[]):void
Constructs string from chars array initializer
**Arguments:**

- data:char[] — initializer

publicconstructor(otherStr: _string_ ):void
Constructs string from another string
**Arguments:**

- otherStr: _string_ — initializer

publicindexOf(str: _string_ , fromIndex:number):number
Finds the first occurrence of another string in this string. The search starts from the specified index.
**Returns:** index of the str from the beginning of this string, or -1 if not found
**Arguments:**

- str: _string_ — to find
- fromIndex:dexOutOfBoundsException if fromIndex param is negative or >= length throws AssertionError if length of str isnumber— to start searching from throws NullPointerException if str param is null throws stringIn-
    greater than the length of this string

publiclastIndexOf(str: _string_ , fromIndex:number):number
Finds the last occurrence of another string in this string. The search starts from the specified index.
**Returns:** index of the str from the beginning of this string, or -1 if not found
**Arguments:**

- str: _string_ — to find

**2.32. String 253**


- fromIndex:dexOutOfBoundsException if fromIndex param is negative or >= length throws AssertionError if length of str innumber— to start searching from throws NullPointerException if str param is null throws stringIn-
    sum of specified index is greater than the length of this string

publiclocaleCompare(another: _string_ , locale: _string_ |null):number
Comparison between this string and another one based on locale. The result is -1 if this string sorts before the anotherstring, 0 if they are equal, and 1 otherwise.

**Returns:** the comparison result
**Arguments:**

- another: _string_ — string to compare with
- locale:or not found throws NullPointerException if another or locale is null _string_ |null— string representing the BCP47 language tag throws RangeError if the locale tag is invalid

publiclocaleCompare(another: _string_ ):number
Comparison between this string and another one based on default host locale. The result is -1 if this string sortsbefore the another string, 0 if they are equal, and 1 otherwise.

**Returns:** the comparison result
**Arguments:**

- another:NullPointerException if another param is null _string_ — string to compare with throws RangeError if the locale tag is invalid or not found throws

publicmatch(regexp: _string_ ): _string_ []
regexp match
**Arguments:**

- regexp: _string_

publicmatch(regexp: _RegExp_ ): _string_ []
Retrieves the result of matching a string against a regular expression

**254 Chapter 2. Classes**


**Returns:** capturing groups are not included Otherwise, only the first complete match and its related capturing groups areIf the regexp.global is true, all results matching the complete regular expression will be returned, but
returned
**Arguments:**

- regexp: _RegExp_ — a regular expression object

publicreplace(w1: _string_ , w2: _string_ ): _string_
replace
**Returns:** new string
**Arguments:**

- w1: _string_ - w2: _string_

publicreplace(pattern: _RegExp_ , replacement: _string_ ): _string_
Returns a new string with one, some, or all matches of a pattern replaced by a replacement. The pattern can be astring or a RegExp, and the replacement can be a string or a function called for each match. If pattern is a string, only
the first occurrence will be replaced. The original string is left unchanged.
**Returns:** new string
**Arguments:**

- pattern: _RegExp_ - replacement: _string_

publicsearch(reg: _string_ ):number
search
**Returns:** int
**Arguments:**

- reg: _string_

publicsearch(regexp: _RegExp_ ):number

**2.32. String 255**


Executes a search for a match between a regular expression and this string object.
**Returns:** the index of the first match between the regular expression and the given string, or -1 if no match was found.
**Arguments:**

- regexp: _RegExp_ — a regular expression object

publicslice(begin:number): _string_
The slice() method extracts a section of a string and returns it as a new string, without modifying the original string.

publicsplit(pattern: _string_ , limit:number): _string_ []
Splits this string by pattern and returns ordered array of substrings. The order of the resulted array corresponds to theorder of the passage of this string from beginning to end. The pattern is excluded from substrings. The array is
limited by some specified value.
**Returns:** string array contains substrings from this string
**Arguments:**

- pattern: _string_ — string to split by
- limit:ception if pattern param is nullnumber— max length of the returned array. If it’s negative then there is no limit. throws NullPointerEx-

publicsubstr(begin:number): _string_
The substr() method returns a portion of the string, starting at the specified index and extending for a given number ofcharacters afterwards.

publicsubstr(begin:number, length:number): _string_
The substr() method returns a portion of the string, starting at the specified index and extending for a given number ofcharacters afterwards.

**256 Chapter 2. Classes**


publicsubstring(begin:number): _string_
Selects a substring of this string, starting at a specified index and ending at the end of this string.
**Returns:** new string which is a substring of this string
**Arguments:**

- begin:lengthnumber— to start substring throws stringIndexOutOfBoundsException if begin param is negative or >=

publictoLocaleLowerCase(locale: _string_ ): _string_
The toLocaleLowerCase() method returns the calling string value converted to lower case, according to anylocale-specific case mappings.

publictoLocaleLowerCase(): _string_
The toLocaleLowerCase() method returns the calling string value converted to lower case, according to anylocale-specific case mappings.

publictoLocaleUpperCase(locale: _string_ ): _string_
The toLocaleUpperCase() method returns the calling string value converted to upper case, according to anylocale-specific case mappings.

publictoLocaleUpperCase(): _string_
The toLocaleUpperCase() method returns the calling string value converted to upper case, according to anylocale-specific case mappings.

**2.32. String 257**


publictoLowerCase(): _string_
Creates new string similar to this string but with all characters in lower case.
**Returns:** new string with all characters in lower case

public overridetoString(): _string_
Theobject.`toString()`method returns the string representation of the given string in the form of a copy of the original

**Returns:** a copy of the original string

publictoUpperCase(): _string_
Creates new string similar to this string but with all characters in upper case.
**Returns:** new string with all characters in upper case

publicvalueOf(): _string_
The valueOf() method returns the primitive value of a string object.

**2.32.2 Properties**

- length:number
**2.33 SyntaxError**
export extends _Error_
**Class:** Represents an error that occurs when trying to interpret syntactically invalid code

**258 Chapter 2. Classes**


**2.33.1 Methods**
publicconstructor():void
Constructs a new instance of error

publicconstructor(s: _string_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_

publicconstructor(s: _string_ , cause: _object_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_
- cause: _object_

**2.33.2 Properties**

- • cause:message: _objectstring_
- name: _string_
- stack: _string_
**2.34 URIError**
export extends _Error_
**Class:** Represents an error that occurs when a global URI handling function was used in a wrong way

**2.34. URIError 259**


**2.34.1 Methods**
publicconstructor():void
Constructs a new instance of error

publicconstructor(s: _string_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_

publicconstructor(s: _string_ , cause: _object_ ):void
Constructs a new instance of error
**Arguments:**

- s: _string_
- cause: _object_

**2.34.2 Properties**

- • cause:message: _objectstring_
- name: _string_
- stack: _string_
**2.35 Uint16Array**
export
JS Uint16Array API-compatible class

**260 Chapter 2. Classes**


**2.35.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at

publicconstructor():void
Creates an empty Uint16Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Uint16Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type number in newly created Uint16Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Uint16Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Uint16Array with respect to buf.
**2.35. Uint16Array 261**


**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Uint16Array_ ):void
Creates a copy of Uint16Array.
**Arguments:**

- other: _Uint16Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Uint16Array.
**Arguments:**

- • insertPos:startPos:numbernumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.insert index to place copied elements
    org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Uint16Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

**262 Chapter 2. Classes**


publicevery( fn: (element:number, index:number, array: _Uint16Array_ ) =>boolean):boolean
Checks that all elements of Uint16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint16Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Uint16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Uint16Array_ ) =>boolean):boolean
Checks that all elements of Uint16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint16Array_ ) =>booleancheck function

**2.35. Uint16Array 263**


publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint16Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Uint16Array_
Fills the Uint16Array with specified value
**Returns:** modified Uint16Array
**Arguments:**

- • value:start:numbernumbernew valuy
- end:number

publicfill(value:number, start:number): _Uint16Array_
Fills the Uint16Array with specified value
**Returns:** modified Uint16Array
**Arguments:**

- value:numbernew valuy
- start:number
publicfill(value:number): _Uint16Array_
Fills the Uint16Array with specified value
**Returns:** modified Uint16Array

**264 Chapter 2. Classes**


**Arguments:**

- value:numbernew valuy

publicfilter( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean): _Uint16Array_
Creates a new Uint16Array from current Uint16Array based on a condition fn.
**Returns:** a new Uint16Array with elements from current Uint16Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Uint16Array_
creates a new Uint16Array from current Uint16Array based on a condition fn
**Returns:** a new Uint16Array with elements from current Uint16Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number) =>boolean): _Uint16Array_
creates a new Uint16Array from current Uint16Array based on a condition fn
**Returns:** a new Uint16Array with elements from current Uint16Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean): _Uint16Array_
Creates a new Uint16Array from current Uint16Array based on a condition fn.
**Returns:** a new Uint16Array with elements from current Uint16Array that satisfy condition fn
**Arguments:
2.35. Uint16Array 265**


- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Uint16Array_
creates a new Uint16Array from current Uint16Array based on a condition fn
**Returns:** a new Uint16Array with elements from current Uint16Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds the first element in the Uint16Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint16Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Uint16Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition
**266 Chapter 2. Classes**


publicfind( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds the first element in the Uint16Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint16Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds an index of the first element in the Uint16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**2.35. Uint16Array 267**


publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Uint16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds an index of the first element in the Uint16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint16Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds the last element in the Uint16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleancondition

**268 Chapter 2. Classes**


publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Uint16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds the last element in the Uint16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint16Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**2.35. Uint16Array 269**


publicfindLastIndex( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds an index of the last element in the Uint16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Uint16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Uint16Array_ ) =>boolean):number
Finds an index of the last element in the Uint16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>booleancondition

**270 Chapter 2. Classes**


publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint16Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicforEach( fn: (val:number, index:number, array: _Uint16Array_ ) =>number):void
Applies a function over all elements of Uint16Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint16Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Uint16Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply

**2.35. Uint16Array 271**


publicforEach( fn: (val:number, index:number, array: _Uint16Array_ ) =>number):void
Applies a function over all elements of Uint16Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Uint16Array_ ) =>numberfunction to apply
publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint16Array
**Returns:** undefined
**Arguments:**
- fn: (val:number, index:number) =>numberfunction to apply

publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Uint16Array_
Creates an Uint16Array from array-like argument
**Returns:** new Uint16Array
**Arguments:**

- o: _object_ array-like object to initialize Uint16Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each
publicfrom(o: _object_ ): _Uint16Array_
Creates an Uint16Array from array-like argument
**Returns:** new Uint16Array
**Arguments:**
- o: _object_ array-like object to initialize Uint16Array
**272 Chapter 2. Classes**


publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint16Array_
Creates an Uint16Array from array-like argument
**Returns:** new Uint16Array
**Arguments:**

- o: _object_ array-like object to initialize Uint16Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint16Array_
Creates an Uint16Array from array-like argument
**Returns:** new Uint16Array
**Arguments:**

- o: _object_ array-like object to initialize Uint16Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Uint16Array
**Returns:** true if e is in Uint16Array, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from
publicincludes(e:number):boolean
Checks if specified argument is in Uint16Array
**Returns:** true if e is in Uint16Array, false otherwise

**2.35. Uint16Array 273**


**Arguments:**

- e:numbersearch element
publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
- fromIndex:numberstart index to search from
publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**
- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

**274 Chapter 2. Classes**


publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search
publicmap( fn: (val:number, index:number) =>number): _Uint16Array_
Creates a new Uint16Array using fn(arr[i]) over all elements of current Uint16Array.
**Returns:** a new Uint16Array where for each element from current Uint16Array fn was applied
**Arguments:**
- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint16Array

publicmap( fn: (val:number) =>number): _Uint16Array_
Creates a new Uint16Array using fn(arr[i]) over all elements of current Uint16Array
**Returns:** a new Uint16Array where for each element from current Uint16Array fn was applied
**Arguments:**

**2.35. Uint16Array 275**


- fn: (val:number) =>numbera function to apply for each element of current Uint16Array

publicmap( fn: (val:number, index:number) =>number): _Uint16Array_
Creates a new Uint16Array using fn(arr[i]) over all elements of current Uint16Array.
**Returns:** a new Uint16Array where for each element from current Uint16Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint16Array

publicof(data:number[]): _Uint16Array_
Creates a new Uint16Array using initializer
**Returns:** a new Uint16Array from data
**Arguments:**

- data:number[] initializer

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition
- init:numberinitial value
publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:
276 Chapter 2. Classes**


- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition
- init:numberinitial value

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition

publicnumberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- • fn: (acc:init:numbernumberinitial value, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition

publicnumberreduceRight( fn: (acc: number, curVal: number, curIndex: number, array: _Uint16Array_ ) =>number):

Reduces data into a single value using right-to-left traversal

**2.35. Uint16Array 277**


**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition

publicnumberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition
- init:numberinitial value
publicnumberreduceRight( fn: (acc: number, curVal: number, curIndex: number, array: _Uint16Array_ ) =>number):

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint16Array_ ) =>numbercondition

publicreverse(): _Uint16Array_
Creates a new Uint16Array using reversed data from the current one
**Returns:** a new Uint16Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.

**278 Chapter 2. Classes**


**Description:** Added to avoid (un)packing a single value into array to use overloaded set(number[], insertPos)
**Arguments:**

- insertPos:numberindex to change
- val:numbervalue to set

publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Uint16Array starting from insertPos.
**Arguments:**

- arr:number[] array to copy data from
- insertPos1:number
publicset(arr:number[]):void
Copies all elements of arr to the current Uint16Array.
**Arguments:**
- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Uint16Array_
Creates a slice of current Uint16Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Uint16Array with elements of current Uint16Array[begin;end) where end index is excluded

**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _Uint16Array_
Creates a slice of current Uint16Array using range [begin, this.length).

**2.35. Uint16Array 279**


**Returns:** a new Uint16Array with elements of current Uint16Array[begin, this.length)
**Arguments:**

- begin:numberstart index to be taken into slice

publicslice(): _Uint16Array_
Creates a slice of current Uint16 with all elements.
**Returns:** a new Uint16Array with elements of current Uint16Array

publicsome( fn: (element:number, index:number, array: _Uint16Array_ ) =>boolean):boolean
Checks that at least one element of Uint16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint16Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Uint16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:
280 Chapter 2. Classes**


- fn: (element:number) =>booleancheck function
publicsome( fn: (element:number, index:number, array: _Uint16Array_ ) =>boolean):boolean
Checks that at least one element of Uint16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**
- fn: (element:number, index:number, array: _Uint16Array_ ) =>booleancheck function
publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint16Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**
- fn: (element:number, index:number) =>booleancheck function
publicsort(): _Uint16Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted Uint16Array

publicsort( fn: (a:number, b:number) =>number): _Uint16Array_
Sorts in-place
**Returns:** sorted Uint16Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator
publicsubarray(begin:number, end:number): _Uint16Array_
**2.35. Uint16Array 281**


Creates a Uint16Array with the same underlying ArrayBuffer
**Returns:** new Uint16Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive
publicsubarray(begin:number): _Uint16Array_
Creates a Uint16Array with the same ArrayBuffer
**Returns:** new Uint16Array with the same ArrayBuffer
**Arguments:**
- begin:numberstart index, inclusive
publicsubarray(): _Uint16Array_
Creates a Uint16Array with the same ArrayBuffer
**Returns:** new Uint16Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Uint16Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_
publictoLocaleString(locales: _object_ ): _string_
Converts Uint16Array to a string with respect to locale
**282 Chapter 2. Classes**


**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Uint16Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Uint16Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Uint16Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Uint16Array
**Returns:** a string representation of the Uint16Array

publicwith(index:number, value:number): _Uint16Array_
**2.35. Uint16Array 283**


Creates a copy with replaced value on index
**Returns:** an Uint16Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.35.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
- length:number
**2.36 Uint32Array**
export
JS Uint32Array API-compatible class

**2.36.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at

publicconstructor():void
Creates an empty Uint32Array.

**284 Chapter 2. Classes**


publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Uint32Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type number in newly created Uint32Array

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Uint32Array with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Uint32Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Uint32Array_ ):void
Creates a copy of Uint32Array.
**Arguments:**

- other: _Uint32Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**2.36. Uint32Array 285**


**Arguments:**

- insertPos:numberinsert index to place copied elements
- • startPos:endPos:numbernumberlast index to end copy from, excluded See rules of parameters normalization on MDNstart index to begin copy from

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Uint32Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Uint32Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number, index:number, array: _Uint32Array_ ) =>boolean):boolean
Checks that all elements of Uint32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint32Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**286 Chapter 2. Classes**


**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Uint32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Uint32Array_ ) =>boolean):boolean
Checks that all elements of Uint32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint32Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint32Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Uint32Array_
Fills the Uint32Array with specified value
**Returns:** modified Uint32Array

**2.36. Uint32Array 287**


**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Uint32Array_
Fills the Uint32Array with specified value
**Returns:** modified Uint32Array
**Arguments:**
- value:numbernew valuy
- start:number
publicfill(value:number): _Uint32Array_
Fills the Uint32Array with specified value
**Returns:** modified Uint32Array
**Arguments:**
- value:numbernew valuy

publicfilter( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean): _Uint32Array_
Creates a new Uint32Array from current Uint32Array based on a condition fn.
**Returns:** a new Uint32Array with elements from current Uint32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Uint32Array_
creates a new Uint32Array from current Uint32Array based on a condition fn
**288 Chapter 2. Classes**


**Returns:** a new Uint32Array with elements from current Uint32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number) =>boolean): _Uint32Array_
creates a new Uint32Array from current Uint32Array based on a condition fn
**Returns:** a new Uint32Array with elements from current Uint32Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean): _Uint32Array_
Creates a new Uint32Array from current Uint32Array based on a condition fn.
**Returns:** a new Uint32Array with elements from current Uint32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Uint32Array_
creates a new Uint32Array from current Uint32Array based on a condition fn
**Returns:** a new Uint32Array with elements from current Uint32Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds the first element in the Uint32Array that satisfies the condition

**2.36. Uint32Array 289**


**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Uint32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds the first element in the Uint32Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint32Array that satisfies the condition
**Returns:** the first element that satisfies fn
**290 Chapter 2. Classes**


**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds an index of the first element in the Uint32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Uint32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds an index of the first element in the Uint32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn

**2.36. Uint32Array 291**


**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint32Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds the last element in the Uint32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Uint32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:
292 Chapter 2. Classes**


- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds the last element in the Uint32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint32Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds an index of the last element in the Uint32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition
**2.36. Uint32Array 293**


publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Uint32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Uint32Array_ ) =>boolean):number
Finds an index of the last element in the Uint32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint32Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicforEach( fn: (val:number, index:number, array: _Uint32Array_ ) =>number):void
Applies a function over all elements of Uint32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>numberfunction to apply

**294 Chapter 2. Classes**


publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Uint32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply

publicforEach( fn: (val:number, index:number, array: _Uint32Array_ ) =>number):void
Applies a function over all elements of Uint32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Uint32Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint32Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

**2.36. Uint32Array 295**


publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Uint32Array_
Creates an Uint32Array from array-like argument
**Returns:** new Uint32Array
**Arguments:**

- o: _object_ array-like object to initialize Uint32Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each

publicfrom(o: _object_ ): _Uint32Array_
Creates an Uint32Array from array-like argument
**Returns:** new Uint32Array
**Arguments:**

- o: _object_ array-like object to initialize Uint32Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint32Array_
Creates an Uint32Array from array-like argument
**Returns:** new Uint32Array
**Arguments:**

- o: _object_ array-like object to initialize Uint32Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint32Array_
Creates an Uint32Array from array-like argument
**Returns:** new Uint32Array
**Arguments:
296 Chapter 2. Classes**


- o: _object_ array-like object to initialize Uint32Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Uint32Array
**Returns:** true if e is in Uint32Array, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Uint32Array
**Returns:** true if e is in Uint32Array, false otherwise
**Arguments:**

- e:numbersearch element

publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**2.36. Uint32Array 297**


**Arguments:**

- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**
- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found

**298 Chapter 2. Classes**


**Arguments:**

- val:numbera value to search
publicmap( fn: (val:number, index:number) =>number): _Uint32Array_
Creates a new Uint32Array using fn(arr[i]) over all elements of current Uint32Array.
**Returns:** a new Uint32Array where for each element from current Uint32Array fn was applied
**Arguments:**
- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint32Array

publicmap( fn: (val:number) =>number): _Uint32Array_
Creates a new Uint32Array using fn(arr[i]) over all elements of current Uint32Array
**Returns:** a new Uint32Array where for each element from current Uint32Array fn was applied
**Arguments:**

- fn: (val:number) =>numbera function to apply for each element of current Uint32Array

publicmap( fn: (val:number, index:number) =>number): _Uint32Array_
Creates a new Uint32Array using fn(arr[i]) over all elements of current Uint32Array.
**Returns:** a new Uint32Array where for each element from current Uint32Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint32Array

publicof(data:number[]): _Uint32Array_
Creates a new Uint32Array using initializer
**Returns:** a new Uint32Array from data
**Arguments:
2.36. Uint32Array 299**


- data:number[] initializer

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition
- init:numberinitial value

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition
- init:numberinitial value

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**300 Chapter 2. Classes**


**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition

publicnumberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition
- init:numberinitial value
publicnumberreduceRight( fn: (acc: number, curVal: number, curIndex: number, array: _Uint32Array_ ) =>number):

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition

publicnumberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition
- init:numberinitial value
publicnumberreduceRight( fn: (acc: number, curVal: number, curIndex: number, array: _Uint32Array_ ) =>number):

**2.36. Uint32Array 301**


Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint32Array_ ) =>numbercondition

publicreverse(): _Uint32Array_
Creates a new Uint32Array using reversed data from the current one
**Returns:** a new Uint32Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(number[], insertPos)
**Arguments:**

- insertPos:numberindex to change
- val:numbervalue to set

publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Uint32Array starting from insertPos.
**Arguments:**

- arr:number[] array to copy data from
- insertPos1:number

publicset(arr:number[]):void
Copies all elements of arr to the current Uint32Array.

**302 Chapter 2. Classes**


**Arguments:**

- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Uint32Array_
Creates a slice of current Uint32Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Uint32Array with elements of current Uint32Array[begin;end) where end index is excluded

**Arguments:**

- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _Uint32Array_
Creates a slice of current Uint32Array using range [begin, this.length).
**Returns:** a new Uint32Array with elements of current Uint32Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _Uint32Array_
Creates a slice of current Uint32 with all elements.
**Returns:** a new Uint32Array with elements of current Uint32Array

publicsome( fn: (element:number, index:number, array: _Uint32Array_ ) =>boolean):boolean
Checks that at least one element of Uint32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:
2.36. Uint32Array 303**


- fn: (element:number, index:number, array: _Uint32Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Uint32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicsome( fn: (element:number, index:number, array: _Uint32Array_ ) =>boolean):boolean
Checks that at least one element of Uint32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint32Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint32Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function
**304 Chapter 2. Classes**


publicsort(): _Uint32Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted Uint32Array

publicsort( fn: (a:number, b:number) =>number): _Uint32Array_
Sorts in-place
**Returns:** sorted Uint32Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Uint32Array_
Creates a Uint32Array with the same underlying ArrayBuffer
**Returns:** new Uint32Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive

publicsubarray(begin:number): _Uint32Array_
Creates a Uint32Array with the same ArrayBuffer
**Returns:** new Uint32Array with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

**2.36. Uint32Array 305**


publicsubarray(): _Uint32Array_
Creates a Uint32Array with the same ArrayBuffer
**Returns:** new Uint32Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Uint32Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts Uint32Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Uint32Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Uint32Array_

**306 Chapter 2. Classes**


Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Uint32Array_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Uint32Array
**Returns:** a string representation of the Uint32Array

publicwith(index:number, value:number): _Uint32Array_
Creates a copy with replaced value on index
**Returns:** an Uint32Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.36.2 Properties**

- static BYTES_PER_ELEMENT:number
- buffer: _ArrayBuffer_
- byteLength:number
- byteOffset:number
- length:number
**2.36. Uint32Array 307**


**2.37 Uint8Array**
export
JS Uint8Array API-compatible class

**2.37.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at
publicconstructor():void
Creates an empty Uint8Array.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Uint8Array with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- • byteOffset:length:numbernumbersize of elements of type number in newly created Uint8Arraybyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Uint8Array with respect to buf and byteOffset.
**Arguments:**

**308 Chapter 2. Classes**


- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Uint8Array with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

publicconstructor(other: _Uint8Array_ ):void
Creates a copy of Uint8Array.
**Arguments:**

- other: _Uint8Array_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Uint8Array.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

**2.37. Uint8Array 309**


publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Uint8Array.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number, index:number, array: _Uint8Array_ ) =>boolean):boolean
Checks that all elements of Uint8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Uint8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Uint8Array_ ) =>boolean):boolean
**310 Chapter 2. Classes**


Checks that all elements of Uint8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8Array_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint8Array satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Uint8Array_
Fills the Uint8Array with specified value
**Returns:** modified Uint8Array
**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Uint8Array_
Fills the Uint8Array with specified value
**Returns:** modified Uint8Array
**Arguments:**
- value:numbernew valuy
- start:number

**2.37. Uint8Array 311**


publicfill(value:number): _Uint8Array_
Fills the Uint8Array with specified value
**Returns:** modified Uint8Array
**Arguments:**

- value:numbernew valuy

publicfilter( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean): _Uint8Array_
Creates a new Uint8Array from current Uint8Array based on a condition fn.
**Returns:** a new Uint8Array with elements from current Uint8Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Uint8Array_
creates a new Uint8Array from current Uint8Array based on a condition fn
**Returns:** a new Uint8Array with elements from current Uint8Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number) =>boolean): _Uint8Array_
creates a new Uint8Array from current Uint8Array based on a condition fn
**Returns:** a new Uint8Array with elements from current Uint8Array that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

**312 Chapter 2. Classes**


publicfilter( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean): _Uint8Array_
Creates a new Uint8Array from current Uint8Array based on a condition fn.
**Returns:** a new Uint8Array with elements from current Uint8Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number) =>boolean): _Uint8Array_
creates a new Uint8Array from current Uint8Array based on a condition fn
**Returns:** a new Uint8Array with elements from current Uint8Array that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds the first element in the Uint8Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint8Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**2.37. Uint8Array 313**


publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Uint8Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfind( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds the first element in the Uint8Array that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleanthe condition to apply for each element

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint8Array that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds an index of the first element in the Uint8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleancondition

**314 Chapter 2. Classes**


publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Uint8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds an index of the first element in the Uint8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint8Array that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

**2.37. Uint8Array 315**


publicfindLast( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds the last element in the Uint8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Uint8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds the last element in the Uint8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleancondition

**316 Chapter 2. Classes**


publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint8Array that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds an index of the last element in the Uint8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Uint8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

**2.37. Uint8Array 317**


publicfindLastIndex( fn: (val:number, index:number, array: _Uint8Array_ ) =>boolean):number
Finds an index of the last element in the Uint8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint8Array that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicforEach( fn: (val:number, index:number, array: _Uint8Array_ ) =>number):void
Applies a function over all elements of Uint8Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Uint8Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint8Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

**318 Chapter 2. Classes**


publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Uint8Array
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply
publicforEach( fn: (val:number, index:number, array: _Uint8Array_ ) =>number):void
Applies a function over all elements of Uint8Array
**Returns:** undefined
**Arguments:**
- fn: (val:number, index:number, array: _Uint8Array_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint8Array
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply
publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Uint8Array_
Creates an Uint8Array from array-like argument
**Returns:** new Uint8Array
**Arguments:**
- o: _object_ array-like object to initialize Uint8Array
- mapFn: (e: _object_ ) =>numberfunction to apply for each
**2.37. Uint8Array 319**


publicfrom(o: _object_ ): _Uint8Array_
Creates an Uint8Array from array-like argument
**Returns:** new Uint8Array
**Arguments:**

- o: _object_ array-like object to initialize Uint8Array

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint8Array_
Creates an Uint8Array from array-like argument
**Returns:** new Uint8Array
**Arguments:**

- o: _object_ array-like object to initialize Uint8Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint8Array_
Creates an Uint8Array from array-like argument
**Returns:** new Uint8Array
**Arguments:**

- o: _object_ array-like object to initialize Uint8Array
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Uint8Array
**Returns:** true if e is in Uint8Array, false otherwise
**Arguments:
320 Chapter 2. Classes**


- e:numbersearch element
- fromIndex:numberstart index to search from
publicincludes(e:number):boolean
Checks if specified argument is in Uint8Array
**Returns:** true if e is in Uint8Array, false otherwise
**Arguments:**
- e:numbersearch element
publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
- fromIndex:numberstart index to search from
publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:
2.37. Uint8Array 321**


- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search
publicmap( fn: (val:number, index:number) =>number): _Uint8Array_
Creates a new Uint8Array using fn(arr[i]) over all elements of current Uint8Array.
**Returns:** a new Uint8Array where for each element from current Uint8Array fn was applied
**Arguments:**
- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint8Array
**322 Chapter 2. Classes**


publicmap( fn: (val:number) =>number): _Uint8Array_
Creates a new Uint8Array using fn(arr[i]) over all elements of current Uint8Array
**Returns:** a new Uint8Array where for each element from current Uint8Array fn was applied
**Arguments:**

- fn: (val:number) =>numbera function to apply for each element of current Uint8Array

publicmap( fn: (val:number, index:number) =>number): _Uint8Array_
Creates a new Uint8Array using fn(arr[i]) over all elements of current Uint8Array.
**Returns:** a new Uint8Array where for each element from current Uint8Array fn was applied
**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint8Array

publicof(data:number[]): _Uint8Array_
Creates a new Uint8Array using initializer
**Returns:** a new Uint8Array from data
**Arguments:**

- data:number[] initializer

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition
**2.37. Uint8Array 323**


- init:numberinitial value
publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition

public):numberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number, init:number

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition
- init:numberinitial value

publicreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number):number
Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition

publicnumber):reduceRight( fn: (acc:number number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result

**324 Chapter 2. Classes**


**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition
- init:numberinitial value
publicreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number):number
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition

publicnumber):reduceRight( fn: (acc:number number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number, init:

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition
- init:numberinitial value
publicreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>number):number
Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**
- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8Array_ ) =>numbercondition

publicreverse(): _Uint8Array_
Creates a new Uint8Array using reversed data from the current one

**2.37. Uint8Array 325**


**Returns:** a new Uint8Array using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(number[], insertPos)
**Arguments:**

- insertPos:numberindex to change
- val:numbervalue to set
publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Uint8Array starting from insertPos.
**Arguments:**
- arr:number[] array to copy data from
- insertPos1:number

publicset(arr:number[]):void
Copies all elements of arr to the current Uint8Array.
**Arguments:**

- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Uint8Array_
Creates a slice of current Uint8Array using range [begin, end)
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Uint8Array with elements of current Uint8Array[begin;end) where end index is excluded

**Arguments:**

**326 Chapter 2. Classes**


- begin:numberstart index to be taken into slice
- end:numberlast index to be taken into slice
publicslice(begin:number): _Uint8Array_
Creates a slice of current Uint8Array using range [begin, this.length).
**Returns:** a new Uint8Array with elements of current Uint8Array[begin, this.length)
**Arguments:**
- begin:numberstart index to be taken into slice

publicslice(): _Uint8Array_
Creates a slice of current Uint8 with all elements.
**Returns:** a new Uint8Array with elements of current Uint8Array

publicsome( fn: (element:number, index:number, array: _Uint8Array_ ) =>boolean):boolean
Checks that at least one element of Uint8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

**2.37. Uint8Array 327**


publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Uint8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicsome( fn: (element:number, index:number, array: _Uint8Array_ ) =>boolean):boolean
Checks that at least one element of Uint8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8Array_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint8Array satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Uint8Array_
Sorts in-place according to the numeric ordering
**Returns:** sorted Uint8Array

publicsort( fn: (a:number, b:number) =>number): _Uint8Array_
**328 Chapter 2. Classes**


Sorts in-place
**Returns:** sorted Uint8Array
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Uint8Array_
Creates a Uint8Array with the same underlying ArrayBuffer
**Returns:** new Uint8Array with the same underlying ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive

publicsubarray(begin:number): _Uint8Array_
Creates a Uint8Array with the same ArrayBuffer
**Returns:** new Uint8Array with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

publicsubarray(): _Uint8Array_
Creates a Uint8Array with the same ArrayBuffer
**Returns:** new Uint8Array with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Uint8Array to a string with respect to locale

**2.37. Uint8Array 329**


**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts Uint8Array to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_

publictoLocaleString(): _string_
Converts Uint8Array to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Uint8Array_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Uint8Array_
Creates a sorted copy
**Returns:** a sorted copy

**330 Chapter 2. Classes**


public overridetoString(): _string_
Returns a string representation of the Uint8Array
**Returns:** a string representation of the Uint8Array

publicwith(index:number, value:number): _Uint8Array_
Creates a copy with replaced value on index
**Returns:** an Uint8Array with replaced value on index
**Arguments:**

- index:number
- value:number

**2.37.2 Properties**

- • static BYTES_PER_ELEMENT:buffer: _ArrayBuffer_ number
- byteLength:number
- byteOffset:number
- length:number
**2.38 Uint8ClampedArray**
export
JS Uint8ClampedArray API-compatible class

**2.38.1 Methods**
publicat(index:number):number
Returns an instance of primitive type at passed index.
**2.38. Uint8ClampedArray 331**


**Returns:** a primitive at index
**Arguments:**

- index:numberindex to look at

publicconstructor():void
Creates an empty Uint8ClampedArray.

publicconstructor( buf: _ArrayBuffer_ , byteOffset:number, length:number):void
Creates an Uint8ClampedArray with respect to data, byteOffset and length.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf
- length:numbersize of elements of type number in newly created Uint8ClampedArray

publicconstructor(buf: _ArrayBuffer_ , byteOffset:number):void
Creates an Uint8ClampedArray with respect to buf and byteOffset.
**Arguments:**

- buf: _ArrayBuffer_ data initializer
- byteOffset:numberbyte offset from begin of the buf

publicconstructor(buf: _ArrayBuffer_ ):void
Creates an Uint8ClampedArray with respect to buf.
**Arguments:**

- buf: _ArrayBuffer_ data initializer

**332 Chapter 2. Classes**


publicconstructor(other: _Uint8ClampedArray_ ):void
Creates a copy of Uint8ClampedArray.
**Arguments:**

- other: _Uint8ClampedArray_ data initializer

publiccopyWithin( insertPos:number, startPos:number, endPos:number):void
Makes a copy of internal elements to insertPos from startPos to endPos.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:numberstart index to begin copy from
- endPos:numberlast index to end copy from, excluded See rules of parameters normalization on MDN

publiccopyWithin(insertPos:number, startPos:number):void
Makes a copy of internal elements to insertPos from startPos to end of Uint8ClampedArray.
**Arguments:**

- insertPos:numberinsert index to place copied elements
- startPos:org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithinnumberstart index to begin copy from See rules of parameters normalization https://developer.mozilla.

publiccopyWithin(insertPos:number):void
Makes a copy of internal elements to insertPos from begin to end of Uint8ClampedArray.See rules of parameters normalization:
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/Array/copyWithin

publicevery( fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):boolean

**2.38. Uint8ClampedArray 333**


Checks that all elements of Uint8ClampedArray satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint8ClampedArray satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicevery( fn: (element:number) =>boolean):boolean
Checks that all elements of Uint8ClampedArray satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicevery( fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):boolean
Checks that all elements of Uint8ClampedArray satisfy the passed function
**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>booleancheck function

publicevery( fn: (element:number, index:number) =>boolean):boolean
Checks that all elements of Uint8ClampedArray satisfy the passed function
**334 Chapter 2. Classes**


**Returns:** true if all elements satisfy fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicfill( value:number, start:number, end:number): _Uint8ClampedArray_
Fills the Uint8ClampedArray with specified value
**Returns:** modified Uint8ClampedArray
**Arguments:**

- value:numbernew valuy
- start:number
- end:number
publicfill(value:number, start:number): _Uint8ClampedArray_
Fills the Uint8ClampedArray with specified value
**Returns:** modified Uint8ClampedArray
**Arguments:**
- value:numbernew valuy
- start:number
publicfill(value:number): _Uint8ClampedArray_
Fills the Uint8ClampedArray with specified value
**Returns:** modified Uint8ClampedArray
**Arguments:**
- value:numbernew valuy

publicfilter( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean): _Uint8ClampedArray_
**2.38. Uint8ClampedArray 335**


Creates a new Uint8ClampedArray from current Uint8ClampedArray based on a condition fn.
**Returns:** a new Uint8ClampedArray with elements from current Uint8ClampedArray that satisfy condition fn
**Arguments:**

- fn: (val:element number, index:number, array: _Uint8ClampedArray_ ) =>booleanthe condition to apply for each

publicfilter( fn: (val:number, index:number) =>boolean): _Uint8ClampedArray_
creates a new Uint8ClampedArray from current Uint8ClampedArray based on a condition fn
**Returns:** a new Uint8ClampedArray with elements from current Uint8ClampedArray that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number) =>boolean): _Uint8ClampedArray_
creates a new Uint8ClampedArray from current Uint8ClampedArray based on a condition fn
**Returns:** a new Uint8ClampedArray with elements from current Uint8ClampedArray that satisfy condition fn
**Arguments:**

- fn: (val:number) =>booleanthe condition to apply for each element

publicfilter( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean): _Uint8ClampedArray_
Creates a new Uint8ClampedArray from current Uint8ClampedArray based on a condition fn.
**Returns:** a new Uint8ClampedArray with elements from current Uint8ClampedArray that satisfy condition fn
**Arguments:**

- fn: (val:element number, index:number, array: _Uint8ClampedArray_ ) =>booleanthe condition to apply for each

publicfilter( fn: (val:number, index:number) =>boolean): _Uint8ClampedArray_

**336 Chapter 2. Classes**


creates a new Uint8ClampedArray from current Uint8ClampedArray based on a condition fn
**Returns:** a new Uint8ClampedArray with elements from current Uint8ClampedArray that satisfy condition fn
**Arguments:**

- fn: (val:number, index:number) =>booleanthe condition to apply for each element
publicfind( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**
- fn: (val:element number, index:number, array: _Uint8ClampedArray_ ) =>booleanthe condition to apply for each

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition
publicfind( fn: (val:number) =>boolean):number
Finds the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**
- fn: (val:number) =>booleancondition
publicfind( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds the first element in the Uint8ClampedArray that satisfies the condition
**2.38. Uint8ClampedArray 337**


**Returns:** the first element that satisfies fn TODO: return number | undefined as in JS
**Arguments:**

- fn: (val:element number, index:number, array: _Uint8ClampedArray_ ) =>booleanthe condition to apply for each

publicfind( fn: (val:number, index:number) =>boolean):number
Finds the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds an index of the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindIndex( fn: (val:number) =>boolean):number
Finds an index of the first element in the Uint8ClampedArray that satisfies the condition
**338 Chapter 2. Classes**


**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindIndex( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds an index of the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>booleancondition

publicfindIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the first element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the first element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint8ClampedArray that satisfies the condition

**2.38. Uint8ClampedArray 339**


**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLast( fn: (val:number) =>boolean):number
Finds the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLast( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>booleancondition

publicfindLast( fn: (val:number, index:number) =>boolean):number
Finds the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the last element that satisfies fn
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds an index of the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**340 Chapter 2. Classes**


**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicfindLastIndex( fn: (val:number) =>boolean):number
Finds an index of the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):number
Finds an index of the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise
**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>booleancondition

publicfindLastIndex( fn: (val:number, index:number) =>boolean):number
Finds an index of the last element in the Uint8ClampedArray that satisfies the condition
**Returns:** the index of the last element that satisfies fn, -1 otherwise

**2.38. Uint8ClampedArray 341**


**Arguments:**

- fn: (val:number, index:number) =>booleancondition

publicforEach( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>number):void
Applies a function over all elements of Uint8ClampedArray
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint8ClampedArray
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

publicforEach( fn: (val:number) =>number):void
Applies a function over all elements of Uint8ClampedArray
**Returns:** undefined
**Arguments:**

- fn: (val:number) =>numberfunction to apply

publicforEach( fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>number):void
Applies a function over all elements of Uint8ClampedArray
**Returns:** undefined
**Arguments:
342 Chapter 2. Classes**


- fn: (val:number, index:number, array: _Uint8ClampedArray_ ) =>numberfunction to apply

publicforEach( fn: (val:number, index:number) =>number):void
Applies a function over all elements of Uint8ClampedArray
**Returns:** undefined
**Arguments:**

- fn: (val:number, index:number) =>numberfunction to apply

publicfrom( o: _object_ , mapFn: (e: _object_ ) =>number): _Uint8ClampedArray_
Creates an Uint8ClampedArray from array-like argument
**Returns:** new Uint8ClampedArray
**Arguments:**

- o: _object_ array-like object to initialize Uint8ClampedArray
- mapFn: (e: _object_ ) =>numberfunction to apply for each

publicfrom(o: _object_ ): _Uint8ClampedArray_
Creates an Uint8ClampedArray from array-like argument
**Returns:** new Uint8ClampedArray
**Arguments:**

- o: _object_ array-like object to initialize Uint8ClampedArray

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint8ClampedArray_
Creates an Uint8ClampedArray from array-like argument
**Returns:** new Uint8ClampedArray
**Arguments:**

**2.38. Uint8ClampedArray 343**


- o: _object_ array-like object to initialize Uint8ClampedArray
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicfrom( o: _object_ , mapFn: (e: _object_ , index:number) =>number): _Uint8ClampedArray_
Creates an Uint8ClampedArray from array-like argument
**Returns:** new Uint8ClampedArray
**Arguments:**

- o: _object_ array-like object to initialize Uint8ClampedArray
- mapFn: (e: _object_ , index:number) =>numberfunction to apply for each

publicincludes(e:number, fromIndex:number):boolean
Checks if specified argument is in Uint8ClampedArray
**Returns:** true if e is in Uint8ClampedArray, false otherwise
**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from

publicincludes(e:number):boolean
Checks if specified argument is in Uint8ClampedArray
**Returns:** true if e is in Uint8ClampedArray, false otherwise
**Arguments:**

- e:numbersearch element

publicindexOf(e:number, fromIndex:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**344 Chapter 2. Classes**


**Arguments:**

- e:numbersearch element
- fromIndex:numberstart index to search from
publicindexOf(e:number):number
Returns index of specified element
**Returns:** index of element if it presents, -1 otherwise
**Arguments:**
- e:numbersearch element
publicjoin(s: _string_ ): _string_
Joins data to a string
**Returns:** joined representation
**Arguments:**
- s: _string_ separator

publicjoin(): _string_
Joins data to a string
**Returns:** joined representation with comma separator

publiclastIndexOf(val:number, fromIndex:number):number
Moves backwards starting at fromIndex to 0 and search val.
**Returns:** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/lastIndexOfright-most index of val. It must be less or equal than fromIndex. -1 if val not found

**2.38. Uint8ClampedArray 345**


**Arguments:**

- val:numbera value to search
- fromIndex:numberthe first index to search val at, i.e. fromIndex is included in search space

publiclastIndexOf(val:number):number
Moves backwards and search val.
**Returns:** right-most index of val. -1 if val not found
**Arguments:**

- val:numbera value to search
publicmap( fn: (val:number, index:number) =>number): _Uint8ClampedArray_
Creates a new Uint8ClampedArray using fn(arr[i]) over all elements of current Uint8ClampedArray.
**Returns:** a new Uint8ClampedArray where for each element from current Uint8ClampedArray fn was applied
**Arguments:**
- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint8ClampedArray

publicmap( fn: (val:number) =>number): _Uint8ClampedArray_
Creates a new Uint8ClampedArray using fn(arr[i]) over all elements of current Uint8ClampedArray
**Returns:** a new Uint8ClampedArray where for each element from current Uint8ClampedArray fn was applied
**Arguments:**

- fn: (val:number) =>numbera function to apply for each element of current Uint8ClampedArray

publicmap( fn: (val:number, index:number) =>number): _Uint8ClampedArray_
Creates a new Uint8ClampedArray using fn(arr[i]) over all elements of current Uint8ClampedArray.
**Returns:** a new Uint8ClampedArray where for each element from current Uint8ClampedArray fn was applied

**346 Chapter 2. Classes**


**Arguments:**

- fn: (val:number, index:number) =>numbera function to apply for each element of current Uint8ClampedArray

publicof(data:number[]): _Uint8ClampedArray_
Creates a new Uint8ClampedArray using initializer
**Returns:** a new Uint8ClampedArray from data
**Arguments:**

- data:number[] initializer

publicnumberreduce( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number, init:

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition
- init:numberinitial value
publicnumberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number):

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition

publicnumberreduce( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number, init:

Reduces data into a single value using left-to-right traversal
**2.38. Uint8ClampedArray 347**


**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition
- init:numberinitial value
publicnumberreduce( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number):

Reduces data into a single value using left-to-right traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition

publicinit:numberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number,

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition
- init:numberinitial value

public):numberreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition

**348 Chapter 2. Classes**


publicinit:numberreduceRight( fn: (acc:):number number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number,

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition
- init:numberinitial value
public):numberreduceRight( fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>number

Reduces data into a single value using right-to-left traversal
**Returns:** reduction result
**Arguments:**

- fn: (acc:number, curVal:number, curIndex:number, array: _Uint8ClampedArray_ ) =>numbercondition

publicreverse(): _Uint8ClampedArray_
Creates a new Uint8ClampedArray using reversed data from the current one
**Returns:** a new Uint8ClampedArray using reversed data from the current one

publicset(insertPos:number, val:number):void
Assigns val as element on insertPos.
**Description:** Added to avoid (un)packing a single value into array to use overloaded set(number[], insertPos)
**Arguments:**

- insertPos:numberindex to change
**2.38. Uint8ClampedArray 349**


- val:numbervalue to set
publicset(arr:number[], insertPos1:number):void
Copies all elements of arr to the current Uint8ClampedArray starting from insertPos.
**Arguments:**
- arr:number[] array to copy data from
- insertPos1:number
publicset(arr:number[]):void
Copies all elements of arr to the current Uint8ClampedArray.
**Arguments:**
- arr:number[] array to copy data from

publicslice(begin:number, end:number): _Uint8ClampedArray_
Creates a slice of current Uint8ClampedArray using range [begin, end)
**Returns:** excluded https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_objects/TypedArray/slicea new Uint8ClampedArray with elements of current Uint8ClampedArray[begin;end) where end index is

**Arguments:**

- • begin:end:numbernumberlast index to be taken into slicestart index to be taken into slice

publicslice(begin:number): _Uint8ClampedArray_
Creates a slice of current Uint8ClampedArray using range [begin, this.length).
**Returns:** a new Uint8ClampedArray with elements of current Uint8ClampedArray[begin, this.length)
**Arguments:**

- begin:numberstart index to be taken into slice

**350 Chapter 2. Classes**


publicslice(): _Uint8ClampedArray_
Creates a slice of current Uint8Clamped with all elements.
**Returns:** a new Uint8ClampedArray with elements of current Uint8ClampedArray

publicsome( fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):boolean
Checks that at least one element of Uint8ClampedArray satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint8ClampedArray satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsome( fn: (element:number) =>boolean):boolean
Checks that at least one element of Uint8ClampedArray satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number) =>booleancheck function

publicsome( fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>boolean):boolean
**2.38. Uint8ClampedArray 351**


Checks that at least one element of Uint8ClampedArray satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number, array: _Uint8ClampedArray_ ) =>booleancheck function

publicsome( fn: (element:number, index:number) =>boolean):boolean
Checks that at least one element of Uint8ClampedArray satisfies the passed function
**Returns:** true if some element satisfies fn
**Arguments:**

- fn: (element:number, index:number) =>booleancheck function

publicsort(): _Uint8ClampedArray_
Sorts in-place according to the numeric ordering
**Returns:** sorted Uint8ClampedArray

publicsort( fn: (a:number, b:number) =>number): _Uint8ClampedArray_
Sorts in-place
**Returns:** sorted Uint8ClampedArray
**Arguments:**

- fn: (a:number, b:number) =>numbercomparator

publicsubarray(begin:number, end:number): _Uint8ClampedArray_
Creates a Uint8ClampedArray with the same underlying ArrayBuffer
**Returns:** new Uint8ClampedArray with the same underlying ArrayBuffer
**352 Chapter 2. Classes**


**Arguments:**

- begin:numberstart index, inclusive
- end:numberlast index, exclusive

publicsubarray(begin:number): _Uint8ClampedArray_
Creates a Uint8ClampedArray with the same ArrayBuffer
**Returns:** new Uint8ClampedArray with the same ArrayBuffer
**Arguments:**

- begin:numberstart index, inclusive

publicsubarray(): _Uint8ClampedArray_
Creates a Uint8ClampedArray with the same ArrayBuffer
**Returns:** new Uint8ClampedArray with the same ArrayBuffer

publictoLocaleString(locales: _object_ , options: _object_ ): _string_
Converts Uint8ClampedArray to a string with respect to locale
**Returns:** string representation
**Arguments:**

- locales: _object_
- options: _object_

publictoLocaleString(locales: _object_ ): _string_
Converts Uint8ClampedArray to a string with respect to locale
**Returns:** string representation
**2.38. Uint8ClampedArray 353**


**Arguments:**

- locales: _object_
publictoLocaleString(): _string_
Converts Uint8ClampedArray to a string with respect to locale
**Returns:** string representation

publictoReversed(): _Uint8ClampedArray_
Creates a reversed copy
**Returns:** a reversed copy

publictoSorted(): _Uint8ClampedArray_
Creates a sorted copy
**Returns:** a sorted copy

public overridetoString(): _string_
Returns a string representation of the Uint8ClampedArray
**Returns:** a string representation of the Uint8ClampedArray

publicwith(index:number, value:number): _Uint8ClampedArray_
Creates a copy with replaced value on index
**354 Chapter 2. Classes**


**Returns:** an Uint8ClampedArray with replaced value on index
**Arguments:**

- index:number
- value:number

**2.38.2 Properties**

- • static BYTES_PER_ELEMENT:buffer: _ArrayBuffer_ number
- byteLength:number
- byteOffset:number
- length:number
**2.39 WeakMap<K, V>**
export
**Class:** values of any arbitrary JavaScript type, and which does not create strong references to its keys.A WeakMap is a collection of key/value pairs whose keys must be objects or non-registered symbols, with

**2.39.1 Methods**
publicconstructor():void
The WeakMap() constructor creates WeakMap objects.

publicdelete(k: K):boolean
The delete() method removes the specified element from a WeakMap object.
**Returns:** boolean
**Arguments:**

- k: K

**2.39. WeakMap<K, V> 355**


publicget(k: K): V |null
The get() method returns a specified element from a WeakMap object.
**Returns:** related value or null
**Arguments:**

- k: K
publichas(k: K):boolean
The has() method returns a boolean indicating whether an element with the specified key exists in the WeakMapobject or not.

**Returns:** true if k is in set
**Arguments:**

- k: K
publicset(k: K, v: V): _WeakMap_ <K, V>
The set() method adds a new element with a specified key and value to a WeakMap object.
**Returns:** updatesWeakMap
**Arguments:**
- k: K
- v: V

**2.40 WeakSet<K>**
export
**Class:** unique in the WeakSet’s collection.A WeakSet is a collection of garbage-collectable values. A value in the WeakSet may only occur once. It is

**356 Chapter 2. Classes**


**2.40.1 Methods**
publicadd(v: K): _WeakSet_ <K>
The add() method appends a new object to the end of a WeakSet object.
**Returns:** new WeakSet with v
**Arguments:**

- v: K
publicconstructor():void
The WeakSet() constructor creates WeakSet objects.

publicdelete(v: K):boolean
The delete() method removes the specified element from a WeakSet object.
**Returns:** boolean
**Arguments:**

- v: K
publichas(v: K):boolean
The has() method returns a boolean indicating whether an object exists in a WeakSet or not.
**Returns:** true if set has v
**Arguments:**
- v: K
**2.41 WeakKey**
export

**2.41. WeakKey 357**


Represents WeakKey

**2.41.1 Methods**
publicconstructor():void
Constructs a new WeakKey

public overridetoString(): _string_
Converts string representation

**358 Chapter 2. Classes**



## v0.11.11 - 2024-03-07

- Update to support CK3 1.12 saves
- Update to support VIC3 1.6 saves

## v0.11.10 - 2023-11-21

- Improve support for VIC3 1.5 saves

## v0.11.9 - 2023-11-15

- Update to support CK3 1.11 saves
- Update to support VIC3 1.5 saves

## v0.11.8 - 2023-11-06

- Update to support EU4 1.36 saves

## v0.11.7 - 2023-10-15

- Update to support HOI4 1.13 saves

## v0.11.6 - 2023-10-09

- Update to support VIC3 1.4.2

## v0.11.5 - 2023-08-29

- Update to support VIC3 1.4
- Update to support CK3 1.10

## v0.11.4 - 2023-07-10

- Additional accuracy for melted vic3 output

## v0.11.3 - 2023-05-26

- Improve vic3 melt accuracy for pop_statistsics

## v0.11.2 - 2023-05-24

- Update EU4 tokens to 1.35.3
- Update CK3 tokens to 1.9
- Update vic3 tokens to 1.3
- Update hoi4 melter to exclude ironman key from output

## v0.11.1 - 2023-04-18

- Update EU4 tokens to 1.35

## v0.11.0 - 2023-01-05

Rework API so that it is no longer geared towards parsing and melting the input
in a single step. Instead, there are now a few steps where the API can cheaply
load the file and from there the downstream developer can decide what they want
to do: melt the metadata first or go straight to melting the main data.

See updated `sample.cpp` for new usage.

## v0.10.1 - 2022-11-12

- Fix human friendly error messages not being written to provided buffer when
  the buffer had exactly enough room for it.

## v0.10.0 - 2022-11-09

- Support converting compressed plaintext saves to uncompressed
- Add `rakaly_melt_binary_unknown_tokens` to know if unknown binary tokens were encountered in the input
- Add `rakaly_melt_is_verbatim` to know if no work was required to convert to uncompressed plaintext
- Add `rakaly_melt_binary_translated` to know if the input required binary translation
- Add `rakaly_melt_error_length` and `rakaly_melt_error_write_data` so one can show a human readable error messages

The C++ wrapper has been updated to take advantage of these new C APIs and has
been rewritten. Please see the README or `sample.cpp` to see new usage.

## v0.9.1 - 2022-11-06

- Consolidate separate c++ helper functions into main header

## v0.9.0 - 2022-11-05

- Initial support for melting Vic3 saves

## v0.8.17 - 2022-09-18

- Fix incorrect CK3 1.7 melted format for floats

## v0.8.16 - 2022-09-12

- Update to CK3 1.7 tokens
- Update to EU4 1.34 tokens
- Sizeable performance improvements for zipped save formats

## v0.8.15 - 2022-06-01

- Support CK3 1.6

## v0.8.14 - 2022-03-20

- Update EU4 melted output to be compatible with loading the save from the in game menu by not containing a terminating newline

## v0.8.13 - 2022-02-22

- Support EU4 1.33
- Support CK3 1.5
- Support HOI4 1.11

## v0.8.12 - 2021-11-14

- Update tokens to support new EU4 1.32 additions

## v0.8.11 - 2021-07-04

- Fix improper melted output when a name ended with a quote

## v0.8.10 - 2021-06-08

- EU4 dates prior to 5000 BC can now be melted properly and not cause an error (only an issue for EU4 mods)

## v0.8.9 - 2021-05-29

- Fix obscenely large CK3 melted output (introduced in v0.8.8) due to not accounting for hidden objects
- Fix some array values not being properly indented

## v0.8.8 - 2021-05-28

- Melt with tabs instead of spaces
- Melted quoted values are now escaped as needed. A quoted value that contained quotes didn't have the inner quotes escaped, leading to output that could fail to parse.

## v0.8.7 - 2021-05-18

- Melted output now only uses newlines for line endings
- eu4: correct number of decimal points are always used
- eu4: fixed the possibility of melted ids being detected as dates
- ck3: rewrite save header line with new metadata size
- ck3: omit certain ironman fields (`ironman` and `ironman_manager`) from melted output

## v0.8.6 - 2021-05-03

- Update tokens to support EU4 1.31.2
- Increase accuracy for melted EU4 64bit floats by up to a 10,000th
- Significant update to CK3 melting output:
  - Fix melted output containing quotes when plaintext has no quotes
  - Rewrite save header to declare the melted output is uncompressed plaintext
  - Increase accuracy of decoding 64 bit floats (alternative format) from ironman format
  - Write numbers as integers when ignoring the fractional component would not result in a loss of accuracy just like the plaintext format
  - Identified additional tokens that use the alternative float format
  - Fixed more numbers being interpreted as dates

## v0.8.5 - 2021-04-29

- Update tokens to support EU4 1.31.1
- Fix regression introduced in v0.8.4 where ck3 and imperator would melt all numbers as dates

## v0.8.4 - 2021-04-27

- Update melting to more accurately decode 64 bit floats (in rare cases large positive numbers could be interpreted as negative)
- Update melting to support Eu4 Leviathan prehistoric dates
- Update melting to support alternative Ck3 floating point format 
- Update tokens to support Eu4 Leviathan

## v0.8.3 - 2021-03-16

- Update CK3 tokens to support the 1.3 update

## v0.8.2 - 2021-02-17

- Update Imperator tokens to support the 2.0 update

## v0.8.1 - 2021-02-05

- Improved melting support. Won't quote values that aren't quoted in plaintext

## v0.8.0 - 2021-02-02

- Support for melting HOI4 saves
- Updated CK3 tokens to latest patch (1.2)

## v0.7.5 - 2021-01-25

Correctly melt seed properties as integers instead of dates

## v0.7.4 - 2020-12-09

Negative dates can now be melted into the equivalent plaintext

## v0.7.3 - 2020-12-09

Skipped release

## v0.7.2 - 2020-12-09

Skipped release

## v0.7.1 - 2020-10-06

Support parsing saves that contain empty containers:

```
history = {
  {}
  1689.10.2={
    decision="abc123"
  }
}
```

It is known that this format can occur in EU4 but it could also occur in others.

## v0.7.0 - 2020-10-02

* `rakaly_imperator_melt` added to melt imperator saves
* CK3 saves now melt `levels = { 10 0=1 1=2 }` losslessly

## v0.6.0 - 2020-09-08

A couple of small, but nevertheless breaking changes:

- `rakaly_melt_write_data` now returns a `size_t` to indicate how many bytes were written. If the given melted buffer is null or if the given length isn't long enough to write into, then a 0 instead of -1 is returned.
- `rakaly_melt_write_data` no longer writes a trailing null terminator into the buffer

## v0.5.0 - 2020-09-07

- Melt CK3 autosaves, ironman, and binary data into utf-8 plaintext
- Add c++ wrapper: `rakaly_wrapper.h`
- The c header, `rakaly.h`, is now c++ compatible

## v0.4.0 - 2020-09-04

This is a rewrite to the API so that one can properly deallocate the melted data that librakaly allocates. This is how the use the new API:

```c
MeltedBuffer *melt = rakaly_eu4_melt(buf_in, buf_in_len);
if (rakaly_melt_error_code(melt)) {
  rakaly_free_melt(melt);
  return 1;
}

size_t melted_len = rakaly_melt_data_length(melt);
size_t melted_str_len = melted_len + 1;
char *melted_buf = malloc(melted_str_len);

if (melted_buf == NULL) {
  rakaly_free_melt(melt);
  return 1;
}

size_t wrote_len = rakaly_melt_write_data(melt, melted_buf, melted_str_len);
if (wrote_len < 0) {
  free(melted_buf);
  rakaly_free_melt(melt);
  return 1;
}

rakaly_free_melt(melt);

// now do whatever you want with melted_buf

free(melted_buf);
```

eu4save is updated from 0.1.2 to 0.2.2. The only noticeable differences should be dates prior to 1000.1.1 will now have leading zeros removed, so the melted output will now correctly contain 1.1.1 instead of 0001.1.1

## v0.3.1 - 2020-08-15

- First open source release
- Fixed unknown ironman key when encountering a save with a revolution center

## v0.3.0 - 2020-07-24

- Instead of raising an error on unknown binary tokens found in ironman saves,
  skip these values if the tokens are part of an object's key, else write out
  an "__unknown_0x" string.
- Write only one checksum instance per melt operation (previously melting a
  zip would result in each file contributing a checksum in the output).
- Remove limit to how nested the binary parser can recurse. This will allow
  rakaly to melt modded saves that rely heavily on recursive events.

## v0.2.0 - 2020-07-09

- Fix failure to parse heavily nested events like Great Britain's symposium event (flavor_eng.9880). Previous parser could only handle this event nested about 15 times. I recently have come across a save where it exceeds this limit, so the fix was to increase the max to 30 layers of nesting.
- Fix melting failure to properly skip `is_ironman` property when it is a non-scalar (while `is_ironman` has always been a scalar, the melter is flexible enough to handle any format)
- (Not visible): a fuzzing suite has been added to the melter to ensure that malicious input can't cause undesirable effects (out of memory, stack overflows, seg faults, etc).

## v0.1.0 - 2020-06-15

Initial Release

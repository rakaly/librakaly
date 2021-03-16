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

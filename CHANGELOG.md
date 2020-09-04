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

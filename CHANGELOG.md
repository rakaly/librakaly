## v0.3 - 2020-07-24

- Instead of raising an error on unknown binary tokens found in ironman saves,
  skip these values if the tokens are part of an object's key, else write out
  an "__unknown_0x" string.
- Write only one checksum instance per melt operation (previously melting a
  zip would result in each file contributing a checksum in the output).
- Remove limit to how nested the binary parser can recurse. This will allow
  rakaly to melt modded saves that rely heavily on recursive events.

## v0.2 - 2020-07-09

- Fix failure to parse heavily nested events like Great Britain's symposium event (flavor_eng.9880). Previous parser could only handle this event nested about 15 times. I recently have come across a save where it exceeds this limit, so the fix was to increase the max to 30 layers of nesting.
- Fix melting failure to properly skip `is_ironman` property when it is a non-scalar (while `is_ironman` has always been a scalar, the melter is flexible enough to handle any format)
- (Not visible): a fuzzing suite has been added to the melter to ensure that malicious input can't cause undesirable effects (out of memory, stack overflows, seg faults, etc).

## v0.1 - 2020-06-15

Initial Release

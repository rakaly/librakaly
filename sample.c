#include "rakaly.h"
#include <stdio.h>
#include <stdlib.h>

long read_file(char const *path, char **buf) {
  FILE *fp = fopen(path, "rb");
  if (NULL == fp) {
    return -1;
  }

  if (fseek(fp, 0L, SEEK_END) != 0) {
    return -1;
  }

  long offset = ftell(fp);
  if (offset == -1) {
    return -1;
  }

  size_t file_size = (size_t)offset;

  *buf = malloc(file_size);
  if (*buf == NULL) {
    return -1;
  }

  rewind(fp);

  if (file_size != fread(*buf, 1, file_size, fp)) {
    free(*buf);
    return -1;
  }

  if (fclose(fp) == EOF) {
    free(*buf);
    return -1;
  }

  return offset;
}

int melt(char *buf_in, size_t buf_in_len) {
  MeltedBuffer *melt = rakaly_eu4_melt(buf_in, buf_in_len);
  if (rakaly_melt_error_code(melt)) {
    rakaly_free_melt(melt);
    fprintf(stderr, "unable to melt save\n");
    return 1;
  }

  size_t melted_len = rakaly_melt_data_length(melt);

  // Create buffer to store plaintext + 1 additional character to guarantee
  // null termination in case we need that behavior in the future.
  char *melted_buf = calloc(melted_len + 1, sizeof(char));

  if (melted_buf == NULL) {
    rakaly_free_melt(melt);
    fprintf(stderr, "unable to allocate melted data\n");
    return 1;
  }

  size_t wrote_len = rakaly_melt_write_data(melt, melted_buf, melted_len);
  if (wrote_len != melted_len) {
    free(melted_buf);
    rakaly_free_melt(melt);
    fprintf(stderr, "unable to write melted data\n");
    return 1;
  }

  rakaly_free_melt(melt);

  if (fwrite(melted_buf, melted_len, 1, stdout) != 1) {
    free(melted_buf);
    fprintf(stderr, "unable to write to stdout\n");
    return 1;
  }

  free(melted_buf);
  return 0;
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "expected one ironman file argument\n");
    return 1;
  }

  char *buf;
  long file_size = read_file(argv[1], &buf);
  if (file_size == -1) {
    fprintf(stderr, "unable to get read file\n");
    return 1;
  }

  int ret = melt(buf, (size_t)file_size);
  free(buf);
  return ret;
}

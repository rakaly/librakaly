#include <stdio.h>
#include <stdlib.h>
#include "rakaly.h"

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

int melt(char* buf_in, size_t buf_in_len) {
  char *out_buf;
  size_t out_size;
  char res = rakaly_eu4_melt(buf_in, buf_in_len, &out_buf, &out_size);
  if (res != 0) {
    perror("unable to melt save");
    return 1;
  }

  if (fwrite(out_buf, out_size, 1, stdout) == -1) {
    perror("unable to write to stdout");
    return 1;
  }

  return 0;
}

int main(int argc, char **argv) {
  if (argc != 2) {
    perror("expected one ironman file argument");
    return 1;
  }

  char *buf;
  long file_size = read_file(argv[1], &buf);
  if (file_size == -1) {
    perror("unable to get read file");
    return 1;
  }

  return melt(buf, (size_t)file_size);
}

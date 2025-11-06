#ifdef __cplusplus
#ifndef RAKALY_WRAPPER_H
#define RAKALY_WRAPPER_H

#include <optional>
#include <stdexcept>
#include <string>

namespace rakaly {

void unwrapError(PdsError *err) {
  if (err != nullptr) {
    int error_len = rakaly_error_length(err);
    std::string error(error_len, ' ');
    rakaly_error_write_data(err, error.data(), error_len);
    rakaly_free_error(err);
    auto msg = std::string("librakaly returned an error ") + error;
    throw std::runtime_error(msg);
  }
}

class MeltedOutput {
  MeltedBuffer *melt;

  MeltedOutput(const MeltedOutput &) = delete;

public:
  MeltedOutput(MeltedBuffer *melt) { this->melt = melt; }

  /**
   * Updates the given string with the melted output. The string is assumed to
   * contain the data that was requested to be melted, as if the melter required
   * no work, the string won't be written to (as it is already melted)
   */
  void writeData(std::string &data) const {
    // The passed in data is already uncompressed plaintext
    if (rakaly_melt_is_verbatim(melt)) {
      return;
    }

    size_t len = rakaly_melt_data_length(melt);
    data.resize(len);
    if (rakaly_melt_write_data(melt, data.data(), len) != len) {
      throw std::runtime_error("librakaly failed to copy data.");
    }
  }

  bool has_unknown_tokens() const {
    return rakaly_melt_binary_unknown_tokens(melt);
  }

  virtual ~MeltedOutput() { rakaly_free_melt(melt); }
};

class GameFile {
  PdsFile *file;

  GameFile(const GameFile &) = delete;

public:
  GameFile(PdsFile *file) { this->file = file; }

  bool is_binary() const { return rakaly_file_is_binary(file); }

  std::optional<MeltedOutput> meltMeta() const {
    PdsMeta *meta = rakaly_file_meta(file);
    if (meta == nullptr) {
      return std::nullopt;
    }

    MeltedBufferResult *melt_result = rakaly_file_meta_melt(meta);
    unwrapError(rakaly_melt_error(melt_result));
    return std::make_optional(rakaly_melt_value(melt_result));
  }

  MeltedOutput melt() const {
    MeltedBufferResult *melt_result = rakaly_file_melt(file);
    unwrapError(rakaly_melt_error(melt_result));
    return MeltedOutput(rakaly_melt_value(melt_result));
  }

  virtual ~GameFile() { rakaly_free_file(file); }
};

GameFile parseEu4(const std::string &data) {
  PdsFileResult *file_result = rakaly_eu4_file(data.c_str(), data.length());
  unwrapError(rakaly_file_error(file_result));
  PdsFile *file = rakaly_file_value(file_result);
  return GameFile(file);
}

GameFile parseCk3(const std::string &data) {
  PdsFileResult *file_result = rakaly_ck3_file(data.c_str(), data.length());
  unwrapError(rakaly_file_error(file_result));
  PdsFile *file = rakaly_file_value(file_result);
  return GameFile(file);
}

GameFile parseImperator(const std::string &data) {
  PdsFileResult *file_result =
      rakaly_imperator_file(data.c_str(), data.length());
  unwrapError(rakaly_file_error(file_result));
  PdsFile *file = rakaly_file_value(file_result);
  return GameFile(file);
}

GameFile parseHoi4(const std::string &data) {
  PdsFileResult *file_result = rakaly_hoi4_file(data.c_str(), data.length());
  unwrapError(rakaly_file_error(file_result));
  PdsFile *file = rakaly_file_value(file_result);
  return GameFile(file);
}

GameFile parseVic3(const std::string &data) {
  PdsFileResult *file_result = rakaly_vic3_file(data.c_str(), data.length());
  unwrapError(rakaly_file_error(file_result));
  PdsFile *file = rakaly_file_value(file_result);
  return GameFile(file);
}

GameFile parseEu5(const std::string &data) {
  PdsFileResult *file_result = rakaly_eu5_file(data.c_str(), data.length());
  unwrapError(rakaly_file_error(file_result));
  PdsFile *file = rakaly_file_value(file_result);
  return GameFile(file);
}

} // namespace rakaly

#endif
#endif // __cplusplus

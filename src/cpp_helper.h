#ifdef __cplusplus
#ifndef RAKALY_WRAPPER_H
#define RAKALY_WRAPPER_H

#include <stdexcept>
#include <string>

namespace rakaly {

class MeltedOutput {
  MeltedBuffer *melt;

public:
  MeltedOutput(MeltedBuffer *melt) { this->melt = melt; }

  /**
   * Updates the given string with the melted output. The string is assumed to
   * contain the data that was requested to be melted, as if the melter required
   * no work, the string won't be written to (as it is already melted)
   */
  void writeData(std::string &data) const {
    int result = rakaly_melt_error_code(melt);
    if (result) {
      int error_len = rakaly_melt_error_length(melt);
      std::string error(error_len, ' ');
      rakaly_melt_error_write_data(melt, error.data(), error_len);
      auto msg = std::string("librakaly returned an error ") + error;
      throw std::runtime_error(msg);
    }

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

  bool was_binary() const { return rakaly_melt_binary_translated(melt); }
  bool has_unknown_tokens() const {
    return rakaly_melt_binary_unknown_tokens(melt);
  }

  virtual ~MeltedOutput() { rakaly_free_melt(melt); }
};

MeltedOutput meltEu4(const std::string &data) {
  return MeltedOutput(rakaly_eu4_melt(data.c_str(), data.length()));
}

MeltedOutput meltCk3(const std::string &data) {
  return MeltedOutput(rakaly_ck3_melt(data.c_str(), data.length()));
}

MeltedOutput meltImperator(const std::string &data) {
  return MeltedOutput(rakaly_imperator_melt(data.c_str(), data.length()));
}

MeltedOutput meltHoi4(const std::string &data) {
  return MeltedOutput(rakaly_hoi4_melt(data.c_str(), data.length()));
}

MeltedOutput meltVic3(const std::string &data) {
  return MeltedOutput(rakaly_vic3_melt(data.c_str(), data.length()));
}

} // namespace rakaly

#endif
#endif // __cplusplus

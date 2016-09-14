#include "html.h"

std::string strip_tags(std::string html) {
  std::regex tag("(</?[^>]+>)");
  std::string replaced = std::regex_replace(html, tag, "");

  return replaced;
}
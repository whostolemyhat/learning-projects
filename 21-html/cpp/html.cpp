#include "html.h"

std::string strip_tags(std::string html) {
  std::regex tag("(</?[^>]+>)");
  std::regex new_lines("(\\n\\s+)");
  std::string replaced = std::regex_replace(std::regex_replace(html, tag, ""), new_lines, "\n");

  return replaced;
}
require "openssl/md5"
require "openssl/sha1"
require "base64"
require "colorize"
require "./icase/*"

module Icase
  extend self

  enum Mode
    Upcase
    Downcase
    Capitalize
    Sha1
    Md5
    Base64_EN
    Base64_DE

    def self.from_str(m: String?): Icase::Mode
      case m
      when "d", "down", "downcase"
        Icase::Mode::Downcase
      when "c", "capitalize"
        Icase::Mode::Capitalize
      when "sha1", "sha"
        Icase::Mode::Sha1
      when "md5"
        Icase::Mode::Md5
      when "base64", "+base64"
        Icase::Mode::Base64_EN
      when "-base64"
        Icase::Mode::Base64_DE
      when "u", "up", "upcase"
        Icase::Mode::Upcase
      when nil
        Icase::Mode::Upcase
      else
        raise %(unknow mode "#{m}")
      end
    end
  end

  def encode(str: String, mode: Mode): String
    case mode
    when Mode::Upcase
      str.upcase
    when Mode::Downcase
      str.downcase
    when Mode::Capitalize
      str.capitalize
    when Mode::Sha1
      OpenSSL::SHA1.hash(str).map {|x| sprintf("%x", x)}.join("")
    when Mode::Md5
      OpenSSL::MD5.hash(str).map {|x| sprintf("%x", x)}.join("")
    when Mode::Base64_EN
      Base64.urlsafe_encode(str)
    when Mode::Base64_DE
      Base64.decode(str)
    else
      raise "unknown mode: #{mode}"
    end
  end
end

################################################################################
def main()
  str, mode_s = ARGV[0]?, ARGV[1]?
  if !str || %w(--help --h -h).any?{|x|x == str}
    puts "icase str (upcase|downcase|capitalize|sha1|md5|bas64|-base64)".colorize(:red)
  else
    mode = Icase::Mode.from_str(mode_s)
    ret = Icase.encode str.not_nil!, mode
    puts ""
    puts "#{str} |> #{mode.to_s.downcase.colorize(:red)} => #{ret.colorize(:green)}"
    puts ""
  end
rescue ex
  puts ex.message.colorize(:red)
end

main()

require "openssl/md5"
require "openssl/sha1"
require "base64"
require "colorize"
require "json"
require "./icase/*"

module Icase
  extend self

  enum Action
    Upcase
    Downcase
    Capitalize
    Underscore
    Sha1
    Md5
    Base64_EN
    Base64_DE
    PrettyJson
    Camelcase
    Size

    def self.from_str(action: String?): Action
      case action
      when "size", "length", "len"
        Size
      when "d", "down", "downcase"
        Downcase
      when "c", "capitalize"
        Capitalize
      when "camelcase", "cc"
        Camelcase
      when "underscore", "us"
        Underscore
      when "sha1", "sha"
        Sha1
      when "md5"
        Md5
      when "base64", "+base64"
        Base64_EN
      when "-base64"
        Base64_DE
      when "u", "up", "upcase"
        Upcase
      when "json"
        PrettyJson
      when nil
        Upcase
      else
        raise %(unknow action "#{action}")
      end
    end
  end

  def encode(str: String, action: Action): String
    case action
    when Action::Size
      str.size.to_s
    when Action::Upcase
      str.upcase
    when Action::Downcase
      str.downcase
    when Action::Capitalize
      str.capitalize
    when Action::Camelcase
      str.camelcase
    when Action::Underscore
      str.underscore
    when Action::Sha1
      OpenSSL::SHA1.hash(str).map {|x| sprintf("%x", x)}.join("")
    when Action::Md5
      OpenSSL::MD5.hash(str).map {|x| sprintf("%x", x)}.join("")
    when Action::Base64_EN
      Base64.urlsafe_encode(str)
    when Action::Base64_DE
      Base64.decode(str)
    when Action::PrettyJson
      JSON.parse(str).to_pretty_json
    else
      raise %(unknow action "#{action}")
    end
  end
end

################################################################################
private def help()
  puts "USAGE: $ icase str action
-------------------------
       * action - size | upcase | downcase | capitalize | camelcase | underscore | sha1 | md5 | base64 | -base64".colorize(:yellow)
end

def main(argv)
  str = argv[0]?
  if str
    if %w(--help --h -h).any? {|x| x == str}
      help
    else
      action_s = argv[1]?
      action = Icase::Action.from_str(action_s)
      ret = Icase.encode(str, action)
      puts %('#{str}' |> #{action.to_s.downcase.colorize(:red)} =>\n#{ret.colorize(:green)})
      puts ""
    end
  else
    help
  end
rescue ex
  puts ex.message.colorize(:red)
  puts ""
  help
end

main(ARGV)

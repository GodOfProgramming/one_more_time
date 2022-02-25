#!/usr/bin/env ruby

require 'fileutils'

OUT = 'target/debug'.freeze
FILE = 'main.dll'.freeze

VERSION = ARGV.first || 'v0.1.0'

FileUtils.rm("#{VERSION}/#{FILE}") if File.exist?("#{VERSION}/#{FILE}")
FileUtils.cp("#{OUT}/#{FILE}", VERSION)

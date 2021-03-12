require 'ffi'

def add_array(n, x)
  a = Array.new(n, 0)
  x.times do
    (0..x - 1).each do |i|
      a[i] += 1
    end
  end
  a.sum
end

module AddArray
  extend FFI::Library
  ffi_lib :'addarray/target/release/libaddarray.dylib'
  attach_function :add_array, %i[uint64 uint64], :uint64
end

puts AddArray::add_array(ARGV[0].to_i, ARGV[1].to_i)

fish_count = File.read("input.txt").split(',').map(&.to_i).tally
fish = (0..8).map { |i| fish_count.fetch(i, 0).to_i64 }
256.times { fish.rotate!; fish[6] += fish.last }
puts fish.sum

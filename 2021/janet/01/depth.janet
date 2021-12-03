(defn map-indexed [f ds]
  (map f (range 0 (length ds)) ds))

(defn window
  [arr &opt step]
  (default step 1)
  (filter truthy?
    (map-indexed
      (fn [i v]
        (if-let [v2 (get arr (+ i step))]
          [v v2]))
    arr)))

# like sum... but difference
(defn difference
  [arr]
  (reduce2 - (reverse arr)))

(def input (string/trim (slurp "input.txt")))
(def nums (map scan-number (string/split "\n" input)))
(print "Part 1 " (count pos? (map difference (window nums))))
(print "Part 2 " (count pos? (map difference (window nums 3))))

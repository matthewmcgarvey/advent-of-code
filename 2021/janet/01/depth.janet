(defn window
  [arr &opt step]
  (default step 1)
  (->> (seq [[i v] :pairs arr]
         (when-let [v2 (get arr (+ i step))]
           [v v2]))
       (filter truthy?)))

# like sum... but difference
(defn difference
  [arr]
  (reduce2 - (reverse arr)))

(def nums (->> (slurp "input.txt")
               string/trim
               (string/split "\n")
               (map scan-number)))
(print "Part 1 " (->> (window nums)
                      (map difference)
                      (count pos?)))
(print "Part 2 " (->> (window nums 3)
                      (map difference)
                      (count pos?)))

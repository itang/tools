(ns testit
  (:import com.alibaba.fastjson.JSON)
  #_(:gen-class))

(defn -main []
  (-> #{1 2 3 "a" [true false 1.0 \a 100]} JSON/toJSONString println))

#_(-> #{1 2 3 ["a" true false 1.0 \a 100]} JSON/toJSONString println)
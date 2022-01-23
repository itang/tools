(require '[clojure.java.browse :as B])
(require '[babashka.deps :as deps])

(deps/add-deps '{:deps {com.moandjiezana.toml/toml4j {:mvn/version "0.7.2"}
                        com.alibaba/fastjson {:mvn/version "1.2.76"}}})

(import com.alibaba.fastjson.JSON)
(-> [] JSON/toJSONString println)

(import java.util.Date)
(println (Date.))

(import com.moandjiezana.toml.Toml)

(println (.read (Toml.) "a=1"))

(B/browse-url "http://www.baidu.com")
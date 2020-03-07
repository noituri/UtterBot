(ns utter.routing
  (:require
   [kee-frame.core :as k]
   [utter.store.db :as db]
   [utter.pages.homepage :refer [home-page]]
   [utter.pages.panel :refer [panel]]))

(def routes
  [["/"      :home]
   ["/panel" :panel]])

(def debug? false)

(def router
  [k/switch-route (comp :name :data)
   :panel [panel]
   :home  [home-page]
   nil [:div "Loading..."]])

(k/start! {:debug?         debug?
           :routes         routes
           :hash-routing?  debug?
           :initial-db     db/initial-state
           :app-db-spec    ::db-spec})
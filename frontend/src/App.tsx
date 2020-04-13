import React, { Suspense, lazy } from "react";
import { Helmet } from "react-helmet";
import { BrowserRouter, Route } from "react-router-dom";

import withEditorTheme from "./components/hoc/withEditorTheme";
import withTracker from "./components/hoc/withTracker";
import Loading from "./components/pages/Loading";

const About = lazy(() => import(/* webpackChunkName: "about" */ "./components/pages/About"));
const Instances = lazy(() => import(/* webpackChunkName: "instances" */ "./components/pages/Instances"));
const Privacy = lazy(() => import(/* webpackChunkName: "privacy" */ "./components/pages/Privacy"));
const Root = lazy(() => import(/* webpackChunkName: "root" */ "./components/pages/Root"));
const Terms = lazy(() => import(/* webpackChunkName: "terms" */ "./components/pages/Terms"));

const App: React.FC = () => {
  return (
    <>
      <Helmet defaultTitle="Arteria" titleTemplate="%s | Arteria" />
      <BrowserRouter>
        <Suspense fallback={<Loading />}>
          <Route path="/" component={withTracker(withEditorTheme(Root))} exact />
          <Route path="/about" component={withTracker(About)} exact />
          <Route path="/instances/:instanceId" component={withTracker(withEditorTheme(Instances))} exact />
          <Route path="/privacy" component={withTracker(Privacy)} exact />
          <Route path="/terms" component={withTracker(Terms)} exact />
        </Suspense>
      </BrowserRouter>
    </>
  );
};

export default App;

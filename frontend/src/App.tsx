import React from "react";
import { Helmet } from "react-helmet";
import { BrowserRouter, Route } from "react-router-dom";

import withEditorTheme from "./components/hoc/withEditorTheme";
import withTracker from "./components/hoc/withTracker";
import About from "./components/pages/About";
import Instances from "./components/pages/Instances";
import Privacy from "./components/pages/Privacy";
import Root from "./components/pages/Root";
import Terms from "./components/pages/Terms";

const App: React.FC = () => {
  return (
    <>
      <Helmet defaultTitle="Arteria" titleTemplate="%s | Arteria" />
      <BrowserRouter>
        <Route path="/" component={withTracker(withEditorTheme(Root))} exact />
        <Route path="/about" component={withTracker(About)} exact />
        <Route path="/instances/:instanceId" component={withTracker(withEditorTheme(Instances))} exact />
        <Route path="/privacy" component={withTracker(Privacy)} exact />
        <Route path="/terms" component={withTracker(Terms)} exact />
      </BrowserRouter>
    </>
  );
};

export default App;

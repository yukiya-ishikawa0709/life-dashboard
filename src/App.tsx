import { Route, Routes } from "react-router-dom";
import Home from "./pages/Home";
import FeatureDetail from "./pages/FeatureDetail";
import "./App.css";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/features/:featureId" element={<FeatureDetail />} />
      {/* TODO: 画面が増えてきたら catch-all(path="*") の404的なフォールバック画面を検討する */}
    </Routes>
  );
}

export default App;

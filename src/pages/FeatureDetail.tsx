import { Link, useParams } from "react-router-dom";
import { FEATURES } from "../features";

function FeatureDetail() {
  const { featureId } = useParams();
  const feature = FEATURES.find((f) => f.id === featureId);

  return (
    <main className="container">
      <h1>{feature?.label ?? "不明な機能"}</h1>
      <p>準備中です。</p>
      <Link to="/">ホームに戻る</Link>
    </main>
  );
}

export default FeatureDetail;

import { Link, useParams } from "react-router-dom";
import { FEATURES } from "../features";

function FeatureDetail() {
  const { featureId } = useParams();
  // Home側(mainFeature)はデフォルトにフォールバックするが、ここでは不正な遷移だと
  // 気づけるようあえて「不明な機能」を出し分ける。TODO: 画面が増えたらこの方針を統一するか検討
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

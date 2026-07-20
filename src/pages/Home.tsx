import { useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import { FEATURES, type FeatureId } from "../features";
import "./Home.css";

function addDays(date: Date, amount: number): Date {
  const next = new Date(date);
  next.setDate(next.getDate() + amount);
  return next;
}

function formatDate(date: Date): string {
  return new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "long",
    day: "numeric",
    weekday: "short",
  }).format(date);
}

function Home() {
  const navigate = useNavigate();
  // ホームの状態(メイン機能・閲覧日)はDB永続化せず、React stateのみで保持する(requirements.md 6.0 TBD)
  const [mainFeatureId, setMainFeatureId] = useState<FeatureId>(FEATURES[0].id);
  const [viewingDate, setViewingDate] = useState(() => new Date());

  // mainFeatureIdは常にFEATURES由来の値しか取らないため `?? FEATURES[0]` は実質到達しない防御的分岐。
  // TODO: stateをidではなくFeatureオブジェクト自体にすれば、この分岐ごと型で不要にできる
  const mainFeature = useMemo(
    () => FEATURES.find((feature) => feature.id === mainFeatureId) ?? FEATURES[0],
    [mainFeatureId],
  );
  const subFeatures = useMemo(
    () => FEATURES.filter((feature) => feature.id !== mainFeatureId),
    [mainFeatureId],
  );

  return (
    <main className="home">
      <header className="home-date-nav">
        <button type="button" onClick={() => setViewingDate((date) => addDays(date, -1))}>
          前日
        </button>
        <span className="home-date">{formatDate(viewingDate)}</span>
        <button type="button" onClick={() => setViewingDate((date) => addDays(date, 1))}>
          翌日
        </button>
      </header>

      <section className="ticket ticket-main">
        <h1>{mainFeature.label}</h1>
        <p>実績: 記録なし</p>
        <button type="button" onClick={() => navigate(`/features/${mainFeature.id}`)}>
          詳細を見る
        </button>
      </section>

      <section className="ticket-sub-list">
        {subFeatures.map((feature) => (
          <button
            type="button"
            key={feature.id}
            className="ticket ticket-sub"
            onClick={() => setMainFeatureId(feature.id)}
          >
            <h2>{feature.label}</h2>
            <p>実績: 記録なし</p>
          </button>
        ))}
      </section>
    </main>
  );
}

export default Home;

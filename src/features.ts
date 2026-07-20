export type FeatureId = "schedule" | "expense" | "training" | "study";

export interface Feature {
  id: FeatureId;
  label: string;
}

export const FEATURES: Feature[] = [
  { id: "schedule", label: "スケジュール" },
  { id: "expense", label: "家計簿" },
  { id: "training", label: "筋トレ記録" },
  { id: "study", label: "勉強記録" },
];

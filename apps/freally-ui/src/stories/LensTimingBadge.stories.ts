import type { Meta, StoryObj } from "@storybook/svelte";
import LensTimingBadge from "../components/results/LensTimingBadge.svelte";

const meta = {
  title: "Results/LensTimingBadge",
  component: LensTimingBadge,
  argTypes: {
    lens: { control: "select", options: ["filename", "content", "audio", "similarity"] },
    ms: { control: "number" }
  }
} satisfies Meta<typeof LensTimingBadge>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Filename: Story = { args: { lens: "filename", ms: 8 } };
export const Content: Story = { args: { lens: "content", ms: 22 } };
export const Audio: Story = { args: { lens: "audio", ms: 5 } };
export const Similarity: Story = { args: { lens: "similarity", ms: 11 } };

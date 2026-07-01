import type { Meta, StoryObj } from "@storybook/svelte";
import QuickFiltersPalette from "../components/filters/QuickFiltersPalette.svelte";

const meta = {
  title: "Filters/QuickFiltersPalette",
  component: QuickFiltersPalette
} satisfies Meta<typeof QuickFiltersPalette>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

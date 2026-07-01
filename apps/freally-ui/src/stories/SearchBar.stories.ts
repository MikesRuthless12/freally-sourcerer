import type { Meta, StoryObj } from "@storybook/svelte";
import SearchBar from "../components/search-bar/SearchBar.svelte";

const meta = {
  title: "Search/SearchBar",
  component: SearchBar
} satisfies Meta<typeof SearchBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

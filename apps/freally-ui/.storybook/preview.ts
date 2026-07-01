import type { Preview } from "@storybook/svelte";
import "../src/app.css";

const preview: Preview = {
  parameters: {
    backgrounds: {
      default: "dark",
      values: [
        { name: "dark", value: "#0E1116" },
        { name: "light", value: "#FAFBFC" }
      ]
    },
    controls: { matchers: { color: /(background|color)$/i, date: /Date$/ } }
  },
  globalTypes: {
    theme: {
      description: "Theme",
      defaultValue: "dark",
      toolbar: {
        title: "Theme",
        items: [
          { value: "system", title: "System" },
          { value: "light", title: "Light" },
          { value: "dark", title: "Dark" }
        ],
        dynamicTitle: true
      }
    }
  },
  decorators: [
    (story, ctx) => {
      const t = (ctx.globals as { theme?: string }).theme;
      if (typeof document !== "undefined") {
        if (t === "system") document.documentElement.removeAttribute("data-theme");
        else if (t) document.documentElement.setAttribute("data-theme", t);
      }
      return story();
    }
  ]
};

export default preview;

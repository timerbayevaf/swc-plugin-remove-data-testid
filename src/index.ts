import type { RsbuildPlugin } from "@rsbuild/core";
import type { JSXAttribute } from "@swc/core";

function removeDataTestIdVisitor() {
  return {
    visitJSXAttribute(n: JSXAttribute): JSXAttribute | null {
      if (n.name.type === "Identifier" && n.name.value === "data-testid") {
        return null;
      }
      return n;
    },
  };
}

export const pluginRemoveDataTestId = (): RsbuildPlugin => ({
  name: "plugin-remove-data-testid",

  setup(api) {
    api.modifyBundlerChain((chain) => {
      // Берём правило TS
      const rule = chain.module.rule("ts");

      // Создаём use с любым именем (имя не важно)
      const use = rule.use("ts-swc");

      // Модифицируем jsc через tap
      use.tap((swcOptions: any = {}) => {
        swcOptions.jsc ??= {};
        swcOptions.jsc.experimental ??= {};
        swcOptions.jsc.experimental.plugins ??= [];

        swcOptions.jsc.experimental.plugins.push([
          function removeDataTestId() {
            return { visitor: removeDataTestIdVisitor() };
          },
          {},
        ]);

        return swcOptions;
      });
    });
  },
});

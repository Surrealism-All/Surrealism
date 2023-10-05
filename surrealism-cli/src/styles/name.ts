/**
 * ============================================
 * @author:syf20020816@outlook.com
 * @since:20230307
 * @version:0.0.1
 * @type:ts
 * @description:用于对Class和ID进行构建
 *
 * ============================================
 */

/**
 * 构建内部组件的名称
 */
export const build = (componentName: string, other: string): string => {
  return componentName + "-" + other;
};
/**
 * 构建外层wrap
 * @param componentName
 * @param wrap
 * @returns
 */
export const buildWrap = (componentName: string, wrap: string): string => {
  return componentName + "-" + wrap + "_wrap";
};
/**
 * 构建主container
 * @param componentName
 * @returns
 */
export const buildView = (componentName: string): string => {
  return componentName + "_view";
};

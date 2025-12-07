import { makePersisted } from "@solid-primitives/storage"
import {
  createContext,
  createEffect,
  createSignal,
  onMount,
  type ParentComponent,
  useContext,
} from "solid-js"

type Theme = "light" | "dark" | "system"

interface ThemeContextValue {
  theme: () => Theme
  setTheme: (theme: Theme) => void
  resolvedTheme: () => "light" | "dark"
}

const ThemeContext = createContext<ThemeContextValue>()

export const ThemeProvider: ParentComponent<{ defaultTheme?: Theme }> = (props) => {
  const [theme, setThemeState] = makePersisted(
    createSignal<Theme>(props.defaultTheme ?? "system"),
    { name: "theme" },
  )
  const [resolvedTheme, setResolvedTheme] = createSignal<"light" | "dark">("dark")

  const getSystemTheme = () =>
    typeof window !== "undefined" && window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light"

  const applyTheme = (t: Theme) => {
    if (typeof document === "undefined") return
    const resolved = t === "system" ? getSystemTheme() : t
    document.documentElement.classList.remove("light", "dark")
    document.documentElement.classList.add(resolved)
    setResolvedTheme(resolved)
  }

  const setTheme = (t: Theme) => {
    setThemeState(t)
    applyTheme(t)
  }

  onMount(() => {
    applyTheme(theme())
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
    const handleChange = () => theme() === "system" && applyTheme("system")
    mediaQuery.addEventListener("change", handleChange)
  })

  createEffect(() => applyTheme(theme()))

  return (
    <ThemeContext.Provider value={{ theme, setTheme, resolvedTheme }}>
      {props.children}
    </ThemeContext.Provider>
  )
}

export function useTheme() {
  const context = useContext(ThemeContext)
  if (!context) throw new Error("useTheme must be used within a ThemeProvider")
  return context
}

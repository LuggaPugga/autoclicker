import { Moon, Sun } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useTheme } from "@/lib/theme-provider"
import { cn } from "@/lib/utils"

export function ThemeToggle({ className }: { className?: string }): React.ReactElement {
  const { theme, setTheme } = useTheme()

  function switchTheme() {
    if (theme == "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      setTheme(systemTheme === "dark" ? "light" : "dark")
    } else {
      setTheme(theme === "dark" ? "light" : "dark")
    }
  }

  return (
    <div className={cn("flex items-center", className)}>
      <Button
        variant="ghost"
        size="icon"
        onClick={() => switchTheme()}
        className="h-8 w-8 rounded-full"
        aria-label="Toggle theme"
      >
        <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:rotate-90 dark:scale-0" />
        <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span className="sr-only">Toggle theme</span>
      </Button>
    </div>
  )
}

import { useTranslations } from "next-intl";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  Badge,
} from "@pos/shared/ui";

export default function HomePage() {
  const t = useTranslations("app");

  return (
    <main className="flex min-h-screen items-center justify-center p-8">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{t("title")}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-muted-foreground">{t("description")}</p>
          <Badge variant="outline">Placeholder</Badge>
        </CardContent>
      </Card>
    </main>
  );
}

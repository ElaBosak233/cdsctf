import { easeInOut, motion, useReducedMotion } from "framer-motion";
import { InfoIcon } from "lucide-react";
import { Link } from "react-router";

import { Button } from "@/components/ui/button";
import { Image } from "@/components/ui/image";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { DefaultLogo } from "@/components/widgets/default-logo";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function Index() {
  const { config } = useConfigStore();

  const shouldReduceMotion = useReducedMotion();
  const floatAnimate = shouldReduceMotion ? { y: 0 } : { y: [0, -8, 0] };
  const floatTransition = shouldReduceMotion
    ? { duration: 0 }
    : { duration: 6, repeat: Infinity, ease: easeInOut };
  const containerVariants = {
    hidden: { opacity: 0, y: 18 },
    show: {
      opacity: 1,
      y: 0,
      transition: { staggerChildren: 0.08 },
    },
  };
  const itemVariants = {
    hidden: { opacity: 0, y: 18 },
    show: { opacity: 1, y: 0 },
  };

  return (
    <>
      <title>{config?.meta?.title}</title>
      <div
        className={cn([
          "flex-1",
          "flex",
          "flex-col",
          "items-center",
          "justify-between",
          "select-none",
          "my-5",
        ])}
      >
        <div />
        <motion.div
          className={cn([
            "flex",
            "flex-col",
            "items-center",
            "flex-1",
            "justify-center",
          ])}
          animate={floatAnimate}
          transition={floatTransition}
        >
          <motion.div
            className={cn([
              "flex",
              "flex-col",
              "items-center",
              "flex-1",
              "justify-center",
            ])}
            variants={containerVariants}
            initial="hidden"
            animate="show"
          >
            <motion.div variants={itemVariants}>
              <Image
                src={"/api/configs/logo"}
                fallback={<DefaultLogo />}
                className={cn(["aspect-square", "h-32"])}
                alt={"logo"}
                delay={0}
                glass={false}
              />
            </motion.div>

            <motion.h1
              className={cn([
                "text-3xl",
                "lg:text-4xl",
                "font-extrabold",
                "mt-5",
              ])}
              variants={itemVariants}
            >
              {config?.meta?.title}
            </motion.h1>

            <motion.span
              variants={itemVariants}
              className={cn(["text-secondary-foreground", "mt-2"])}
            >
              {config?.meta?.description}
              <span className={cn(["animate-ping"])}>_</span>
            </motion.span>
          </motion.div>
        </motion.div>
        <div className={cn(["hidden", "sm:flex", "items-center", "gap-3"])}>
          <Button>
            <Typography className={cn(["text-secondary-foreground"])}>
              <MarkdownRender src={config?.meta?.footer} />
            </Typography>
          </Button>
          <Separator orientation={"vertical"} className={cn(["h-5"])} />
          <Button
            square
            asChild
            icon={<InfoIcon />}
            className={cn(["text-secondary-foreground"])}
          >
            <Link to={"/about"} />
          </Button>
        </div>
      </div>
    </>
  );
}

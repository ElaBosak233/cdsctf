import { AnimatePresence, motion } from "framer-motion";
import { LoaderCircleIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { Image } from "@/components/ui/image";
import { DefaultLogo } from "@/components/widgets/default-logo";
import type { GameMini } from "@/models/game";
import { cn } from "@/utils";

interface EntranceProps {
  game?: GameMini;
  onFinish?: () => void;
}

export default function Entrance({ game, onFinish }: EntranceProps) {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const [expanded, setExpanded] = useState(false);

  // Drives entrance animation timing and navigation.
  useEffect(() => {
    if (game?.id) {
      setExpanded(true);

      const timer1 = setTimeout(() => {
        navigate(`/games/${game.id}`);
      }, 1000);

      const timer2 = setTimeout(() => {
        setExpanded(false);
        onFinish?.();
      }, 3000);

      return () => {
        clearTimeout(timer1);
        clearTimeout(timer2);
      };
    }
  }, [game, navigate, onFinish]);

  return (
    <AnimatePresence>
      {expanded && game && (
        <motion.div
          key={game.id}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.25, ease: "easeInOut" }}
          className={cn(
            "fixed top-0 left-0 w-full h-full z-50 overflow-hidden pointer-events-none"
          )}
        >
          {/* Background layer */}
          <motion.div
            className="absolute inset-0"
            initial={{ opacity: 0, scale: 1 }}
            animate={{ opacity: 1, scale: 1.15, filter: "blur(6px)" }}
            exit={{ opacity: 0, scale: 1.05 }}
            transition={{ duration: 0.25 }}
          >
            <div className="absolute inset-0 bg-background" />
          </motion.div>

          {/* Foreground content */}
          <motion.div
            className="absolute inset-0 flex flex-col items-center justify-center text-center text-foreground space-y-6"
            initial="hidden"
            animate="visible"
            exit="hidden"
            variants={{
              hidden: { opacity: 0 },
              visible: {
                opacity: 1,
                transition: {
                  staggerChildren: 0.15, // quick staggered children
                  delayChildren: 0.2,
                },
              },
            }}
          >
            {/* LOGO */}
            <motion.div
              variants={{
                hidden: { opacity: 0, scale: 1.2 },
                visible: {
                  opacity: 1,
                  scale: 1,
                  transition: { duration: 0.4, ease: "easeOut" },
                },
              }}
              className="aspect-square h-40"
            >
              <Image
                src={
                  game?.has_icon
                    ? `/api/games/${game.id}/icon`
                    : `/api/configs/logo`
                }
                fallback={<DefaultLogo />}
                delay={0}
                className={cn(["w-full", "h-full", "object-contain"])}
              />
            </motion.div>

            {/* Title */}
            <motion.h1
              variants={{
                hidden: { opacity: 0, y: 20 },
                visible: {
                  opacity: 1,
                  y: 0,
                  transition: { duration: 0.4, ease: "easeOut" },
                },
              }}
              className="text-3xl font-bold"
            >
              {game.title}
            </motion.h1>

            {/* Sketch / summary */}
            {game.sketch && (
              <motion.p
                variants={{
                  hidden: { opacity: 0, y: 10 },
                  visible: {
                    opacity: 1,
                    y: 0,
                    transition: { duration: 0.4, ease: "easeOut" },
                  },
                }}
                className="text-sm text-secondary-foreground max-w-md"
              >
                {game.sketch}
              </motion.p>
            )}

            {/* Loading hint */}
            <motion.div
              variants={{
                hidden: { opacity: 0 },
                visible: {
                  opacity: 1,
                  transition: { duration: 0.3, ease: "easeOut" },
                },
              }}
              className="absolute left-6 bottom-4 flex items-center text-secondary-foreground space-x-2"
            >
              <LoaderCircleIcon className="animate-spin w-4 h-4" />
              <span>{t("common:loading")}...</span>
            </motion.div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

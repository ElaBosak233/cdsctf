import { AnimatePresence, motion } from "framer-motion";
import { LoaderCircleIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import type { GameMini } from "@/models/game";
import { cn } from "@/utils";

interface EntranceProps {
  game?: GameMini;
  onFinish?: () => void;
}

export default function Entrance({ game, onFinish }: EntranceProps) {
  const navigate = useNavigate();
  const [expanded, setExpanded] = useState(false);

  // 控制动画生命周期
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
          {/* 背景层 */}
          <motion.div
            className="absolute inset-0"
            initial={{ opacity: 0, scale: 1 }}
            animate={{ opacity: 1, scale: 1.15, filter: "blur(6px)" }}
            exit={{ opacity: 0, scale: 1.05 }}
            transition={{ duration: 0.25 }}
          >
            <div className="absolute inset-0 bg-background" />
          </motion.div>

          {/* 内容层 */}
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
                  staggerChildren: 0.15, // 快速顺序动画
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
              <img
                src={`/api/games/${game.id}/icon`}
                alt={game.title}
                className="w-full h-full object-contain"
              />
            </motion.div>

            {/* 标题 */}
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

            {/* 简介 */}
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

            {/* 加载提示 */}
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
              <span>加载中...</span>
            </motion.div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

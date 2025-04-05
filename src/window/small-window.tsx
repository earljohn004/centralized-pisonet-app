import { useSettingStore } from "../store/Settings";

export const SmallWindow = () => {
  const coin = useSettingStore((state) => state.coin);
  const remainingTime = useSettingStore((state) => state.remainingTime);
  return (
    <>
      <h2>Inserted PHP {coin}</h2>
      <h3>Remaining Time: {remainingTime} seconds</h3>
    </>
  );
};

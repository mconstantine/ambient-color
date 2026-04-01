/**
 * Turns a timestamp in seconds from midnight into a time string.
 * E.g: 43230 into "12:00:30"
 */
export const timeFromTimestamp = (timestamp: number): string => {
  const date = new Date(timestamp * 1000);

  const hours = date.getUTCHours();
  const minutes = date.getUTCMinutes();
  const seconds = date.getUTCSeconds();

  const reference = new Date(Date.UTC(2020, 0, 1, hours, minutes, seconds));

  return reference.toLocaleTimeString("it-IT", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
};

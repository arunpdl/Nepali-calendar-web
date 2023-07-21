import { relativeTimeFromDates } from "../helper/dates";
import nepaliNumber from "../helper/nepaliNumber";
import { Event } from "../types/calendar.types";
import UseLanguage from "../helper/useLanguage";
import NepaliDate from "nepali-date-converter";
function SingleCalendarEvent({ date, events }: { date: NepaliDate; week_day: number; events: Event[] }) {
  const { isNepaliLanguage } = UseLanguage();
  const nepaliEvents = isNepaliLanguage
    ? events.map((event) => event?.jds?.ne)
    : events.map((event) => event?.jds?.en);

  let eventsString = nepaliEvents.join(" / ");

  // Add today's date to the array even if there are no events for today
  const today = new Date();
  const isToday = today.toDateString() === date.toJsDate().toDateString();
  if (isToday && nepaliEvents.length === 1) {
    // If today and there are no events, show "No Event" instead of an empty string
    if (isNepaliLanguage) {
      eventsString = "कुनै घटना छैन";
    } else {
      eventsString = "No Event";
    }
  }
  return (
    <div className="relative">
      {eventsString.length > 0 && (
        <div className="flex max-w-[600px] border border-gray-400 py-3 pl-1 font-mukta dark:text-gray-100">
          <div className="min-w-[80px]  border-r pr-2">
            <h1 className="text-center font-semibold">
              {isNepaliLanguage ? nepaliNumber(`${date.getBS().date}`) : date.getBS().date}
            </h1>
            <h2 className="text-center">
              {date.toJsDate().toLocaleDateString(isNepaliLanguage ? "ne-NP" : "en-US", { weekday: "long" })}
            </h2>
          </div>
          <div className=" events flex justify-between pl-3">
            <div>
              <h3 className="text-start text-sm">
                {date.toJsDate().toLocaleDateString(isNepaliLanguage ? "ne-NP" : "en-US", {
                  month: "long",
                  day: "numeric",
                  year: "numeric",
                })}
              </h3>
              <h1 className="py-1 text-start tracking-wider">{eventsString}</h1>
            </div>
            <h1 className="absolute right-2 text-sm">
              {relativeTimeFromDates(date.toJsDate(), isNepaliLanguage)}
            </h1>
          </div>
        </div>
      )}
    </div>
  );
}

export default SingleCalendarEvent;

import { mockEvents } from "../lib/mock-data";

export default function HomePage() {
  return (
    <main className="max-w-7xl mx-auto px-8 py-8">
      <section className="mb-8">
        <h2 className="text-2xl font-bold mb-4">イベントを探す</h2>
        <div className="flex gap-4 mb-8">
          <input
            type="text"
            placeholder="キーワードで検索"
            className="flex-1 px-4 py-3 border border-gray-300 rounded-lg text-base focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button className="px-8 py-3 bg-blue-600 text-white rounded-lg text-base hover:bg-blue-700 transition-colors">
            検索
          </button>
        </div>
      </section>

      <section>
        <h2 className="text-2xl font-bold mb-6">開催予定のイベント</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {mockEvents.map((event) => (
            <article
              key={event.id}
              className="border border-gray-200 rounded-lg p-6 bg-white shadow-sm hover:shadow-md transition-shadow cursor-pointer"
            >
              <h3 className="text-lg font-bold mb-3 text-gray-900">
                {event.title}
              </h3>
              <p className="text-gray-600 text-sm mb-4 line-clamp-3">
                {event.description}
              </p>
              <div className="flex flex-col gap-2 mb-4 text-sm text-gray-700">
                <div className="flex items-center gap-2">
                  <span>📅</span>
                  <span>
                    {event.date} {event.time}
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  <span>📍</span>
                  <span>{event.location}</span>
                </div>
                <div className="flex items-center gap-2">
                  <span>👥</span>
                  <span>
                    {event.participants} / {event.capacity}
                  </span>
                </div>
              </div>
              <div className="flex flex-wrap gap-2 mb-3">
                {event.tags.map((tag) => (
                  <span
                    key={tag}
                    className="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-xs"
                  >
                    {tag}
                  </span>
                ))}
              </div>
              <div className="text-sm text-gray-600">
                主催: {event.organizer}
              </div>
            </article>
          ))}
        </div>
      </section>
    </main>
  );
}

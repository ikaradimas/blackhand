import { commands, type CategoryInfo } from "$lib/bindings";
import { unwrap } from "$lib/api";

/** Sentinel for the "no category" filter (different from `null` which is "All"). */
export const UNCATEGORIZED = "__uncategorized__";
/** Sentinel for showing all torrents. */
export type Filter = string | typeof UNCATEGORIZED | null;

class CategoriesStore {
  list = $state<CategoryInfo[]>([]);
  /** Currently selected filter — null means "All". */
  filter = $state<Filter>(null);

  async refresh() {
    try {
      this.list = await unwrap(commands.listCategories());
    } catch {
      // tolerable — empty list
    }
  }

  select(filter: Filter) {
    this.filter = filter;
  }

  async assign(infoHash: string, category: string | null) {
    await unwrap(commands.setTorrentCategory(infoHash, category));
    await this.refresh();
  }
}

export const categories = new CategoriesStore();

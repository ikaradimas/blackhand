import { commands, type CategoryInfo } from "$lib/bindings";
import { unwrap } from "$lib/api";

/** Sentinel for the "no category" filter (different from `null` which is "All"). */
export const UNCATEGORIZED = "__uncategorized__";
/** Built-in system filters — not user-editable, not stored on disk. */
export const FINISHED = "__finished__";
export const PENDING = "__pending__";
export type Filter =
  | string
  | typeof UNCATEGORIZED
  | typeof FINISHED
  | typeof PENDING
  | null;

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

export class Wallpaper {
  description: string
  properties: {}[]
  title: string
  preview: string

  constructor(project_json: any) {
    this.description = project_json.description;
    this.title = project_json.title;
    this.properties = [];
    this.preview = project_json.preview;

    if (project_json.general !== undefined && project_json.general.properties !== undefined) {
      for (const key in project_json.general.properties) {
        this.properties.push(project_json.general.properties[key]);
      }
    }
  }
}
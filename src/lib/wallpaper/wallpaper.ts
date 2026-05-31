export class Wallpaper {
  public description: string
  public properties: {}[]
  public title: string
  public preview: string
  public id: number 

  constructor(project_json: any, id: string, preview: string) {
    this.description = project_json.description;
    this.title = project_json.title;
    this.properties = [];
    this.id = parseInt(id)
    this.preview = preview

    if (project_json.general !== undefined && project_json.general.properties !== undefined) {
      for (const key in project_json.general.properties) {
        this.properties.push(project_json.general.properties[key]);
      }
    }
  }
}

export class Property {
	index: number
	order: number

	constructor(index: number, order: number) {
		this.index = index
		this.order = order
	}
}

export class Boolean extends Property {
	value: boolean
	
	constructor(json: any) {
		super(json.index, json.order)
		this.value = json.value
	}
}

import { ComponentFixture, TestBed } from '@angular/core/testing';

import { App.MainComponent } from './app-main.component';

describe('App.MainComponent', () => {
  let component: App.MainComponent;
  let fixture: ComponentFixture<App.MainComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ App.MainComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(App.MainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

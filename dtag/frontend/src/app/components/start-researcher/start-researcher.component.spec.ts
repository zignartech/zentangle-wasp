import { ComponentFixture, TestBed } from '@angular/core/testing';

import { StartResearcherComponent } from './start-researcher.component';

describe('StartReseacherComponent', () => {
  let component: StartResearcherComponent;
  let fixture: ComponentFixture<StartResearcherComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [StartResearcherComponent],
    }).compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(StartResearcherComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
